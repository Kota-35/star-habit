use utoipa::OpenApi;

use crate::routes::api::auth::signin::{SigninRequest, SigninResponse};
use crate::routes::api::auth::signup::{SignupRequest, SignupResponse};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::api::auth::signup::signup,
        crate::routes::api::auth::signin::signin,
    ),
    components(schemas(
        SignupRequest,
        SignupResponse,
        SigninRequest,
        SigninResponse,
    ))
)]
pub struct ApiDoc;
