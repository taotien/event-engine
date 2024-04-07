use crate::good_data::validate_good_data;
use crate::interests::relevance;
use crate::maps::get_time_and_distance;
use crate::Distance;
use backend::Event;
use chrono::NaiveDateTime;
use chrono::TimeDelta;
use google_maps::directions::TravelMode;

/// Filters a list of events based on various criteria.
///
/// This function takes a list of events and applies filters based on the provided parameters.
/// The filtered events are returned as a vector.
/// - Transit filters trigger if origin_location and travel mode is some and if max_acceptable_travel_time or max_acceptable_travel_distance is some.
/// - Interest filter triggers if both interests and interest_threshold are some
/// # Arguments
///
/// * `events` - A reference to a vector of `Event` structs representing the list of events to filter.
/// * `origin_location` - An optional `String` representing the origin location of the user for transit filters.
/// * `travel_mode` - An optional `TravelMode` enum representing the travel mode for transit filters.
/// * `max_acceptable_travel_time` - An optional `TimeDelta` representing the maximum acceptable travel time for transit filters.
/// * `max_acceptable_travel_distance` - An optional `Distance` representing the maximum acceptable travel distance for transit filters.
/// * `interests` - An optional `String` representing the interests for interest filter.
/// * `interest_threshold` - An optional `f32` representing the relevance threshold for interest filter.
/// * `max_acceptable_price` - An optional `u8` representing the maximum acceptable price for price filters.
///
/// # Returns
///
/// A vector of `Event` structs representing the filtered events.
///
/// # Examples
///
/// ```
/// use backend::Event;
/// use google_maps::directions::TravelMode;
/// use chrono::TimeDelta;
/// use crate::filter::filter_events;
///
/// let events: Vec<Event> = vec![/* ... */];
/// let filtered_events = filter_events(
///     &events,
///     Some("New York".to_string()),
///     Some(TravelMode::Transit),
///     Some(TimeDelta::minutes(60)),
///     Some(Distance::miles(10)),
///     Some("music and ramen".to_string()),
///     Some(0.5),
///     Some(50),
/// );
/// ```
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
            println!("DEBUG FILTER {:?}", event);
            filtered_events.push(event.clone()); // Add the event to the vector if it matches the filter
            if count > 10 {
                break;
            }
        }
    }
    filtered_events // Return the filtered events vector
}

/// Filters an event based on various criteria.
///
/// This function takes an events and applies filters based on the provided parameters.
/// The filtered events are returned as a vector.
/// - Transit filters trigger if origin_location and travel mode is some and if max_acceptable_travel_time or max_acceptable_travel_distance is some.
/// - Interest filter triggers if both interests and interest_threshold are some
/// # Arguments
///
/// * `event` - A reference to an `Event` structs representing the event to filter.
/// * `origin_location` - An optional `String` representing the origin location of the user for transit filters.
/// * `travel_mode` - An optional `TravelMode` enum representing the travel mode for transit filters.
/// * `max_acceptable_travel_time` - An optional `TimeDelta` representing the maximum acceptable travel time for transit filters.
/// * `max_acceptable_travel_distance` - An optional `Distance` representing the maximum acceptable travel distance for transit filters.
/// * `interests` - An optional `String` representing the interests for interest filter.
/// * `interest_threshold` - An optional `f32` representing the relevance threshold for interest filter.
/// * `max_acceptable_price` - An optional `u8` representing the maximum acceptable price for price filters.
///
/// # Returns
///
/// A boolean for whether the event met all filter criteria
///
/// # Examples
///
/// ```
/// use backend::Event;
/// use google_maps::directions::TravelMode;
/// use chrono::TimeDelta;
/// use crate::filter::filter_events;
///
/// let event: Event = todo!();
/// let event_passed = event_filter(
///     &event,
///     Some("New York".to_string()),
///     Some(TravelMode::Transit),
///     Some(TimeDelta::minutes(60)),
///     Some(Distance::miles(10)),
///     Some("music and ramen".to_string()),
///     Some(0.5),
///     Some(50),
/// );
/// ```
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
        // println!("invalid data");
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
            // println!("origin and mode wrong");
            return false;
        }
    }

    // if interests.is_some() && interest_threshold.is_some() {
    //     if !with_interests(&interests.unwrap(), interest_threshold.unwrap(), event).await {
    //         println!("no interests found");
    //         return false;
    //     }
    // }

    if max_acceptable_price.is_some() {
        if !with_price(max_acceptable_price.unwrap(), event.price.clone()) {
            // println!("no price passed");
            return false;
        }
    }
    true
}

