use utoipa::OpenApi;

use crate::routes::api::auth::signup::{Signup, User};

#[derive(OpenApi)]
#[openapi(
    paths(crate::routes::api::auth::signup::signup),
    components(schemas(Signup, User))
)]
pub struct ApiDoc;
