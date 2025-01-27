//! Connections to remote Jupyter servers over HTTP and WebSocket.

use std::time::Duration;

use reqwest::{
    header::{self, HeaderMap},
    StatusCode,
};
use serde::Deserialize;
use serde_json::json;
use time::OffsetDateTime;
use url::Url;

use super::{create_websocket_connection, KernelConnection};
use crate::Error;

/// A running Jupyter kernel connected over the WebSocket wire protocol.
#[derive(Clone)]
pub struct RemoteKernel {
    client: JupyterClient,
    kernel_id: String,
    conn: KernelConnection,
}

impl RemoteKernel {
    /// Start a new kernel on the server.
    pub async fn start(client: &JupyterClient, spec_name: &str) -> Result<Self, Error> {
        let kernel_info = client.create_kernel(spec_name).await?;

        let ws_url = client
            .server_url
            .join(&format!("/api/kernels/{}/channels", kernel_info.id))?;
        let mut ws_url = ws_url.to_string();
        if ws_url.starts_with("https://") {
            ws_url = ws_url.replacen("https://", "wss://", 1);
        } else {
            ws_url = ws_url.replacen("http://", "ws://", 1);
        }

        let conn = create_websocket_connection(&ws_url, &client.token).await?;

        Ok(Self {
            client: client.clone(),
            kernel_id: kernel_info.id,
            conn,
        })
    }

    /// Get the kernel ID.
    pub fn id(&self) -> &str {
        &self.kernel_id
    }

    /// Kill the kernel and delete its kernel ID.
    pub async fn kill(self) -> Result<(), Error> {
        self.client.kill_kernel(&self.kernel_id).await
    }

    /// Get a reference to the kernel connection object.
    pub fn conn(&self) -> &KernelConnection {
        &self.conn
    }
}

/// HTTP client for a remote Jupyter server.
///
/// This client can make REST API requests and create new WebSocket connections.
/// It is generally stateless and cheaply cloneable though.
#[derive(Clone)]
pub struct JupyterClient {
    server_url: Url,
    token: String,
    http_client: reqwest::Client,
}

impl JupyterClient {
    /// Return a new client to a Jupyter server without connecting.
    pub fn new(server_url: &str, token: &str) -> Result<Self, Error> {
        let headers = HeaderMap::from_iter([(
            header::AUTHORIZATION,
            format!("token {token}")
                .parse()
                .expect("server token parse"),
        )]);
        let server_url = Url::parse(server_url)?;
        let http_client = reqwest::ClientBuilder::new()
            .connect_timeout(Duration::from_secs(1))
            .default_headers(headers)
            .build()?;

        Ok(Self {
            server_url,
            token: token.into(),
            http_client,
        })
    }

    /// Get the API version of the Jupyter server.
    pub async fn get_api_version(&self) -> Result<String, Error> {
        let url = self.server_url.join("/api")?;
        let resp = self.http_client.get(url).send().await?.error_for_status()?;

        #[derive(Deserialize)]
        struct ApiVersion {
            version: String,
        }
        Ok(resp.json::<ApiVersion>().await?.version)
    }

    /// List the active kernels on the Jupyter server.
    pub async fn list_kernels(&self) -> Result<Vec<KernelInfo>, Error> {
        let url = self.server_url.join("/api/kernels")?;
        let resp = self.http_client.get(url).send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    /// Get information about a specific kernel by its ID.
    pub async fn get_kernel_by_id(&self, kernel_id: &str) -> Result<Option<KernelInfo>, Error> {
        let url = self.server_url.join(&format!("/api/kernels/{kernel_id}"))?;
        let resp = self.http_client.get(url).send().await?;
        if resp.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }
        Ok(resp.error_for_status()?.json().await?)
    }

    /// Create a new kernel from the spec with the give name.
    pub async fn create_kernel(&self, spec_name: &str) -> Result<KernelInfo, Error> {
        let url = self.server_url.join("/api/kernels")?;
        let resp = self
            .http_client
            .post(url)
            .json(&json!({ "name": spec_name }))
            .send()
            .await?
            .error_for_status()?;
        Ok(resp.json().await?)
    }

    /// Kill a kernel and delete its kernel ID.
    pub async fn kill_kernel(&self, kernel_id: &str) -> Result<(), Error> {
        let url = self.server_url.join(&format!("/api/kernels/{kernel_id}"))?;
        self.http_client
            .delete(url)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}

/// Information about a remote Jupyter kernel.
#[derive(Clone, Debug, Deserialize)]
pub struct KernelInfo {
    /// The unique identifier of the kernel.
    pub id: String,

    /// Name of the type of kernel being run (e.g., `python3`).
    pub name: String,

    /// Last activity ISO timestamp, typically UTC.
    #[serde(with = "time::serde::iso8601")]
    pub last_activity: OffsetDateTime,

    /// The execution state of the kernel: `starting`, `running`, etc.
    pub execution_state: String,

    /// The number of active connections to the kernel.
    pub connections: u32,
}
