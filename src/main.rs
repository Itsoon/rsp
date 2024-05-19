use serde::Deserialize;
use std::fs;

mod persistent_cmd;

use persistent_cmd::persistent_cmd;

#[derive(Debug, Deserialize)]
struct Profile {
    run: Option<Vec<String>>,
    #[serde(rename = "kitty-session")]
    kitty_session: Option<Vec<String>>,
    commands: Option<Vec<String>>,
}

fn main() {
    let path = "./src/profile.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let profiles: Vec<serde_json::Value> = serde_json::from_str(&data).expect("Unable to parse");

    let profile = &profiles[0];
    if let Some(profile_obj) = profile.as_object() {
        if let Some((_profile_key, profile_value)) = profile_obj.iter().next() {
            let profile_data: Profile =
                serde_json::from_value(profile_value.clone()).expect("Unable to parse profile");

            if let Some(run) = &profile_data.run {
                println!("{:?}", run);
                for app in run {
                    println!("{}", app);
                    persistent_cmd(app, None);
                }
            }

            if let Some(kitty_session) = &profile_data.kitty_session {
                for command in kitty_session {
                    let mut args = vec!["--session"];
                    args.push(command);
                    println!("{:?}", args);
                    persistent_cmd("kitty", Some(&args));
                }
            }

            if let Some(commands) = &profile_data.commands {
                for command in commands {
                    let mut args = vec!["sh", "-c"];
                    let formatted_command = format!("{} && exec zsh", command);
                    args.push(&formatted_command);
                    persistent_cmd("kitty", Some(&args));
                }
            }
        }
    }
}
