use indexmap::IndexMap;
use serde_json::Value;
use std::env;
use std::fs;
use std::process::exit;

mod launch_profile;
use launch_profile::launch_profile;

fn main() {
    let args: Vec<String> = env::args().collect();

    let arg = check_args_validity(args);

    let home_dir = match std::env::var("HOME") {
        Ok(val) => val,
        Err(_) => panic!("HOME environment variable is not set"),
    };

    let path = format!("{}/.config/rsp/profile.json", home_dir);
    let data = fs::read_to_string(path).expect("Unable to read file");
    let profiles: Vec<serde_json::Value> = serde_json::from_str(&data).expect("Unable to parse");

    let mut available_profile = IndexMap::new();

    for (index, profile) in profiles.iter().enumerate() {
        if let Value::Object(map) = profile {
            for (key, _value) in map {
                available_profile.insert(index as i32, key.to_string());
            }
        }
    }

    if arg == "l" || arg == "ls" || arg == "list" {
        list_available_profile(available_profile);
        exit(0);
    } else if arg == "-h" || arg == "-help" || arg == "--help" || arg == "help" {
        display_help();
        exit(0);
    } else if let Ok(arg) = arg.parse::<i32>() {
        if available_profile.contains_key(&arg) {
            if let Some(user_value) = available_profile.get(&arg) {
                for value in profiles.iter() {
                    if let Some(value) = value.get(user_value) {
                        if let Ok(json_value) = serde_json::to_value(value) {
                            println!("{}", &json_value);
                            launch_profile(&json_value);
                        }
                    }
                }
            }
        } else {
            println!("Bad character");
            display_help();
            exit(0);
        }
    } else {
        println!("Invalid input or failed parser");
        display_help();
        exit(0);
    }
}

fn check_args_validity(args: Vec<String>) -> String {
    if args.len() == 2 {
        args[1].clone()
    } else {
        display_help();
        exit(0);
    }
}

fn list_available_profile(available_profile: IndexMap<i32, String>) {
    for (index, value) in available_profile.iter() {
        println!("{}  {}", index, value);
    }
}

fn display_help() {
    let help: &str = "
Usage: starter-profile [COMMAND]

Commands:
  l, ls, list   Print available profiles
  <profile>  Processing profile
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version";
    println!("{}", help);
}
