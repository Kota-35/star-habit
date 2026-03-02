use utoipa::OpenApi;

use crate::routes::api::auth::refresh::{RefreshRequest, RefreshResponse};
use crate::routes::api::auth::signin::{SigninRequest, SigninResponse};
use crate::routes::api::auth::signup::{SignupRequest, SignupResponse};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::api::auth::signup::signup,
        crate::routes::api::auth::signin::signin,
        crate::routes::api::auth::refresh::refresh,
    ),
    components(schemas(
        SignupRequest,
        SignupResponse,
        SigninRequest,
        SigninResponse,
        RefreshRequest,
        RefreshResponse,
    ))
)]
pub struct ApiDoc;
