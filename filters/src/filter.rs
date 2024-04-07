use crate::good_data::validate_good_data;
use crate::interests::relevance;
use crate::maps::get_time_and_distance;
use backend::Event as backendEvent;
use chrono::TimeDelta;
use futures::executor::block_on;
use google_maps::directions::TravelMode;
use std::process::Command;

use crate::Distance;
use chrono::NaiveDateTime;

async fn filter_events(
    events: &Vec<backendEvent>,
    origin_location: Option<String>,
    travel_mode: Option<TravelMode>,
    max_acceptable_travel_time: Option<TimeDelta>,
    max_acceptable_travel_distance: Option<Distance>,
    interests: Option<String>,
    interest_threshold: Option<f32>,
    max_price: Option<u8>,
) -> Vec<&backendEvent> {
    let filtered_events: Vec<&backendEvent> = events
        .iter()
        .filter(|event| {
            //initial data validation
            if !validate_good_data(event) {
                return false;
            }
            // Check if transit is in parameters
            if origin_location.is_some()
                && travel_mode.is_some()
                && (max_acceptable_travel_time.is_some()
                    || max_acceptable_travel_distance.is_some())
            if let (
                Some(origin_location),
                Some(travel_mode),
                Some(max_acceptable_travel_time),
                Some(max_acceptable_travel_distance),
            ) = (
                origin_location.as_ref(),
                travel_mode.as_ref(),
                max_acceptable_travel_time.as_ref(),
                max_acceptable_travel_distance.as_ref(),
            ) {
                if with_transit(
            if origin_location.is_some() && travel_mode.is_some() {
                if !with_transit(
                    event,
                    origin_location.unwrap(),
                    travel_mode.unwrap(),
                    max_acceptable_travel_time,
                    max_acceptable_travel_distance,
                )
                .await
                {
                    return false;
                }
            }

            // check if interests align
            if let (Some(interests), Some(interest_threshold)) =
                (interests.as_ref(), interest_threshold)
            {
                let time_and_distance = block_on(get_time_and_distance(
                    origin_location,
                    event.location,
                    travel_mode,
                    parse_date_time(event.start_time),
                ));

                if max_acceptable_travel_distance.is_some() {
                    if time_and_distance.distance > max_acceptable_travel_distance.unwrap() {
                        return false;
                    }
                }
                if max_acceptable_travel_time.is_some() {
                    if time_and_distance.travel_duration > max_acceptable_travel_time.unwrap() {
                        return false;
                    }
                }
                if with_interests(interests, interest_threshold, event).await {
                    return false;
                }
            }
            //check if interests align
            if interests.is_some() && interest_threshold.is_some() {
                if let Ok(relevance_value) = block_on(relevance(event, interests.unwrap())) {
                    if relevance_value < interest_threshold.unwrap() {
                        return false;
                    }
                }
            }
            true
        })
        .collect();

            true
        })
        .collect()
        .await;

    filtered_events
    // TODO: Handle filtered_events
}

fn parse_date_time(date_time_str: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(date_time_str, "%Y,%m,%d,%H,%M,%S").ok()
}
async fn with_interests(interests: &String, interest_threshold: f32, event: &backendEvent) -> bool {
    let relevance_result = relevance(event, interests).await;
    match relevance_result {
        Ok(good_relevance_value) => return good_relevance_value >= interest_threshold,
        Err(_) => {
            return false;
        }
    }
}
async fn with_transit(
    event: &backendEvent,
    origin_location: String,
    travel_mode: TravelMode,
    max_acceptable_travel_time: Option<TimeDelta>,
    max_acceptable_travel_distance: Option<Distance>,
) -> bool {
    if max_acceptable_travel_time.is_some() || max_acceptable_travel_distance.is_some() {
        let parsed_date_time = parse_date_time(&event.start_time);
        match parsed_date_time {
            Some(good_parsed_date_time) => {
                let time_and_distance = get_time_and_distance(
                    origin_location,
                    &event.location,
                    travel_mode,
                    good_parsed_date_time,
                )
                .await;
                match time_and_distance {
                    Ok(good_time_and_distance) => {
                        if max_acceptable_travel_distance.is_some() {
                            return good_time_and_distance.distance
                                <= *max_acceptable_travel_distance.as_ref().unwrap();
                        }
                        if max_acceptable_travel_time.is_some() {
                            return good_time_and_distance.travel_duration
                                <= *max_acceptable_travel_time.as_ref().unwrap();
                        }
                        return false;
                    }
                    Err(_) => {
                        return false;
                    }
                };
            }
            None => return false,
        };
    } else {
        return false;
    }
}
