mod persistent_cmd;
use persistent_cmd::persistent_cmd;

pub fn launch_profile(block_name: &str, workspace_name: &str, options: &Vec<String>) {
    if block_name == "kitty_session" {
        for raw_cmd in options {
            let cmd = format!("kitty --session {}", raw_cmd);
            let params = format!("[workspace {} silent] {}", workspace_name, cmd);
            let args = vec!["dispatch", "exec", &params];
            persistent_cmd("hyprctl", Some(&args));
        }
    } else if block_name == "kitty_cmd" {
        for raw_cmd in options {
            let cmd = format!("kitty sh -c '{}; exec zsh'", raw_cmd);
            let params = format!("[workspace {} silent] {}", workspace_name, cmd);
            let args = vec!["dispatch", "exec", &params];
            persistent_cmd("hyprctl", Some(&args));
        }
    } else if block_name == "run" {
        for cmd in options {
            let params = format!("[workspace {} silent] {}", workspace_name, cmd);
            let args = vec!["dispatch", "exec", &params];
            persistent_cmd("hyprctl", Some(&args));
        }
    }
}
