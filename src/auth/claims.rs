use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Option<String>,
    pub upn: Option<String>,
    #[serde(default)]
    pub groups: Vec<String>,
    pub exp: usize,
    pub iat: Option<usize>,
    pub iss: Option<String>,
}

impl Claims {
    pub fn has_group(&self, group: &str) -> bool {
        self.groups.iter().any(|g| g == group)
    }
}
