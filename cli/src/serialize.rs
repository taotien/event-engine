use std::process::Command;

use serde::Serialize;

use backend::Event;

#[derive(Serialize, Debug)]
struct JsonArray {
    events: Vec<ICalJson>,
}

#[derive(Serialize, Debug, Clone)]
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

fn cnvt_event_to_ical_fmt(events: Vec<Event>) -> Vec<ICalJson> {
    let mut ical_events: Vec<ICalJson> = Vec::new();
    for event in events {
        let ical_event = ICalJson {
            name: event.name,
            location: event.location,
            start_time: event.start_time,
            end_time: event.end_time,
            check_list: event.check_list,
            description: event.description,
            price: event.price,
            source: event.source,
        };

        ical_events.push(ical_event);
    }

    ical_events
}

pub fn cnvt_event_to_json() -> String {
    let json_arr = cnvt_event_to_ical_fmt(todo!());

    /* Serialize struct to a JSON list string */
    let json_arr = serde_json::to_string(&json_arr).unwrap();

    /* Insert top level key to ensure JSON format is valid */
    format!("{{\"events\": {}}}", json_arr)
}
