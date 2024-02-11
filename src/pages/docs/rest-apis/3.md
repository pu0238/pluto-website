---
layout: ../../../layouts/DocsLayout.astro
title: "Adding CORS"
pubDate: 2024-01-29
author: "Reyth"
---

We often face issues where we don't want our backend to be called from just any old website. That is why we decided to implement CORS helper methods into Pluto. In this section we're going to quickly go over the steps to add CORS to our server.

## Bootstraping CORS

To add CORS module, let's open the `bootstrap.rs` file and modify the `bootstrap()` method accordingly:

```rs
async fn bootstrap(mut app: HttpServe, req: RawHttpRequest) -> RawHttpResponse {
    let router = ROUTER.with(|r| r.borrow().clone());
    let cors = Cors::new()
        .allow_origin("*")
        .allow_methods(vec![Method::POST, Method::PUT])
        .allow_headers(vec!["Content-Type", "Authorization"])
        .max_age(Some(3600));

    app.set_router(router);
    app.use_cors(cors);
    app.serve(req).await
}
```

With the addition of the `Cors` struct we can now restrict our origin policy in any way we would like. That's it, nothing else is required beyond these few lines of code. Now the appropriate headers will be added to every preflight request.
