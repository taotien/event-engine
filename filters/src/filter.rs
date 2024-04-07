use crate::good_data::validate_good_data;
use crate::interests::relevance;
use crate::maps::get_time_and_distance;
use crate::Distance;
use backend::Event;
use chrono::NaiveDateTime;
use chrono::TimeDelta;
use google_maps::directions::TravelMode;

pub async fn filter_events(
    events: &Vec<Event>,
    origin_location: Option<String>,
    travel_mode: Option<TravelMode>,
    max_acceptable_travel_time: Option<TimeDelta>,
    max_acceptable_travel_distance: Option<Distance>,
    interests: Option<String>,
    interest_threshold: Option<f32>,
    max_acceptable_price: Option<u8>,
) -> Vec<Event> {
    let mut filtered_events: Vec<Event> = Vec::new(); // Create an empty vector to store filtered events

    let mut count = 0;
    for event in events {
        if event_filter(
            &event,
            origin_location.clone(),
            travel_mode.clone(),
            max_acceptable_travel_time.clone(),
            max_acceptable_travel_distance.clone(),
            interests.clone(),
            interest_threshold.clone(),
            max_acceptable_price.clone(),
        )
        .await
        {
            count += 1;
            println!("PUSHPUSHPSUH {:?}", event);
            filtered_events.push(event); // Add the event to the vector if it matches the filter
            if count > 10 {
                break;
            }
            filtered_events.push(event.clone()); // Add the event to the vector if it matches the filter
        }
    }
    filtered_events // Return the filtered events vector
}

async fn event_filter(
    event: &Event,
    origin_location: Option<String>,
    travel_mode: Option<TravelMode>,
    max_acceptable_travel_time: Option<TimeDelta>,
    max_acceptable_travel_distance: Option<Distance>,
    interests: Option<String>,
    interest_threshold: Option<f32>,
    max_acceptable_price: Option<u8>,
) -> bool {
    if !validate_good_data(event) {
        println!("invalid data");
        return false;
    }

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
            println!("origin and mode wrong");
            return false;
        }
    }

    if interests.is_some() && interest_threshold.is_some() {
        if !with_interests(&interests.unwrap(), interest_threshold.unwrap(), event).await {
            println!("no interests found");
            return false;
        }
    }

    if max_acceptable_price.is_some() {
        if !with_price(max_acceptable_price.unwrap(), event.price.clone()) {
            println!("no price passed");
            return false;
        }
    }
    true
}

fn with_price(max_acceptable_price: u8, event_price: String) -> bool {
    match event_price.parse::<u8>() {
        Ok(parsed_price) => parsed_price >= max_acceptable_price,
        Err(e) => {
            eprintln!("{}", e);
            false
        }
    }
}

fn parse_date_time(date_time_str: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(date_time_str, "%Y,%m,%d,%H,%M,%S").ok()
}
async fn with_interests(interests: &String, interest_threshold: f32, event: &Event) -> bool {
    let relevance_result = relevance(event, interests).await;
    match relevance_result {
        Ok(good_relevance_value) => return good_relevance_value >= interest_threshold,
        Err(e) => {
            eprintln!("{}", e);
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
                        let mut ret = false;
                        if max_acceptable_travel_distance.is_some() {
                            ret = good_time_and_distance.distance
                                <= *max_acceptable_travel_distance.as_ref().unwrap();
                        }
                        if !ret && max_acceptable_travel_time.is_some() {
                            ret = good_time_and_distance.travel_duration
                                <= *max_acceptable_travel_time.as_ref().unwrap();
                        }
                        return ret;
                    }
                    Err(e) => {
                        eprintln!("{:?}", e);
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
