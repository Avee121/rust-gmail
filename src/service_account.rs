use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Serialize, Deserialize, Clone)]
pub struct ServiceAccount {
    #[serde(rename = "type")]
    pub account_type: String,
    pub project_id: String,
    pub private_key_id: String,
    pub private_key: String,
    pub client_email: String,
    pub client_id: String,
    pub auth_uri: String,
    pub token_uri: String,
    pub auth_provider_x509_cert_url: String,
    pub client_x509_cert_url: String,
}

impl ServiceAccount {
    pub fn load_from_file<P: AsRef<Path>>(file_path: P) -> Result<Self> {
        let file_contents = fs::read_to_string(file_path)?;
        Ok(serde_json::from_str(&file_contents)?)
    }
}
