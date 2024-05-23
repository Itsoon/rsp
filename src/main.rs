use colored::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

mod launch_profile;
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

fn main() {
    let home_dir = match std::env::var("HOME") {
        Ok(val) => val,
        Err(_) => panic!("HOME environment variable is not set"),
    };

    let path = format!("{}/.config/rsp/profiles/arcade-db.toml", home_dir);

    println!("{}", path);

    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Erreur de lecture du fichier: {}", e);
            return;
        }
    };

    let profile: Profile = match toml::from_str(&content) {
        Ok(profile) => profile,
        Err(e) => {
            eprintln!("Erreur de parsing TOML: {}", e);
            return;
        }
    };

    println!("{:#?}", profile);

    for (workspace_name, workspace) in &profile.workspaces {
        println!(
            "{}{}{}",
            "workspace ".blue().bold(),
            workspace_name.blue().bold(),
            " :".blue().bold()
        );

        launch_cmd_block(
            &workspace.kitty_session,
            "kitty_session",
            "No kitty_session found",
        );
        launch_cmd_block(&workspace.kitty_cmd, "kitty_cmd", "No kitty_cmd found");
        launch_cmd_block(&workspace.run, "run", "No run found");
        launch_cmd_block(&workspace.cmd, "cmd", "No cmd found")
    }
}

fn launch_cmd_block(options: &Option<Vec<String>>, block_name: &str, msg: &str) {
    match options {
        Some(opts) => {
            println!("{:?}, {:?}", opts, block_name);
            launch_profile(block_name, opts)
        }
        None => {
            println!("{}", msg.yellow());
        }
    }
}
