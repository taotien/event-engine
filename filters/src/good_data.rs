use backend::Event;

/// Validates if the given event contains non whitespace for every field
///
/// The function checks if the event's start time, end time, location, description, price, tags, and source are not empty.
/// It also checks if any element in the event's check list and tags is not empty.
///
/// # Arguments
///
/// * `event` - A reference to the `Event` struct to be validated.
///
/// # Returns
/// A boolean value indicating whether the event contains good data or not.
///
/// # Example
///
/// ```
/// use backend::Event;
/// use filters::good_data::validate_good_data;
///
/// let event = Event {
///     start_time: "2022-01-01 10:00".to_string(),
///     end_time: "2022-01-01 12:00".to_string(),
///     location: "New York".to_string(),
///     description: "Lorem ipsum".to_string(),
///     price: "$10".to_string(),
///     tags: vec!["music".to_string(), "concert".to_string()],
///     source: "website".to_string(),
///     check_list: vec!["ticket".to_string(), "ID".to_string()],
/// };
///
/// let is_good_data = validate_good_data(&event);
/// assert_eq!(is_good_data, true);
/// ```
pub(crate) fn validate_good_data(event: &Event) -> bool {
    !event.start_time.trim().is_empty()
        && !event.end_time.trim().is_empty()
        && !event.location.trim().is_empty()
        && !event.description.trim().is_empty()
        && !event.price.trim().is_empty()
        && event.tags.iter().any(|element| !element.trim().is_empty())
        && !event.source.trim().is_empty()
        && event
            .check_list
            .iter()
            .any(|element| !element.trim().is_empty())
}
