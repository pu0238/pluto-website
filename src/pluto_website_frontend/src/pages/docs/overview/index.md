---
layout: ../../../layouts/DocsLayout.astro
pubDate: 2024-02-21
author: "Pu0238"
---

# Overview

The Pluto Framework allows for easy and intuitive management of [API](https://en.wikipedia.org/wiki/API) paths. This is possible thanks to maintaining standards that make it easy to build code. This framework, with simple initialization, allows for the creation of paths that, depending on the decisions of the developer creating a specific path, can introduce changes to the data stored in the container.

## Input methods:

Below are two input methods: "[http_request](https://internetcomputer.org/docs/current/references/http-gateway-protocol-spec#upgrade-to-update-calls) " and "[http_request_update](https://internetcomputer.org/docs/current/references/http-gateway-protocol-spec#upgrade-to-update-calls) ". Both of these methods are input methods, however, only one of them is capable of making changes to the container.

When querying the container through the [HTTP interface](https://en.wikipedia.org/wiki/HTTP) , the request initially goes to the "[http_request](https://github.com/pu0238/pluto/blob/de36ada4305cb6ad9c2276fbece975878ff3d947/src/pluto/src/http.rs#L267)" method. In the event that the path is found and it is not mutable, the request will be processed on 1 node, and then return a response. However, if the request encounters a mutable path, it will be delegated from the "http_request" method to the "http_request_update" method in order to be able to make changes to the container data. The "http_request_update" method will involve the entire subsystem to validate the request's correctness.
``` rust
#[query]
async fn http_request(req: RawHttpRequest) -> RawHttpResponse {
    bootstrap(http_serve!(), req).await
}

#[update]
async fn http_request_update(req: RawHttpRequest) -> RawHttpResponse {
    bootstrap(http_serve!(), req).await
}
```

The "http_serve" macro dynamically recognizes the input functions in which it is located and then provides the appropriate variation of the main method. This method, with the data passed from the macro, will execute paths as mutable or as query.

## Routing

The "bootstrap" function handles the main execution of the program. It registers the router, configures the server, and then executes a specific method from the provided path. This function utilizes the previously saved router to maximize efficiency. The router is registered with each upgrade of the container's code.

``` rust
async fn bootstrap(mut app: HttpServe, req: RawHttpRequest) -> RawHttpResponse {
    let router = ROUTER.with(|r| r.borrow().clone());
    app.set_router(router);
    app.serve(req).await
}
```

The router should be initialized in the following way:

``` rust
thread_local! {
    static ROUTER: RefCell<Router>  = RefCell::new(controller::setup());
}
```

This initialization allows for path caching, which saves time needed for initialization.

Routes can be defined within a function that returns a "[Router](https://github.com/pu0238/pluto/blob/de36ada4305cb6ad9c2276fbece975878ff3d947/src/pluto/src/router.rs#L21C12-L21C19)". The "Router" structure should have methods implemented to allow adding paths handling individual HTTP requests (GET, HEAD, OPTIONS, POST, PUT, PATCH, DELETE). These methods should return an "HttpResponse" structure, and any processed data will be saved IF the path is defined as mutable.

Below is an example of a created router:

``` rust
    let mut router = Router::new();
    router.get("/", false, |_req: HttpRequest| async move {
              |___| |____| |_________________|
                /      \                  \
         API path     Is path mutable?   Request content

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
```

"HttpResponse" is a basic struct representing the response to an HTTP request. It includes a status code, headers, and a body. The status is the response code. The header is a hash map of the response, and the body can be JSON or a string.

