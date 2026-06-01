#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Claims {

    pub iss: String,
    pub exp: usize,
    pub upn: Option<String>,
    pub uid: Option<i64>,
    pub user: Option<String>,
    pub fullName: Option<String>,
    pub directory: Option<String>,
    pub groups: Option<Vec<String>>,

}