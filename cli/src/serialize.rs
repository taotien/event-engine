use std::process::Command;

use serde::Serialize;

#[derive(Serialize, Debug)]
struct ICalJson {
    name: String,
    location: String,
    start_time: String,
    end_time: String,
    check_list: Vec<String>,
    description: String,
    price: String,
    source: String,
}

pub fn print_event_as_json() -> String {
    // TODO: remove test dummy data
    let ical_json = ICalJson {
        name: "Yiyu's coffee shop".to_owned(),
        location: "123 Foobar Ave".to_owned(),
        start_time: "2024,04,06,19,30,00".to_owned(),
        end_time: "2024,04,06,21,30,00".to_owned(),
        check_list: vec!["camera".to_owned(), "money".to_owned()],
        description: "This is a description".to_owned(),
        price: "100".to_owned(),
        source: "https://www.example.org/".to_owned(),
    };

    /* Serialize struct to a JSON string */
    serde_json::to_string(&ical_json).unwrap()
}
