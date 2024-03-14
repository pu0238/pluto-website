use std::collections::HashMap;

use pluto::{http::HttpRequest, render_view, router::Router};

use crate::compiled::templates::index_html;

pub(crate) fn setup() -> Router {
    let mut router = Router::new();
    router.get("/", false, |_req: HttpRequest| async move {
        render_view!(index_html);
    });

    router
}