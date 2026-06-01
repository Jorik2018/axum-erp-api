//src/service/company_service.rs
use crate::{
    //cannot find `entity` in `crate`
//could not find `entity` in the crate root
    entity::company::{Company},
    repository::company_repository::CompanyRepository,
};

pub struct CompanyService {
    pub repo: CompanyRepository,
}

impl CompanyService {
    pub fn new(repo: CompanyRepository) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> Vec<Company> {
        self.repo.find_all().await.unwrap()
    }

    pub async fn get_by_id(&self, id: i64) -> Option<Company> {
        self.repo.find_by_id(id).await.unwrap()
    }

    pub async fn create(&self, data: Company) -> i64 {
        self.repo.create(data).await.unwrap()
    }

    pub async fn update(&self, id: i64, data: Company) {
        self.repo.update(id, data).await.unwrap()
    }

    pub async fn delete(&self, id: i64) {
        self.repo.delete(id).await.unwrap()
    }
}