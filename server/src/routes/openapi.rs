use utoipa::OpenApi;

use crate::routes::api::auth::refresh::{RefreshRequest, RefreshResponse};
use crate::routes::api::auth::signin::{SigninRequest, SigninResponse};
use crate::routes::api::auth::signup::{SignupRequest, SignupResponse};
use crate::routes::api::me::MeResponse;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::api::auth::signup::signup,
        crate::routes::api::auth::signin::signin,
        crate::routes::api::auth::refresh::refresh,
        crate::routes::api::me::me,
    ),
    components(schemas(
        SignupRequest,
        SignupResponse,
        SigninRequest,
        SigninResponse,
        RefreshRequest,
        RefreshResponse,
        MeResponse,
    )),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}
