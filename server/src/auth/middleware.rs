use std::future::Ready;
use std::pin::Pin;
use std::task::{Context, Poll};

use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{request::Parts, Request, Response, StatusCode},
};
use tower::Layer;
use tower::Service;

use crate::auth::jwt::{JwtConfig, verify_access_token};
use crate::config::env_vars;

#[derive(Clone, Debug)]
pub struct AuthenticatedUser {
    pub user_id: uuid::Uuid,
}

fn jwt_config() -> JwtConfig {
    JwtConfig {
        secret: env_vars().jwt_secret.clone(),
        issuer: env_vars().jwt_issuer.clone(),
        audience: env_vars().jwt_audience.clone(),
        access_ttl_secs: 60 * 15,            // 15分
        refresh_ttl_secs: 60 * 60 * 24 * 30, // 30日
    }
}

fn unauthorized_response() -> Response<Body> {
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(Body::empty())
        .expect("building 401 response")
}

/// 認証必須ルート用の tower Layer。
/// `Authorization: Bearer <access_token>` を検証し、成功時は `AuthenticatedUser` を request extensions に挿入する。
#[derive(Clone, Default)]
pub struct RequireAuthLayer;

impl<S> Layer<S> for RequireAuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for AuthMiddleware<S>
where
    S: Service<Request<Body>, Response = Response<Body>>
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = AuthFuture<S>;

    fn poll_ready(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let bearer = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "));

        let result = match bearer {
            Some(token) => {
                let cfg = jwt_config();
                match verify_access_token(token, &cfg) {
                    Ok(verified) => Ok(verified.claims().id),
                    Err(_) => Err(()),
                }
            }
            None => Err(()),
        };

        match result {
            Err(()) => AuthFuture::Unauthorized(std::future::ready(
                unauthorized_response(),
            )),
            Ok(user_id) => {
                req.extensions_mut().insert(AuthenticatedUser { user_id });
                let mut inner = self.inner.clone();
                AuthFuture::Forward(Box::pin(inner.call(req)))
            }
        }
    }
}

pub enum AuthFuture<S>
where
    S: Service<Request<Body>, Response = Response<Body>>,
{
    Unauthorized(Ready<Response<Body>>),
    Forward(Pin<Box<S::Future>>),
}

impl<S> std::future::Future for AuthFuture<S>
where
    S: Service<Request<Body>, Response = Response<Body>>,
    S::Future: Send,
{
    type Output = Result<S::Response, S::Error>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        match self.get_mut() {
            AuthFuture::Unauthorized(f) => {
                let resp = std::pin::pin!(f).poll(cx);
                match resp {
                    Poll::Ready(r) => Poll::Ready(Ok(r)),
                    Poll::Pending => Poll::Pending,
                }
            }
            AuthFuture::Forward(fut) => fut.as_mut().poll(cx),
        }
    }
}

/// ハンドラで `AuthenticatedUser` を引数に取るための extractor。
/// この extractor を使うルートには `RequireAuthLayer` をかけておくこと。
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts.extensions.remove::<AuthenticatedUser>().ok_or((
            StatusCode::UNAUTHORIZED,
            "Missing or invalid authorization",
        ))
    }
}
