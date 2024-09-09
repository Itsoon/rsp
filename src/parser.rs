use colored::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

use crate::launch_profile;
use launch_profile::launch_profile;

use crate::settings::SETTINGS;

#[derive(Deserialize, Debug)]
struct Workspace {
    kitty_session: Option<Vec<String>>,
    kitty_cmd: Option<Vec<String>>,
    run: Option<Vec<String>>,
    cmd: Option<Vec<String>>,
}
#[derive(Deserialize, Debug)]
struct Monitor {
    workspaces: HashMap<String, Workspace>,
}

// #[derive(Deserialize, Debug)]
// struct Monitor_Profile {
//     monitors: HashMap<String, Monitor>,
// }

// #[derive(Deserialize, Debug)]
// struct Profile {
//     workspaces: HashMap<String, Workspace>,
// }
#[derive(Deserialize, Debug)]
struct Profile {
    monitors: Option<HashMap<String, Workspace>>,
    workspaces: Option<HashMap<String, Workspace>>,
}

impl Profile {
    fn validate(&self) -> Result<(), &'static str> {
        if self.monitors.is_some() || self.workspaces.is_some() {
            Ok(())
        } else {
            Err("Profile must contain either monitors or workspaces")
        }
    }
}

impl Workspace {
    fn validate(&self) -> Result<(), &'static str> {
        if self.kitty_session.is_some()
            || self.kitty_cmd.is_some()
            || self.run.is_some()
            || self.cmd.is_some()
        {
            Ok(())
        } else {
            Err("Workspaces must contain at least one parameter")
        }
    }
}

pub fn file_parser(file: String) {
    let home_dir = match std::env::var("HOME") {
        Ok(val) => val,
        Err(_) => {
            panic!("HOME environment variable is not set");
        }
    };

    let path = format!("{}/.config/rsp/profiles/{}", home_dir, file);

    if let Some(true) = SETTINGS.settings.debug {
        println!("{}", path);
    }

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

    match profile.validate() {
        Ok(_) => println!("Profile 1: {:?}", profile),
        Err(e) => eprintln!("Profile 1 Error: {}", e),
    }

    // let mut profile: Option<Profile> = None;

    // match toml::from_str(&content) {
    //     Ok(prf) => profile = Some(prf),
    //     Err(_) => match toml::from_str(&content) {
    //         Ok(prf) => profile = Some(prf),
    //         Err(e) => {
    //             eprintln!("TOML parsing errors: {}", e);
    //             return;
    //         }
    //     },
    // };
    println!("{:?}", profile);

    // for (workspace_name, workspace) in &profile.monitors {
    //     if let Some(true) = SETTINGS.settings.debug {
    //         println!(
    //             "{}{}{}",
    //             "workspace ".bold(),
    //             workspace_name.bold(),
    //             " :".bold()
    //         );
    //     }

    //     launch_cmd_block(
    //         &workspace.kitty_session,
    //         workspace_name,
    //         "kitty_session",
    //         "No kitty_session found",
    //     );
    //     launch_cmd_block(
    //         &workspace.kitty_cmd,
    //         workspace_name,
    //         "kitty_cmd",
    //         "No kitty_cmd found",
    //     );
    //     launch_cmd_block(&workspace.run, workspace_name, "run", "No run found");
    //     launch_cmd_block(&workspace.cmd, workspace_name, "cmd", "No cmd found");
    // }
}

fn launch_cmd_block(
    options: &Option<Vec<String>>,
    workspace_name: &str,
    block_name: &str,
    msg: &str,
) {
    match options {
        Some(opts) => {
            if let Some(true) = SETTINGS.settings.debug {
                print!("{} {}", block_name.bright_green(), "found\n".bright_green());
            }
            launch_profile(block_name, workspace_name, opts);
        }
        None => {
            if let Some(true) = SETTINGS.settings.debug {
                println!("{}", msg.yellow());
            }
        }
    }
}
