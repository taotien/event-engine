use std::fs::read_to_string;
use std::path::PathBuf;
use std::process::exit;

use dirs::config_dir;
use serde::Deserialize;

use toml;

#[derive(Deserialize, Debug)]
struct Config {
    main: Main,
    home: Home,
    school: School,
    work: Work,
}

#[derive(Deserialize, Debug)]
struct Main {
    start: String,
    transit: String, // TODO: subject to remove
}

#[derive(Deserialize, Debug)]
struct Home {
    location: String,
    transit: String,
}

#[derive(Deserialize, Debug)]
struct School {
    location: String,
    transit: String,
}

#[derive(Deserialize, Debug)]
struct Work {
    location: String,
    transit: String,
}

fn get_config_file_path() -> PathBuf {
    let mut path = config_dir().unwrap();
    path.push("event-aggregator/config.toml");
    return path;
}

fn parse_config(path: PathBuf) -> Config {
    let file = read_to_string(&path);
    if file.is_err() {
        println!("Config file not found at path: {:?}", path);
        exit(-1);
    }

    let config: Config = toml::from_str(&file.unwrap()).unwrap();
    return config;
}

#[derive(Debug)]
pub struct Place {
    pub location: String,
    pub transit: String,
}

pub fn get_config() -> Place {
    let config = parse_config(get_config_file_path());

    let location: String;
    let transit: String;
    match config.main.start.as_str() {
        "home" => {
            location = config.home.location;
            transit = config.home.transit;
        }
        "school" => {
            location = config.school.location;
            transit = config.school.transit;
        }
        "work" => {
            location = config.work.location;
            transit = config.work.transit;
        }
        _ => {
            println!("Unsupported field");
            exit(-1);
        }
    }

    let place = Place {
        location: location,
        transit: transit,
    };

    return place;
}
