use crate::Distance;
use crate::Event;
use crate::EventFilter;
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use futures::executor::block_on;
use google_maps::directions::response::Response;
use google_maps::prelude::*;
use serde_json;
use std::env;
use std::time::Duration;
use url::Url;

fn filter_event_by_travel_time(
    origin: String,
    destination: String,
    transit_method: TravelMode,
    arrival_time: NaiveDateTime,
) -> bool {
    print!("getting api key");
    let google_maps_api_key =
        &env::var("GOOGLE_MAPS_API_KEY").expect("There's Not Google Maps API Key");
    print!("got the key");
    let google_maps_client = match GoogleMapsClient::try_new(&google_maps_api_key) {
        Ok(client) => client,
        Err(_) => {
            println!("Failed to create GoogleMapsClient.");
            return false;
        }
    };

    let direction = block_on(calculate_direction(
        &google_maps_client,
        origin.clone(),
        destination.clone(),
        transit_method.clone(),
        arrival_time.clone(),
    ));

    match direction {
        Ok(direction) => {
            if let Ok(json) = serde_json::to_string_pretty(&direction) {
                // Print the formatted JSON
                println!("Direction: {}", json);
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    return true;
}

async fn calculate_direction(
    google_maps_client: &GoogleMapsClient,
    home: String,
    destination: String,
    transit_method: TravelMode,
    arrival_time: NaiveDateTime,
) -> anyhow::Result<Response> {
    let directions = google_maps_client
        .directions(
            Location::Address(String::from(home)),
            Location::Address(String::from(destination)),
        )
        .with_travel_mode(transit_method)
        .with_arrival_time(arrival_time)
        .execute()
        .await?;

    Ok(directions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_filter_event_by_travel_time() {
        // Create a sample event
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
        assert_eq!(
            filter_event_by_travel_time(
                filter.home_location,
                event.location,
                filter.transit_method,
                event.start_time.naive_local()
            ),
            true
        );
    }
}
