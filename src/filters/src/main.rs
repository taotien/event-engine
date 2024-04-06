use futures::executor::block_on;
use google_maps::directions::response::Response;
use google_maps::prelude::*;
use std::env;
use std::time::Duration;
use url::Url;

async fn calculate_direction(
    google_maps_client: &GoogleMapsClient,
    home: String,
    destination: String,
    transit_method: TravelMode,
) -> anyhow::Result<Response> {
    let directions = google_maps_client
        .directions(
            // Origin: Canadian Museum of Nature
            Location::Address(String::from(home)),
            // Destination: Canada Science and Technology Museum
            Location::Address(String::from(destination)),
        )
        .with_travel_mode(transit_method)
        .execute()
        .await?;

    Ok(directions)
}

fn main() {}

fn filter_event_by_travel_time(event: Event, filter: EventFilter) -> bool {
    let google_maps_client =
        GoogleMapsClient::try_new(&env::var("GOOGLE_MAPS_API_KEY").unwrap()).unwrap();

    let direction = block_on(calculate_direction(
        &google_maps_client,
        filter.home_location,
        event.location,
        filter.transit_method,
    ));

    // Debug print the direction variable
    println!("Direction: {:?}", direction);
    return true;
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
    transit_method: TravelMode,
    max_radius_distance: Distance,

    max_radius_time: Duration,
    interests: Vec<String>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_event_by_travel_time() {
        // Create a sample event
        let event = Event {
            name: String::from("Sample Event"),
            start_time: Local::now(),
            end_time: Local::now(),
            location: String::from("Sample Location"),
            desc: String::from("Sample Description"),
            price: 10,
            tags: vec![String::from("tag1"), String::from("tag2")],
            source: Url::parse("https://example.com").unwrap(),
        };

        // Create a sample event filter
        let filter = EventFilter {
            home_location: String::from("Home Location"),
            transit_method: TravelMode::Driving,
            max_radius_distance: Distance::from_kilometers(10.0),
            max_radius_time: Duration::from_secs(3600),
            interests: vec![String::from("interest1"), String::from("interest2")],
        };

        // Call the function and assert the result
        assert_eq!(filter_event_by_travel_time(event, filter), true);
    }
}
