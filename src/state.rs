use std::sync::Arc;

use sqlx::MySqlPool;

use crate::auth::JwtService;
use crate::service::company_service::CompanyService;
use crate::service::session_service::SessionService;

#[derive(Clone)]
pub struct AppState {
    pub company_service: Arc<CompanyService>,

    pub session_service: Option<SessionService>,

    pub db: MySqlPool,

    pub jwt: Arc<JwtService>,
}