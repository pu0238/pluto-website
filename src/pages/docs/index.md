---
layout: ../../layouts/DocsLayout.astro
pubDate: 2024-01-25
author: "Pu0238"
---

# Introduction

Pluto (Plutonium) is a [Rust](https://www.rust-lang.org/) framework for building efficient blockchain-side applications. This framework allows developers to create scalable REST-API servers as well as blockchain-side rendered websites.

Pluto provides a level of abstraction above [Internet Computer](https://internetcomputer.org/) [HTTP gateway protocol](https://internetcomputer.org/how-it-works/smart-contracts-serve-the-web/#http-gateway-protocol). This allow developers to focus on building them own solutions without need of maintaining HTTP service infrastructure.

## Idea

While developing a NFT solution we find out that because of architecture of most of existing blockchain fetching data is only possible via them interface that is not flexible at all. Because of that we was looking for a way to store NFT metadata in decentralized way with possibility to edit them. Our first idea was to use [arweave](https://www.arweave.org/) or [IPFS](https://ipfs.tech/) but using sollution like one of this would make our app centralized because of payment gateway because to perform payment for storage wallet was needed. After much more research we find out that only Internet Computer may solve our problems.

At the time of building Pluto HTTP on Internet Computer was very raw thats why we decided to create a framework that will support HTTP out of the box with architecture that if familiar for most of web 2 developers. Developer experience was our first goal thats why we was trying to achieve developer experience like in these frameworks: [NestJS](https://nestjs.com/), [ExpressJS](https://expressjs.com/), [Rocker](https://rocket.rs/)

## Installation

To get started, you can create a new project using the [dfx CLI](https://internetcomputer.org/docs/current/references/cli-reference/) and add a few configuration lines to the code of the canister, or clone our repo and play around with the examples.

To create a new project using the dfx CLI, run the following commands. The given commands will install and create a new project directory containing the code of the canister. The created canister isn't yet prepared to receive direct HTTP calls, that's why in the next step, we will install Pluto.

``` bash
$ sh -ci "$(curl -fsSL https://smartcontracts.org/install.sh)"
$ dfx new project-name --no-frontend --type rust
```

After initializing the new project, we can set up Pluto in it. To do that, open the created folder containing your project and navigate to the backend part of your project (project-name_backend). Then open the Cargo.toml file and add the Pluto dependency. The file should contain something like this (versions may be newer):

### cargo.toml
``` toml
[package]
name = "project-name_backend"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[lib]
crate-type = ["cdylib"]

[dependencies]
candid = "0.8"
ic-cdk = "0.7"
ic-cdk-timers = "0.1" # Feel free to remove this dependency if you don't need timers

pluto = { git = "https://github.com/pu0238/pluto.git" } # <- Pluto deps

```

After Pluto installation simply configure package at lib.rs at src directory. Successfully configured file should contain something like below:

### lib.rs

``` rust
use std::collections::HashMap;
use std::cell::RefCell;

use ic_cdk_macros::{post_upgrade, query, update};
use pluto::{
    http::{
        HttpServe, RawHttpRequest, RawHttpResponse,
        HttpRequest, HttpResponse
    },
    http_serve,
    router::Router,
};
use serde_json::json;

// Build routes
fn setup_routes() {
    let mut router = Router::new();

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

thread_local! {
    // Router storage that contains cached routes
    static ROUTER: RefCell<Router>  = RefCell::new(setup_routes());
}

// System functions
#[post_upgrade]
fn post_upgrade() {
    // After update rebuild routes
    ROUTER.with(|r| *r.borrow_mut() = setup_routes())
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

// Main functions that handle routing
async fn bootstrap(mut app: HttpServe, req: RawHttpRequest) -> RawHttpResponse {
    let router = ROUTER.with(|r| r.borrow().clone());
    app.set_router(router);
    app.serve(req).await
}
```

Run configured canister using below commands:

``` bash
dfx start --background
dfx deploy
```

After deploy we may find in console link to candid ui:
```
http://asrmz-lmaaa-aaaaa-qaaeq-cai.localhost:4943/?id=by6od-j4aaa-aaaaa-qaadq-cai
      |___________________________|                  |___________________________|
           Candid canister ID                             Backend canister ID
```

To access canister HTTP interface replace "backend-canister-id" with your backend canister ID from above and go to: http://backend-carnister-id.localhost:4943/

At given link you should find response from your HTTP server.

