use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process::{Command, Stdio};

const SESSION_FILE: &str = "/home/roki/.cache/hypr-session.json";

#[derive(Serialize, Deserialize, Debug)]
struct WindowInfo {
    workspace: i32,
    class: String,
}

fn get_clients() -> Result<Vec<WindowInfo>> {
    let output = Command::new("hyprctl")
        .args(["clients", "-j"])
        .output()
        .context("Failed to execute hyprctl")?;

    let clients: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout)?;
    let mut windows = Vec::new();

    for client in clients {
        if let (Some(workspace), Some(class)) = (client.get("workspace"), client.get("class")) {
            let workspace_id = workspace.get("id").and_then(|id| id.as_i64()).unwrap_or(0) as i32;
            let class_name = class.as_str().unwrap_or_default().to_string();
            windows.push(WindowInfo {
                workspace: workspace_id,
                class: class_name,
            })
        }
    }

    Ok(windows)
}

fn save_session(path: &str) -> Result<()> {
    let windows = get_clients()?;
    let data = serde_json::to_string_pretty(&windows)?;

    fs::write(path, data)?;
    println!("Session save to {path}");

    Ok(())
}

fn load_session(path: &str) -> Result<()> {
    let data = fs::read_to_string(path)?;
    let windows: Vec<WindowInfo> = serde_json::from_str(&data)?;

    let mut app_map = HashMap::new();
    app_map.insert("telegram-desktop_telegram-desktop", "telegram-desktop");
    app_map.insert("firefox", "firefox");
    app_map.insert("Spotify", "spotify-launcher");
    app_map.insert("kitty", "kitty");

    for window in windows {
        if let Some(cmd) = app_map.get(window.class.as_str()) {
            Command::new("hyprctl")
                // .args(["dispatch", "workspace", window.workspace.to_string()])
                .arg("dispatch")
                .arg("workspace")
                .arg(window.workspace.to_string())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()?;

            Command::new(cmd)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .context(format!("Failed to launch {cmd}"))?;
        }
    }

    println!("Session loaded");
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("save") => save_session(SESSION_FILE)?,
        Some("load") => load_session(SESSION_FILE)?,
        _ => {
            println!("Usage: hypr-session load|save");
        }
    }

    Ok(())
}
