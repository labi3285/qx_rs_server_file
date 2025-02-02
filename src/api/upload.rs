use axum::{
    extract::{Multipart, DefaultBodyLimit},
    routing,
    Router,
};

use qx_rs_server::err::Result;
use qx_rs_server::def::resp_json::{ApplicationJson, Payload};

use crate::service;


pub fn route() -> Router {
    Router::new().nest(
        "/upload",
        Router::new()
            .route("/upload_to_path", routing::post(upload_to_path).layer(DefaultBodyLimit::max(1024 * 1024 * 999)))
    )
}


async fn upload_to_path(
    multipart: Multipart
) -> Result<ApplicationJson<Payload<String>>>
{
    let url = service::upload::upload_to_path(multipart).await?;
    Ok(ApplicationJson::payload(url))
}

