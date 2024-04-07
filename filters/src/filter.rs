use backend::Event as backendEvent;
use chrono::TimeDelta;
use filters::good_data::validate_good_data;
use filters::interests::relevance;
use filters::maps::get_time_and_distance;
use std::process::Command;

use crate::Distance;

fn filter_events(
    events: &Vec<backendEvent>,
    origin_location: Option<String>,
    max_acceptable_travel_time: Option<TimeDelta>,
    max_acceptable_travel_distance: Option<Distance>,
    interests: Option<String>,
    interest_threshold: Option<f64>,
) {
    let filtered_events: Vec<&backendEvent> = events.iter().filter(|event| {
        {
            //initial data validation
            if !validate_good_data(event) {
                return false;
            }
            /*
                       pub fn get_time_and_distance(
                       origin: String,
                       destination: String,
                       transit_method: TravelMode,
                       arrival_time: NaiveDateTime,
                       ) -> anyhow::Result<TimeDistance>
            */
            // Check if max_acceptable_travel_time is given
            if let Some(origin) = origin_location {
                // Check if max_acceptable_travel_time or max_acceptable_travel_distance are some
                if max_acceptable_travel_time.is_some() || max_acceptable_travel_distance.is_some() {
                    let travel_time = get_time_and_distance(origin, event.location,)?;
                    // Perform additional filtering based on travel time or distance
                    // ...
                }
            }

            // Call the relevance function
            let result = relevance(event, filter.interests).await?;
            // Assert that the result is within the expected range
            return result >= interests_data.interest_threshhold;
            // if event.location.is_none() {
            //     return false;
            // }
            // if event.tags.is_none() {
            //     return false;
            // }
            // ...
            // true
        }
        .collect()
    });
}
