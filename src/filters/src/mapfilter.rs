use crate::Distance;
use crate::Event;
use crate::EventFilter;
use chrono::TimeDelta;
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use futures::executor::block_on;
use google_maps::distance_matrix::response::Response;
use google_maps::prelude::*;
use serde_json;
use serde_json::Value;
use std::env;
use std::time::Duration;
use url::Url;

pub fn filter_event_by_travel_time(
    origin: String,
    destination: String,
    transit_method: TravelMode,
    arrival_time: NaiveDateTime,
    max_acceptable_travel_time: TimeDelta,
) -> bool {
    let google_maps_api_key =
        &env::var("GOOGLE_MAPS_API_KEY").expect("There's Not Google Maps API Key");
    let google_maps_client = match GoogleMapsClient::try_new(&google_maps_api_key) {
        Ok(client) => client,
        Err(_) => {
            println!("Failed to create GoogleMapsClient.");
            return false;
        }
    };

    let direction = block_on(get_distance_matrix(
        &google_maps_client,
        origin.clone(),
        destination.clone(),
        transit_method.clone(),
        arrival_time.clone(),
    ));

    match direction {
        Ok(direction) => {
            if let Ok(json) = serde_json::to_string_pretty(&direction) {
                println!("{}", json);
                return true;
            } else {
                // Handle the error case here
                return false;
            }
        }
        Err(e) => {
            println!("Error: {}", e);
            return false;
        }
    }
}

pub async fn get_distance_matrix(
    google_maps_client: &GoogleMapsClient,
    home: String,
    destination: String,
    transit_method: TravelMode,
    arrival_time: NaiveDateTime,
) -> anyhow::Result<Response> {
    println!("Home: {}", home);
    println!("Destination: {}", destination);
    println!("Transit method: {:?}", transit_method);
    println!("Arrival time: {:?}", arrival_time);
    match google_maps_client
        .distance_matrix(
            vec![Waypoint::Address(String::from(home))],
            vec![Waypoint::Address(String::from(destination))],
        )
        .with_travel_mode(transit_method)
        .with_arrival_time(arrival_time)
        .execute()
        .await
    {
        Ok(response) => return Ok(response),
        Err(e) => return Err(anyhow::Error::msg(e.to_string())),
    }
}
