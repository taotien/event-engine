use crate::Distance;
use crate::DistanceUnit;
use crate::TimeDistance;
use chrono::TimeDelta;
use google_maps::distance_matrix::response::Response;
use google_maps::prelude::*;
use serde_json::Value;
use std::env;

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
/*
#[cfg(test)]
mod tests {
    use url::Url;

    use super::*;

    #[tokio::test]
    async fn test_get_time_and_distance() {
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

        let result = get_time_and_distance(
            filter.home_location,
            event.location,
            filter.transit_method,
            event.start_time.naive_local(),
        );

        match result {
            Ok(time_distance) => {
                println!("{}", time_distance.travel_duration.to_string());
                println!("{}", time_distance.distance.to_string());
            }
            Err(_) => todo!(),
        }
    }
}
*/
