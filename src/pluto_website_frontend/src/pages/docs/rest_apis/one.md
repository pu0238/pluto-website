---
layout: ../../../layouts/DocsLayout.astro
title: "Setting Up the Project"
pubDate: 2024-01-28
author: "Reyth"
---

At this time, Pluto projects require some boilerplate to set up. This will be reduced significantly in future releases, but for now we need to be brave.

## Setting up the project

First, we need to start a new project using `dfx`:

```sh
dfx new pluto_rest_api --no-frontend --type rust
```

Once the command is done downloading the dependencies, we can open our code editor in that new catalog. As we specified in the command, we're not going to be getting any frontend-focused code. We don't need that for our API.

Now that we have a project, it's time to add the required dependencies. In the project root, navigate to `src/pluto_rest_api_backend` and open the `Cargo.toml` file. There we will add the required dependencies:

```toml
[dependencies]
ic-cdk = "0.11.3"
candid = "0.9.6"
pluto = { git = "https://github.com/pu0238/pluto" }
serde_json = "1.0.108"
```

After that we are ready to start writing our API.

## Server setup

The server needs a bit of setup as of right now. This boilerplate will eventually be hidden away, but for now it needs to be added manually. For that we need to create a new file called `bootstrap.rs` in the `src/` directory, right alongside the `lib.rs` file. In that file we need to initialize our server:

```rs
use ic_cdk::{post_upgrade, query, update};
use pluto::{
    http::{HttpServe, RawHttpRequest, RawHttpResponse},
    http_serve,
    router::Router,
};
use std::cell::RefCell;

use crate::controller;

thread_local! {
    static ROUTER: RefCell<Router>  = RefCell::new(controller::setup());
}

// System functions
#[post_upgrade]
fn post_upgrade() {
    ROUTER.with(|r| *r.borrow_mut() = controller::setup())
}

// Http interface
#[query]
async fn http_request(req: RawHttpRequest) -> RawHttpResponse {
    bootstrap(http_serve!(), req).await
}

#[update]
async fn http_request_update(req: RawHttpRequest) -> RawHttpResponse {
    bootstrap(http_serve!(), req).await
}

async fn bootstrap(mut app: HttpServe, req: RawHttpRequest) -> RawHttpResponse {
    let router = ROUTER.with(|r| r.borrow().clone());
    app.set_router(router);
    app.serve(req).await
}
```

This set of methods and the `ROUTER` static value define the behavior of our server. It is necessary for the internal HTTP router to intercept requests from the client and process them internally by our controllers.

Now what are controllers? A controller in pluto is aiming to represent the `C` in the MVC pattern - the business logic of our code. We can create a new file called `controller.rs` to start programming logic in our API:

```rs
use std::collections::HashMap;

use ic_cdk::println;
use pluto::{
    http::{HttpRequest, HttpResponse, HttpServe},
    router::Router,
};
use serde_json::json;

pub(crate) fn setup() -> Router {
    let mut router = Router::new();

    router.put("/:value", false, |req: HttpRequest| async move {
        println!("Hello World from PUT {:?}", req.params.get("value"));

        Ok(HttpResponse {
            status_code: 200,
            headers: HashMap::new(),
            body: json!({
                "statusCode": 200,
                "message": "Hello World from PUT",
                "paramValue": req.params.get("value")
            })
            .into(),
        })
    });
    router.post("/", false, |req: HttpRequest| async move {
        let received_body: Result<String, HttpResponse> = String::from_utf8(req.body)
            .map_err(|_| HttpServe::internal_server_error().unwrap_err());
        Ok(HttpResponse {
            status_code: 200,
            headers: HashMap::new(),
            body: json!({
                "statusCode": 200,
                "message": "Hello World from POST",
                "receivedBody": received_body?
            })
            .into(),
        })
    });
    router.get("/", false, |_req: HttpRequest| async move {
        Ok(HttpResponse {
            status_code: 200,
            headers: HashMap::new(),
            body: json!({
                "statusCode": 200,
                "message": "Hello World from GET",
            })
            .into(),
        })
    });

    router
}
```

As you can see, here we have defined three very simple endpoints. The beauty of an IC canister is that you can make them as complex, or as simple, as you'd like. Anything that will compile to WASM is allowed here.

Now, to bring it all together, all we need to do is add both modules to the `lib.rs` file like so:

```rs
mod bootstrap;
mod controller;
```

And that's it! Now you can build your canister with `dfx build` and deploy it to your local network.
