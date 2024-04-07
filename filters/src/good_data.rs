use backend::Event as backendEvent;

pub(crate) fn validate_good_data(event: &backendEvent) -> bool {
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
