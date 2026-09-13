use axum::{extract::FromRequestParts, http::{header::AUTHORIZATION, request::Parts}};

use crate::{error::ApiError, AppState};
use super::Claims;

#[derive(Debug, Clone)]
pub struct AuthUser(pub Claims);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(ApiError::Unauthorized)?;

        let token = header
            .strip_prefix("Bearer ")
            .ok_or(ApiError::Unauthorized)?;

        let claims = state.jwt.validate(token).map_err(|_| ApiError::Unauthorized)?;
        Ok(Self(claims))
    }
}
