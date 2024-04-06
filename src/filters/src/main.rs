use futures::executor::block_on;
use google_maps::directions::response::Response;
use google_maps::prelude::*;
use serde_json;
use std::env;
use std::time::Duration;
use url::Url;

/// Calculates the direction between two locations using the Google Maps API.
///
/// # Arguments
///
/// * `google_maps_client` - A reference to the GoogleMapsClient instance.
/// * `home` - The home location as a String.
/// * `destination` - The destination location as a String.
/// * `transit_method` - The travel mode as a TravelMode enum variant.
///
/// # Returns
///
/// Returns a Result containing the directions response or an error.

#[tokio::main]
async fn main() {
    // Create a sample event
    let event = Event {
        name: String::from("Sample Event"),
        start_time: Local::now(),
        end_time: Local::now(),
        location: String::from("2130 Fulton St, San Francisco, CA 94117"),
        desc: String::from("Sample Description"),
        price: 10,
        tags: vec![String::from("tag1"), String::from("tag2")],
        source: Url::parse("https://example.com").unwrap(),
    };

    // Create a sample event filter
    let filter = EventFilter {
        home_location: String::from("800 Great Hwy, San Francisco, CA 94121"),
        transit_method: TravelMode::Transit,
        max_radius_distance: Distance::from_kilometers(10.0),
        max_radius_time: Duration::from_secs(3600),
        interests: vec![String::from("interest1"), String::from("interest2")],
    };

    // Call the function and assert the result
    assert_eq!(filter_event_by_travel_time(event, filter), true);
}

async fn calculate_direction(
    google_maps_client: &GoogleMapsClient,
    home: String,
    destination: String,
    transit_method: TravelMode,
) -> anyhow::Result<Response> {
    let directions = google_maps_client
        .directions(
            Location::Address(String::from(home)),
            Location::Address(String::from(destination)),
        )
        .with_travel_mode(transit_method)
        .with_arrival_time(NaiveDate::from_ymd(2024, 4, 6).and_hms(12, 00, 0))
        .execute()
        .await?;

    Ok(directions)
}

fn filter_event_by_travel_time(event: Event, filter: EventFilter) -> bool {
    let google_maps_api_key = &env::var("GOOGLE_MAPS_API_KEY").unwrap();

    let google_maps_client = match GoogleMapsClient::try_new(&google_maps_api_key) {
        Ok(client) => client,
        Err(_) => {
            println!("Failed to create GoogleMapsClient.");
            return false;
        }
    };

    let direction = block_on(calculate_direction(
        &google_maps_client,
        filter.home_location,
        event.location,
        filter.transit_method,
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

/// Represents an event.
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

/// Represents an event filter.
struct EventFilter {
    home_location: String,
    transit_method: TravelMode,
    max_radius_distance: Distance,
    max_radius_time: Duration,
    interests: Vec<String>,
}

/// Represents a distance value with a unit.
struct Distance {
    value: f64,
    unit: DistanceUnit,
}

impl Distance {
    /// Creates a Distance instance from kilometers.
    ///
    /// # Arguments
    ///
    /// * `kilometers` - The distance value in kilometers.
    ///
    /// # Returns
    ///
    /// Returns a Distance instance.
    fn from_kilometers(kilometers: f64) -> Distance {
        Distance {
            value: kilometers,
            unit: DistanceUnit::Kilometer,
        }
    }

    /// Creates a Distance instance from miles.
    ///
    /// # Arguments
    ///
    /// * `miles` - The distance value in miles.
    ///
    /// # Returns
    ///
    /// Returns a Distance instance.
    fn from_miles(miles: f64) -> Distance {
        Distance {
            value: miles,
            unit: DistanceUnit::Mile,
        }
    }

    /// Converts the distance value to kilometers.
    ///
    /// # Returns
    ///
    /// Returns the distance value in kilometers.
    fn to_kilometers(&self) -> f64 {
        match self.unit {
            DistanceUnit::Kilometer => self.value,
            DistanceUnit::Mile => self.value * 1.60934,
        }
    }

    /// Converts the distance value to miles.
    ///
    /// # Returns
    ///
    /// Returns the distance value in miles.
    fn to_miles(&self) -> f64 {
        match self.unit {
            DistanceUnit::Kilometer => self.value / 1.60934,
            DistanceUnit::Mile => self.value,
        }
    }
}

/// Represents a distance unit.
enum DistanceUnit {
    Kilometer,
    Mile,
}
