use crate::Distance;
use crate::DistanceUnit;
use crate::TimeDistance;
use chrono::TimeDelta;
use google_maps::distance_matrix::response::Response;
use google_maps::prelude::*;
use serde_json::Value;
use std::env;

/// Gets the time and distance to get to an event at it's start time using the specified transit mode
///
/// Requires enviornment variable of GOOGLE_MAPS_API_KEY to be set
/// This function only works if `max_acceptable_travel_time` or `max_acceptable_travel_distance` are `Some`
/// max_acceptable_travel_time and max_acceptable_travel_distance will use the provided travel mode to find the duration &or distance of travel using the specified travel mode when set to arrive at the start of the event
///
/// # Arguments
///
/// * `origin` - A `String` representing the origin location of the user as an standard street address
/// * `destination` ~ A `String` representing the destination location of the event as an standard street address
/// * `travel_mode` - A `TravelMode` enum representing the travel mode
/// * `arrival_time` - A `NaiveDateTime`  representing the time the user should arrive that the destination
///
/// # Returns
///
/// A Result that has a TimeDistance containing the time it takes to get there and the distance of the route or an Error if something goes wrong
///
/// # Example
///
/// ```
///#[tokio::main]
///async fn main() {
///    let origin = String::from("123 Main St, City");
///    let destination = String::from("456 Park Ave, City");
///    let transit_method = TravelMode::Driving;
///    let arrival_time = NaiveDateTime::from_timestamp(1634567890, 0);
///
///    match get_time_and_distance(origin, &destination, transit_method, arrival_time).await {
///        Ok(time_distance) => {
///            println!("Travel Duration: {:?}", time_distance.travel_duration);
///            println!("Distance: {:?}", time_distance.distance);
///        }
///        Err(err) => {
///            eprintln!("Error: {}", err);
///        }
///    }
///}
/// ```
pub(crate) async fn get_time_and_distance(
    origin: String,
    destination: &String,
    transit_method: TravelMode,
    arrival_time: NaiveDateTime,
) -> anyhow::Result<TimeDistance> {
    let google_maps_api_key =
        &env::var("GOOGLE_MAPS_API_KEY").expect("There's Not Google Maps API Key");
    let google_maps_client = match GoogleMapsClient::try_new(&google_maps_api_key) {
        Ok(client) => client,
        Err(_) => {
            return Err(anyhow::Error::msg("Failed to create GoogleMapsClient."));
        }
    };

    let direction = get_distance_matrix(
        &google_maps_client,
        origin.clone(),
        destination.clone(),
        transit_method.clone(),
        arrival_time.clone(),
    );

    match direction.await {
        Ok(direction) => {
            if let Ok(parsed_direction) = parse_json_to_time_and_distance(&direction) {
                return Ok(parsed_direction);
            } else {
                return Err(anyhow::Error::msg("Failed to parse direction"));
            }
        }
        Err(e) => {
            return Err(anyhow::Error::msg(e));
        }
    }
}

fn parse_json_to_time_and_distance(
    responce: &DistanceMatrixResponse,
) -> anyhow::Result<TimeDistance> {
    let parsed_json: Value = serde_json::from_str(&serde_json::to_string(&responce)?)?;
    // print!(
    //     "{}",
    //     parsed_json["rows"][0]["elements"][0]["duration"]["value"]
    // );
    let elements = &parsed_json["rows"][0]["elements"][0];
    let duration = &elements["duration"]["value"];
    let distance = &elements["distance"]["value"];

    let distance_uh = Distance {
        value: distance.to_string().parse::<f64>()? / 1000.0,
        unit: DistanceUnit::Kilometer,
    };

    let time_distance = TimeDistance {
        travel_duration: TimeDelta::seconds(duration.as_u64().unwrap_or(0) as i64),
        distance: distance_uh,
    };

    println!("Travel Duration: {:?}", time_distance.travel_duration);
    println!("Distance: {:?}", time_distance.distance);

    Ok(time_distance)
}

async fn get_distance_matrix(
    google_maps_client: &GoogleMapsClient,
    home: String,
    destination: String,
    transit_method: TravelMode,
    arrival_time: NaiveDateTime,
) -> anyhow::Result<Response> {
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
