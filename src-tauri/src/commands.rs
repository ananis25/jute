//! Invoke handlers for commands callable from the frontend.

use std::env;

use sysinfo::System;
use tauri::ipc::Channel;
use tracing::info;

use crate::{
    backend::{
        commands::{self, RunCellEvent},
        local::{environment, LocalKernel},
        notebook::NotebookRoot,
    },
    state::State,
    Error,
};

pub mod venv;

/// Measure the current system CPU usage. (unused, for future reference)
#[tauri::command]
pub async fn cpu_usage() -> f32 {
    let mut system = System::new();
    system.refresh_cpu();
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    system.refresh_cpu();
    system.global_cpu_info().cpu_usage()
}

/// Start a new Jupyter kernel.
#[tauri::command]
pub async fn start_kernel(
    spec_name: &str,
    state: tauri::State<'_, State>,
) -> Result<String, Error> {
    // TODO: Save the client in a better place.
    // let client = JupyterClient::new("", "")?;

    // Temporary hack to just start a kernel locally with ZeroMQ.
    let kernels = environment::list_kernels(None).await;
    let mut kernel_spec = match kernels
        .iter()
        .find(|(path, _spec)| path.file_name().and_then(|s| s.to_str()) == Some(spec_name))
    {
        Some((_, kernel_spec)) => kernel_spec.clone(),
        None => {
            return Err(Error::KernelConnect(format!(
                "no kernel named {spec_name:?} found"
            )))
        }
    };

    if kernel_spec.argv[0] == "python" {
        if let Ok(python_path) = env::var("PYTHON_PATH") {
            kernel_spec.argv[0] = python_path;
        } else {
            // Temporary hack
            kernel_spec.argv[0] = "/opt/homebrew/bin/python3.11".into();
        }
    }

    let kernel = LocalKernel::start(&kernel_spec).await?;

    let info = commands::kernel_info(kernel.conn()).await?;
    info!(banner = info.banner, "started new jute kernel");

    let kernel_id = String::from(kernel.id());
    state.kernels.insert(kernel_id.clone(), kernel);
    Ok(kernel_id)
}

/// Stop a Jupyter kernel.
#[tauri::command]
pub async fn stop_kernel(kernel_id: &str, state: tauri::State<'_, State>) -> Result<(), Error> {
    info!("stopping jute kernel {kernel_id}");
    let (_, mut kernel) = state
        .kernels
        .remove(kernel_id)
        .ok_or(Error::KernelDisconnect)?;
    kernel.kill().await?;
    Ok(())
}

/// Get the contents of a Jupyter notebook on disk.
#[tauri::command]
pub async fn get_notebook(path: &str) -> Result<NotebookRoot, Error> {
    info!("getting notebook at {path}");

    let contents = tokio::fs::read_to_string(path)
        .await
        .map_err(Error::Filesystem)?;
    Ok(serde_json::from_str(&contents)?)
}

/// Run a code cell in a Jupyter kernel.
#[tauri::command]
pub async fn run_cell(
    kernel_id: &str,
    code: &str,
    on_event: Channel<RunCellEvent>,
    state: tauri::State<'_, State>,
) -> Result<(), Error> {
    let conn = state
        .kernels
        .get(kernel_id)
        .ok_or(Error::KernelDisconnect)?
        .conn()
        .clone();

    let rx = commands::run_cell(&conn, code).await?;
    while let Ok(event) = rx.recv().await {
        if on_event.send(event).is_err() {
            break;
        }
    }
    Ok(())
}