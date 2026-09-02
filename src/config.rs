use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(rename = "certPEMFile")]
    pub cert_pem_file: Option<String>,
    #[serde(rename = "privateKeyFile")]
    pub private_key_file: Option<String>,
    #[serde(rename = "privateKeyDERFile")]
    pub private_key_der_file: Option<String>,
    #[serde(rename = "totpSecret")]
    pub totp_secret: Option<String>,
    pub acvp_server: String,
    pub session_tokens_cache: Option<String>,
    pub log_file: Option<String>,
}

impl Config {
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {:?}", path))?;

        let content = Self::remove_comments(&content);

        serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {:?}", path))
    }

    fn remove_comments(content: &str) -> String {
        content
            .lines()
            .filter(|line| {
                let trimmed = line.trim_start();
                !trimmed.starts_with("//")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_comments() {
        let input = r#"{
  // This is a comment
  "key": "value",
  // Another comment
  "number": 42
}"#;
        let result = Config::remove_comments(input);
        assert!(!result.contains("// This is a comment"));
        assert!(result.contains("\"key\": \"value\""));
    }

    #[test]
    fn test_config_parsing() {
        let json = r#"{
            "acvpServer": "https://demo.acvts.nist.gov/",
            "certPEMFile": "cert.pem",
            "totpSecret": "BASE64SECRET"
        }"#;

        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.acvp_server, "https://demo.acvts.nist.gov/");
        assert_eq!(config.cert_pem_file, Some("cert.pem".to_string()));
        assert_eq!(config.totp_secret, Some("BASE64SECRET".to_string()));
    }

    #[test]
    fn test_config_with_comments() {
        let json = r#"{
            // Server configuration
            "acvpServer": "https://demo.acvts.nist.gov/",
            // Authentication
            "certPEMFile": "cert.pem"
        }"#;

        let cleaned = Config::remove_comments(json);
        let config: Result<Config, _> = serde_json::from_str(&cleaned);
        assert!(config.is_ok());
    }

    #[test]
    fn test_config_optional_fields() {
        let json = r#"{
            "acvpServer": "https://test.server.com/"
        }"#;

        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.acvp_server, "https://test.server.com/");
        assert!(config.cert_pem_file.is_none());
        assert!(config.totp_secret.is_none());
    }
}
