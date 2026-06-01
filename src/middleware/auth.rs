use axum::{
    body::Body, extract::State, http::{Request, StatusCode}, middleware::Next, response::Response
};
use std::sync::Arc;

use jsonwebtoken::{decode, Algorithm, Validation};

use crate::{
    state::AppState,
    middleware::claims::Claims, // o donde pongas Claims
    middleware::keys::load_decoding_key,
};

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    
    // 🔐 1. Obtener Authorization header
    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(h) if h.starts_with("Bearer ") => &h[7..],
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    // 🔑 2. Validar JWT
    let decoding_key = load_decoding_key();

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&["https://example.com/issuer"]);

    let token_data = match decode::<Claims>(token, &decoding_key, &validation) {
    Ok(data) => data,
    Err(e) => {
        eprintln!("❌ JWT decode failed: {:#?}", e);
        return Err(StatusCode::UNAUTHORIZED);
    }
};

    let claims = token_data.claims;

    // 🧠 3. Validar sesión en Redis
    // 👉 puedes usar el token completo como key (como estás haciendo)
    let session = match state.session_service
        .get(token).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ Redis error: {}", e); // 👈 IMPORTANTE
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
println!("token=[{}]", token);
    if session.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // 🔥 4. Guardar claims para handlers
    req.extensions_mut().insert(claims);
//mismatched types
//expected struct `axum::http::Request<Body>`
  // found struct `axum::http::Request<B>
    Ok(next.run(req).await)
}