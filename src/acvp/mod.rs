use anyhow::{Context, Result};
use log::{info, warn};
use reqwest::Client;
use std::path::Path;

use crate::config::Config;
use crate::subprocess::Subprocess;

pub struct ACVPClient {
    config: Config,
    #[allow(dead_code)]
    client: Client,
    #[allow(dead_code)]
    subprocess: Subprocess,
    access_token: Option<String>,
}

impl ACVPClient {
    pub async fn new(config: Config, wrapper_path: &Path, param: Option<&str>) -> Result<Self> {
        let client = Client::builder()
            .danger_accept_invalid_certs(false)
            .build()?;

        let subprocess = Subprocess::new(wrapper_path, param)?;

        Ok(Self {
            config,
            client,
            subprocess,
            access_token: None,
        })
    }

    pub async fn fetch_vectors(&mut self, primitive: &str) -> Result<()> {
        info!("Fetching vectors for primitive: {}", primitive);
        self.authenticate().await?;

        warn!("ACVP server interaction not fully implemented in this port");
        warn!("Use file-based mode with --in and --out instead");

        Ok(())
    }

    pub async fn run_tests(&mut self, primitive: &str) -> Result<()> {
        info!("Running tests for primitive: {}", primitive);
        self.authenticate().await?;

        warn!("ACVP server interaction not fully implemented in this port");
        warn!("Use file-based mode with --in and --out instead");

        Ok(())
    }

    async fn authenticate(&mut self) -> Result<()> {
        if self.access_token.is_some() {
            return Ok(());
        }

        info!(
            "Authenticating with ACVP server: {}",
            self.config.acvp_server
        );

        if let Some(ref totp_secret) = self.config.totp_secret {
            let totp = self.generate_totp(totp_secret)?;
            info!("Generated TOTP: {}", totp);
        }

        Ok(())
    }

    fn generate_totp(&self, secret: &str) -> Result<String> {
        use base64::Engine;
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(secret)
            .context("Invalid TOTP secret")?;
        let totp = totp_lite::totp_custom::<totp_lite::Sha256>(30, 6, &decoded, 0);
        Ok(format!("{:06}", totp))
    }
}
