use utoipa::OpenApi;

use crate::routes::api::auth::signup::{SignupRequest, SignupResponse};

#[derive(OpenApi)]
#[openapi(
    paths(crate::routes::api::auth::signup::signup),
    components(schemas(SignupRequest, SignupResponse))
)]
pub struct ApiDoc;
