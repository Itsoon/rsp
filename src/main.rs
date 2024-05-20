use indexmap::IndexMap;
use serde_json::Value;
use std::fs;
use std::io::{stdin, stdout, Write};

mod launch_profile;
use launch_profile::launch_profile;

fn main() {
    let path = "./src/profile.json";
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

    println!("Available profile :");

    for (index, value) in available_profile.iter() {
        println!("{}  {}", index, value);
    }

    println!();

    print!("Enter a profile(s) : ");

    let mut input = String::new();

    let _ = stdout().flush();
    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }

    let input = input.trim();

    if let Ok(number) = input.parse::<i32>() {
        if available_profile.contains_key(&number) {
            if let Some(user_value) = available_profile.get(&number) {
                for value in profiles.iter() {
                    if let Some(value) = value.get(user_value) {
                        if let Ok(json_value) = serde_json::to_value(value) {
                            launch_profile(&json_value);
                        }
                    }
                }
            }
        } else {
            println!("Bad character");
        }
    } else {
        println!("Invalid input or failed parser");
    }
}
