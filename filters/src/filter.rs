use crate::good_data::validate_good_data;
use crate::interests::relevance;
use crate::maps::get_time_and_distance;
use crate::Distance;
use backend::Event as backendEvent;
use chrono::NaiveDateTime;
use chrono::TimeDelta;
use futures::executor::block_on;
use futures::stream::StreamExt;
use google_maps::directions::TravelMode;

use backend::Event;

async fn filter_events(
    events: &Vec<Event>,
    origin_location: Option<String>,
    travel_mode: Option<TravelMode>,
    max_acceptable_travel_time: Option<TimeDelta>,
    max_acceptable_travel_distance: Option<Distance>,
    interests: Option<String>,
    interest_threshold: Option<f32>,
    max_price: Option<u8>,
    max_acceptable_price: Option<u8>,
) -> Vec<&Event> {
    todo!();
}

fn parse_date_time(date_time_str: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(date_time_str, "%Y,%m,%d,%H,%M,%S").ok()
}
async fn with_interests(interests: &String, interest_threshold: f32, event: &Event) -> bool {
    let relevance_result = relevance(event, interests).await;
    match relevance_result {
        Ok(good_relevance_value) => return good_relevance_value >= interest_threshold,
        Err(_) => {
            return false;
        }
    }
}
async fn with_transit(
    event: &Event,
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
