use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct VaultResponse {
    data: VaultData,
}

#[derive(Debug, Deserialize)]
struct VaultData {
    data: HashMap<String, String>,
}

pub async fn get_secret(
    vault_addr: &str,
    vault_token: &str,
    path: &str,
    key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/v1/secret/data/{}",
        vault_addr.trim_end_matches('/'),
        path.trim_start_matches('/')
    );

    let response = Client::new()
        .get(&url)
        .header("X-Vault-Token", vault_token)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    if !status.is_success() {
        return Err(format!(
            "Vault request failed: status={}, url={}, body={}",
            status,
            url,
            body
        )
        .into());
    }

    let response: VaultResponse = serde_json::from_str(&body)?;

    response
        .data
        .data
        .get(key)
        .cloned()
        .ok_or_else(|| {
            format!(
                "Secret key '{}' not found at '{}'",
                key,
                path
            )
            .into()
        })
}