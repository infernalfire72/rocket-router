A simple fastapi inspired APIRouter implementation for rocket-rs

```rs
use rocket_router::{RouterFactory, router};

#[rocket::get("/")]
async fn get_something() {
  ...
}

const SUBROUTER: RouterFactory = router!("/v1", routes=[get_something]);
const API: RouterFactory = router!("/api", include_routers=[SUBROUTER]);

#[rocket::main]
async fn main() {
  rocket::build()
    .mount_router(API)
    ...
}

```