/// Checks whether a price with within a certain maximum price.
///
/// This function takes a price as a string of a u8 that represents USD and a u8 that represents USD
/// It converts the string to u8 and checks whether the price is under the max acceptable price
///
/// # Arguments
///
/// * `max_acceptable_price` - A u8 int that represents the maximum acceptable price an event price can be in USD
/// * `event_price` - A String to be converted into a u8 in that represents the price of an event in USD
///
/// # Returns
///
/// A boolean for whether the event_price was within the max_acceptable_price
///
/// # Examples
///
/// ```
///     let max_acceptable_price = 50;
///     let event_price = "30".to_string();

///     if with_price(max_acceptable_price, event_price.clone()) {
///         println!("Event price is within the acceptable range");
///     } else {
///         println!("Event price is too high");
///     }
/// ```
fn with_price(max_acceptable_price: u8, event_price: String) -> bool {
    match event_price.parse::<u8>() {
        Ok(parsed_price) => parsed_price <= max_acceptable_price,
        Err(e) => {
            eprintln!("{}", e);
            false
        }
    }
}

/// Parses a date and time string into a `NaiveDateTime` object.
///
/// This function takes a date and time string in the format "%Y,%m,%d,%H,%M,%S" and attempts to parse it into a `NaiveDateTime` object.
///
/// # Arguments
///
/// * `date_time_str` - A string representing the date and time in the format "%Y,%m,%d,%H,%M,%S".
///
/// # Returns
///
/// An `Option` containing the parsed `NaiveDateTime` object if successful, or `None` if parsing fails.
///
/// # Examples
///
/// ```
/// let date_time_str = "2022,01,01,12,00,00";
/// let parsed_date_time = parse_date_time(date_time_str);
/// match parsed_date_time {
///     Some(dt) => println!("Parsed date and time: {}", dt),
///     None => println!("Failed to parse date and time"),
/// }
/// ```
fn parse_date_time(date_time_str: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(date_time_str, "%Y,%m,%d,%H,%M,%S").ok()
}

/// Checks if an event is relevent to specified interestsa
///
/// Requires enviornment variable of OPENAI_API_KEY to be set
/// Rates the relevency of a an event's name, description, and tags to specified interests on a scale of 0.0 to 1.0 using openai chat
/// Filters out events whose relevency is less than the interest_threshold thresh
///
/// # Arguments
///
/// * `event` - A reference to an `Event` struct representing the event to filter.
/// * `interests` - A `String` representing interests of a user
/// * `interest_threshold` - A `f32` between 0.0 and 1.0 that is the minimum relevancy score for a event to be relevent
/// # Returns
///
/// A boolean indicating whether the event is relevent to specified interests
///
/// # Examples
///
/// ```
/// let interests = "music, sports".to_string();
/// let interest_threshold = 0.7;
/// let event = Event {
///     name: "Concert".to_string(),
///     description: "A live music performance".to_string(),
///     tags: vec!["music".to_string(), "rock".to_string()],
/// };
///
/// let is_relevant = with_interests(&interests, interest_threshold, &event).await;
///
/// if is_relevant {
///     println!("Event is relevant to specified interests");
/// } else {
///     println!("Event is not relevant to specified interests");
/// }
/// ```
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
/// Checks if an event meets the transit filter criteria.
///
/// Requires enviornment variable of GOOGLE_MAPS_API_KEY to be set
/// This function only works if `max_acceptable_travel_time` or `max_acceptable_travel_distance` are `Some`
/// max_acceptable_travel_time and max_acceptable_travel_distance will use the provided travel mode to find the duration &or distance of travel using the specified travel mode when set to arrive at the start of the event
///
/// # Arguments
///
/// * `event` - A reference to an `Event` struct representing the event to filter.
/// * `origin_location` - A `String` representing the origin location of the user for transit filters.
/// * `travel_mode` - A `TravelMode` enum representing the travel mode for transit filters.
/// * `max_acceptable_travel_time` - An optional `TimeDelta` representing the maximum acceptable travel time for transit filters.
/// * `max_acceptable_travel_distance` - An optional `Distance` representing the maximum acceptable travel distance for transit filters.
///
/// # Returns
///
/// A boolean indicating whether the event meets the transit filter criteria.
///
/// # Examples
///
/// ```
/// use backend::Event;
/// use google_maps::directions::TravelMode;
/// use chrono::TimeDelta;
/// use crate::filter::filter_events;
///
/// let event: Event = todo!();
/// let origin_location = Some("New York".to_string());
/// let travel_mode = Some(TravelMode::Transit);
/// let max_acceptable_travel_time = Some(TimeDelta::minutes(60));
/// let max_acceptable_travel_distance = Some(Distance::miles(10));
///
/// let meets_criteria = with_transit(
///     &event,
///     origin_location,
///     travel_mode,
///     max_acceptable_travel_time,
///     max_acceptable_travel_distance,
/// );
/// if meets_criteria {
///     println!("Event meets transit filter criteria");
/// } else {
///     println!("Event does not meet transit filter criteria");
/// }
/// ```
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
