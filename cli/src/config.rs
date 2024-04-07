use dirs::config_dir;
use serde::Deserialize;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::process::exit;
use toml;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Config {
    main: Main,
    home: Home,
    school: School,
    work: Work,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Main {
    start: String,
    transit: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Home {
    location: String,
    transit: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct School {
    location: String,
    transit: String,
}

#[allow(dead_code)]
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

pub struct Place {
    location: String,
    transit: String,
}

pub fn get_config() -> Place {
    let config = parse_config(get_config_file_path());

    let place: Place = Place {
        location: config.home.location,
        transit: config.home.transit,
    };

    if config.main.start == "home" {
        place.location = config.home.location;
    } else if config.main.start == "school" {
    } else if config.main.start == "work" {
    } else {
    }

    return place;
}
