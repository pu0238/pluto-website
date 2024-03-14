---
layout: ../../../layouts/DocsLayout.astro
title: "Adding Body Validation"
pubDate: 2024-01-20
author: "Reyth"
---

In this section we're going to extend our REST API with strongly typed body validation. This is going to allow us to catch errors in parameters passed in by the users of our service. That in turn will lead to better and clearer error messages.

Let's get started with the basics. For this we will need a few new dependencies - namely `serde` for serializing and deserializing our request data, as well as the `validator` crate,which is an extremely handy way to do struct field validation. Let's set up our Cargo.toml with these:

```toml
[dependencies]
ic-cdk = "0.11.3"
candid = "0.9.6"
pluto = { git = "https://github.com/pu0238/pluto" }
serde = "1.0"
serde_json = "1.0.108"
validator = { version = "0.16", features = ["derive"] }
```

Great! Now we're ready to add our first validated endpoint.

## Adding an addition endpoint

Let's add an endpoint that will allow users to add up two numbers together. If this sounds simple, that's because it is. But those simple examples are great for illustrating the principles behind using new features of frameworks. First of all, we're going to set up a new model for our request in a file called `model.rs`. This will let us use it in the future. Don't forget to add it to `lib.rs`.

```rs
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidateArgs, ValidationError};

#[derive(Debug, Validate, Serialize, Deserialize)]

pub struct AddRequest {
    #[validate(range(min = 0, max = 100))]
    pub a: i32,
    #[validate(range(min = 0, max = 100))]
    pub b: i32,
}
```

Notice how our struct uses the `Validate` derive macro, which handles 80% of the validation part for us. Now we just need to add a new route to our `controller.rs` file:

```rs
    // Helper method for displaying an error back to the request sender
    pub fn map_validation_err(err: ValidationErrors) -> HttpResponse {
        HttpResponse {
            status_code: 400,
            headers: HashMap::new(),
            body: json!({
                "statusCode": 400,
                "message": err.to_string(),
            })
            .into(),
        }
    }
    // ...
    router.post("/add", false, |req: HttpRequest| async move {
        // This call will automatically handle the validation of the request body in addition to parsing it into a struct
        let my_body: AddRequest = req.body_into_struct()?;
        my_params.validate().map_err(map_validation_err)?;
        let sum = my_body.a + my_body.b;
        Ok(HttpResponse {
            status_code: 200,
            headers: HashMap::new(),
            body: json!({
                "statusCode": 200,
                "sum": sum
            })
            .into(),
        })
    });
```

With this our endpoint is complete! It will notify the client about any errors in the request, and once everything is passed in correctly, it will actually process the addition.
