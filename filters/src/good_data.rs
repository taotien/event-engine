use backend::Event as backendEvent;

fn validate_good_data(event: backendEvent) -> bool {
    event.start_time.is_some()
        && event.end_time.is_some()
        && event.location.is_some()
        && event.desciption.is_some()
        && event.price.is_some()
        && event.tags.is_some()
        && event.source.is_some()
        && !event.check_list.is_empty()
    // TODO: Tao make the event keys public
}
