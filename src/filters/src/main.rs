use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use filters::mapfilter::filter_event_by_travel_time;
use filters::Distance;
use filters::Event;
use filters::EventFilter;
use futures::executor::block_on;
use google_maps::distance_matrix::response::Response;
use google_maps::prelude::*;
use serde_json;
use std::env;
use std::time::Duration;
use url::Url;

#[tokio::main]
async fn main() {
    // Test case setup
    let start_time = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2024, 4, 7).unwrap(),
        NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
    );
    let end_time = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2024, 4, 7).unwrap(),
        NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
    );

    let event = Event {
        name: String::from("Asian Art Museum: Free Admission Day (Every First Sunday)"),
        start_time: chrono::Local.from_local_datetime(&start_time).unwrap(),
        end_time: chrono::Local.from_local_datetime(&end_time).unwrap(),
        location: String::from("200 Larkin Street San Francisco, CA"),
        desc: String::from("Through the power of art, the Asian Art Museum in San Francisco brings the diverse cultures of Asia to life."),
        price: 0,
        tags: vec![String::from("art"), String::from("asian"), String::from("culture")],
        source: Url::parse("https://sf.funcheap.com/asian-art-museum-free-admission-day-every-first-sunday-35/").unwrap(),
    };

    // Create a sample event filter
    let filter = EventFilter {
        home_location: String::from("2345 Golden Gate Ave, San Francisco, CA 94118"),
        transit_method: TravelMode::Transit,
        max_radius_distance: Distance::from_kilometers(10.0),
        max_radius_time: Duration::from_secs(1800),
        interests: vec![String::from("technology"), String::from("art")],
    };

    // Call the function and assert the result
    filter_event_by_travel_time(
        filter.home_location,
        event.location,
        filter.transit_method,
        event.start_time.naive_local(),
    );
}