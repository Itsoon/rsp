mod persistent_cmd;
use persistent_cmd::persistent_cmd;

pub fn launch_profile(block_name: &str, options: &Vec<String>) {
    if block_name == "kitty_session" {
        for cmd in options {
            let mut args = vec!["--session"];
            args.push(cmd);
            persistent_cmd("kitty", Some(&args));
        }
    } else if block_name == "kitty_cmd" {
        for cmd in options {
            let mut args = vec!["sh", "-c"];
            let formatted_cmd = format!("{} && exec zsh", cmd);
            args.push(&formatted_cmd);
            persistent_cmd("kitty", Some(&args));
        }
    } else if block_name == "run" {
        for cmd in options {
            persistent_cmd(cmd, None);
        }
    }
}
