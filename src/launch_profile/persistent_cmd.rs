use nix::libc;
use nix::unistd::setsid;
use std::process::{Command, Stdio};

pub fn persistent_cmd(cmd: &str, args: Option<&[&str]>) {
    match unsafe { libc::fork() } {
        -1 => {
            eprintln!("Failed to fork process");
        }
        0 => {
            setsid().expect("Failed to create new session");

            let mut command = Command::new("setsid");

            command.arg(cmd);

            if let Some(args) = args {
                for arg in args.iter() {
                    command.arg(arg);
                }
            }

            command
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("Failed to start process");

            std::process::exit(0);
        }
        _ => {
            println!("Process launched and detached.");
        }
    }
}
