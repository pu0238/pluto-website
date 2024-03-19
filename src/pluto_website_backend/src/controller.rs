use ic_cdk::api;
use pluto::{
    http::{HttpRequest, HttpResponse},
    router::Router,
};
use serde_json::json;
use std::collections::HashMap;

pub(crate) fn backend(router: &mut Router) {
    router.get("/stats", false, |_req: HttpRequest| async move {
        Ok(HttpResponse {
            status_code: 200,
            headers: HashMap::new(),
            body: json!({
                "statusCode": 200,
                "time": api::time(),
                "id": api::id(),
                "canisterBalance": api::canister_balance128(),
                "message": "Hello World!",
            })
            .into(),
        })
    });
}
