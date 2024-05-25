use colored::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

use crate::launch_profile;
use launch_profile::launch_profile;

#[derive(Deserialize, Debug)]
struct Workspace {
    kitty_session: Option<Vec<String>>,
    kitty_cmd: Option<Vec<String>>,
    run: Option<Vec<String>>,
    cmd: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
struct Profile {
    workspaces: HashMap<String, Workspace>,
}

pub fn file_parser(file: String) {
    let home_dir = match std::env::var("HOME") {
        Ok(val) => val,
        Err(_) => panic!("HOME environment variable is not set"),
    };

    let path = format!("{}/.config/rsp/profiles/{}", home_dir, file);

    println!("{}", path);

    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let profile: Profile = match toml::from_str(&content) {
        Ok(profile) => profile,
        Err(e) => {
            eprintln!("TOML parsing errors: {}", e);
            return;
        }
    };

    for (workspace_name, workspace) in &profile.workspaces {
        println!(
            "{}{}{}",
            "workspace ".blue().bold(),
            workspace_name.blue().bold(),
            " :".blue().bold()
        );

        launch_cmd_block(
            &workspace.kitty_session,
            workspace_name,
            "kitty_session",
            "No kitty_session found",
        );
        launch_cmd_block(
            &workspace.kitty_cmd,
            workspace_name,
            "kitty_cmd",
            "No kitty_cmd found",
        );
        launch_cmd_block(&workspace.run, workspace_name, "run", "No run found");
        launch_cmd_block(&workspace.cmd, workspace_name, "cmd", "No cmd found");
    }
}

fn launch_cmd_block(
    options: &Option<Vec<String>>,
    workspace_name: &str,
    block_name: &str,
    msg: &str,
) {
    match options {
        Some(opts) => launch_profile(block_name, workspace_name, opts),
        None => {
            println!("{}", msg.yellow());
        }
    }
}
