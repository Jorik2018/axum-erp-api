use std::sync::Arc;
use sqlx::MySqlPool;
use crate::service::company_service::CompanyService;
use crate::service::session_service::SessionService;

#[derive(Clone)]
pub struct AppState {

    pub company_service: Arc<CompanyService>,

    pub session_service: SessionService,
    
    pub db: MySqlPool,

}