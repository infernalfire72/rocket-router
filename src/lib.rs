pub trait RouterExt {
    fn mount_router<F: FnOnce() -> Router>(self, router: F) -> Self;
}

impl RouterExt for rocket::Rocket<rocket::Build> {
    fn mount_router<F: FnOnce() -> Router>(self, router: F) -> Self {
        let router = router();
        let routes = router.routes();
        routes
            .into_iter()
            .fold(self, |this, (prefix, routes)| this.mount(prefix, routes))
    }
}

pub struct Router {
    pub prefix: &'static str,
    tags: Vec<&'static str>,
    subrouters: Vec<Router>,
    routes: Vec<rocket::Route>,
}

fn concat_routes(mut prefix: String, route: String) -> String {
    if prefix.ends_with("/") {
        if route.starts_with("/") {
            prefix.push_str(&route[1..]);
        } else {
            prefix.push_str(&route);
        }
    } else {
        if !route.starts_with("/") {
            prefix.push('/');
        }
        prefix.push_str(&route);
    }

    prefix
}

impl Router {
    pub fn new(
        prefix: &'static str,
        tags: Vec<&'static str>,
        subrouters: Vec<Router>,
        routes: Vec<rocket::Route>,
    ) -> Router {
        Router {
            prefix,
            tags,
            subrouters,
            routes,
        }
    }

    pub fn routes(&self) -> Vec<(String, Vec<rocket::Route>)> {
        let router_prefix = self.prefix.to_string();
        let mut routes = vec![];
        if !self.routes.is_empty() {
            routes.push((router_prefix.clone(), self.routes.clone()));
        }
        let subroutes = self
            .subrouters
            .iter()
            .map(|router| {
                router
                    .routes()
                    .into_iter()
                    .map(|(subrouter_prefix, routes)| {
                        let full_route = concat_routes(router_prefix.clone(), subrouter_prefix);
                        (full_route, routes)
                    })
            })
            .flatten();
        routes.extend(subroutes);
        routes
    }
}

#[macro_export]
macro_rules! router {
    ($e:expr, $($p:path),+) => {
        router!($e, routes=[$($p),+]);
    };

    ($e:expr
        $(,tags=[$($t:expr),* $(,)?])?
        $(,include_routers=[$($p:path),* $(,)?])?
        $(,routes=[$($r:path),* $(,)?])?
    ) => {
        || {
            use $crate::Router;
            use ::rocket::routes;
            Router::new($e, vec![$($($t),*)?], vec![$($($p()),*)?], routes![$($($r),*)?])
        }
    };
}

pub type RouterFactory = fn() -> Router;
