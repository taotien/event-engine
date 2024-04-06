use google_maps::prelude::*;
use std::env;
use std::time::Duration;

fn calculate_distance_matrix(
    google_maps_client: &GoogleMapsClient,
    home: String,
    destination: String,
) -> Result<DistanceMatrix, Error> {
    let distance_matrix = google_maps_client
        .distance_matrix(
            vec![Waypoint::Address(String::from(home))],
            vec![Waypoint::PlaceId(String::from(destination))],
        )
        .execute()
        .await?;

    Ok(distance_matrix)
}

fn main() {}

fn filter_event(event: Event, filter: EventFilter) -> bool {
    let google_maps_client = GoogleMapsClient::new(&env::var("GOOGLE_MAPS_API_KEY").unwrap());

    let distance_matrix = calculate_distance_matrix(
        &google_maps_client,
        EventFilter.home_location,
        Event.location,
    )
    .unwrap();
    println!("{:#?}", distance_matrix);
}

struct Event {
    name: String,
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
    location: String,
    desc: String,
    price: u64,
    tags: Vec<String>,
    source: Url,
}

struct EventFilter {
    home_location: String,
    transit_method: TransitMethod,
    max_radius_distance: Distance,

    max_radius_time: Duration,
    interests: Vec<String>,
}

enum TransitMethod {
    Walking,
    Car,
    Transit,
}
struct Distance {
    value: f64,
    unit: DistanceUnit,
}

impl Distance {
    fn from_kilometers(kilometers: f64) -> Distance {
        Distance {
            value: kilometers,
            unit: DistanceUnit::Kilometer,
        }
    }

    fn from_miles(miles: f64) -> Distance {
        Distance {
            value: miles,
            unit: DistanceUnit::Mile,
        }
    }

    fn to_kilometers(&self) -> f64 {
        match self.unit {
            DistanceUnit::Kilometer => self.value,
            DistanceUnit::Mile => self.value * 1.60934,
        }
    }

    fn to_miles(&self) -> f64 {
        match self.unit {
            DistanceUnit::Kilometer => self.value / 1.60934,
            DistanceUnit::Mile => self.value,
        }
    }
}

enum DistanceUnit {
    Kilometer,
    Mile,
}
