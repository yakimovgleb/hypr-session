use std::{fs, io::{stdout, Error, ErrorKind, Result}, path::{Path, PathBuf}, process::{Command, Stdio}};

use serde::Deserialize;


const CONFIG_FILE: &str = "hypr/hypr-session.json";

#[derive(Deserialize)]
struct AppInfo {
    workspace: Option<i32>,
    name: String,
    command: String,
}

fn get_config_file() -> PathBuf {
    dirs::config_dir()
        .unwrap()
        .join(CONFIG_FILE)
}

fn load_from_json(path: PathBuf) -> Result<Vec<AppInfo>> {
    println!("{path:?}");
    if !Path::exists(&path) {
        return Err(Error::new(ErrorKind::InvalidFilename, "Config file doesn't exist"));
    }

    let file = fs::read_to_string(path)?;

    let json: Vec<AppInfo> = serde_json::from_str(&file)?;

    Ok(json)
}

pub fn load_session() -> Result<()> {
    let path = get_config_file();
    let apps = load_from_json(path)?;

    for a in apps {
        if let Some(workspace) = a.workspace {
            Command::new("hyprctl")
                .arg("dispatch")
                .arg("workspace")
                .arg(workspace.to_string())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()?;
        }

        Command::new(a.command)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        println!("'{}' launched.", a.name);
    }

    Ok(())
}
