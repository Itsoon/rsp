use serde::Deserialize;
use serde_json::Value;

mod persistent_cmd;
use persistent_cmd::persistent_cmd;

#[derive(Debug, Deserialize)]
struct Profile {
    run: Option<Vec<String>>,
    #[serde(rename = "kitty-session")]
    kitty_session: Option<Vec<String>>,
    commands: Option<Vec<String>>,
}

pub fn launch_profile(profile: &Value) {
    let profile_data: Profile =
        serde_json::from_value(profile.clone()).expect("unable to parse profile");

    if let Some(run) = &profile_data.run {
        for app in run {
            persistent_cmd(app, None);
        }
    }

    if let Some(kitty_session) = &profile_data.kitty_session {
        for command in kitty_session {
            let mut args = vec!["--session"];
            args.push(command);
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
