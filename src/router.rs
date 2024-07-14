use axum::{routing::post, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::stamp;

#[derive(OpenApi)]
#[openapi(paths(stamp::stamp), components(schemas(stamp::StampRequest, stamp::Mode)))]
struct ApiDoc;

pub fn init_router() -> Router {
    Router::new()
        .route("/api/stamp", post(stamp::stamp))
        .merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi()))
}
