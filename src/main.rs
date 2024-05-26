use colored::*;
use indexmap::IndexMap;
use std::env;
use std::fs;
use std::process::exit;
mod parser;
use parser::file_parser;
mod launch_profile;
mod settings;

fn main() {
    let args: Vec<String> = env::args().collect();

    let arg = check_args_validity(args);

    let home_dir = match std::env::var("HOME") {
        Ok(val) => val,
        Err(_) => panic!("HOME environment variable is not set"),
    };

    let available_profiles = get_available_profiles(home_dir);

    if arg == "-l" || arg == "-ls" || arg == "l" || arg == "ls" || arg == "list" {
        list_available_profiles(available_profiles);
        exit(0);
    } else if arg == "h" || arg == "-h" || arg == "-help" || arg == "--help" || arg == "help" {
        display_help();
        exit(0);
    } else if arg == "-v" || arg == "-V" || arg == "--version" {
        display_version();
        exit(0);
    } else if available_profiles.values().any(|value| *value == arg) {
        let file_name = format_args!("{}.toml", arg).to_string();
        file_parser(file_name);
    } else if let Ok(arg_i32) = arg.parse::<i32>() {
        if available_profiles.contains_key(&arg_i32) {
            let file_name = format_args!("{}.toml", available_profiles[&arg_i32]).to_string();
            file_parser(file_name);
        } else {
            print!("{}", "Number out of range\n".bright_red());
            display_help();
        }
    } else {
        println!("{}", "Bad character".bright_red());
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

fn list_available_profiles(available_profiles: IndexMap<i32, String>) {
    for (index, value) in available_profiles.iter() {
        println!("{}  {}", index, value);
    }
}

fn get_available_profiles(home_dir: String) -> indexmap::IndexMap<i32, String> {
    let mut available_profiles = IndexMap::new();

    let profiles_path = format!("{}/.config/rsp/profiles/", home_dir);

    let entries = match fs::read_dir(&profiles_path) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Error reading directory: {}", err);
            exit(1);
        }
    };
    for (index, entry) in entries.enumerate() {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("Error reading directory entry: {}", err);
                continue;
            }
        };

        let file_name = match entry.file_name().into_string() {
            Ok(name) => name,
            Err(os_string) => os_string.to_string_lossy().into_owned(),
        };

        if file_name.ends_with(".toml") {
            let file_name_without_extension = file_name.trim_end_matches(".toml").to_string();
            available_profiles.insert(index as i32, file_name_without_extension);
        }
    }
    available_profiles
}

fn display_version() {
    let version = env!("CARGO_PKG_VERSION");
    println!("Version: {}", version);
}

fn display_help() {
    let help: &str = "\
Usage: rsp [COMMAND]

Commands:
  l, ls, list           Print available profiles
  <profile>          Processing profile
  h, help            Print this message or the help of the given subcommand(s)

Options:
  -l, -ls            Print available profiles
  -h, --help         Print help
  -v, -V, --version  Print version";
    println!("{}", help);
}
