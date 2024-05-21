use event_scraper::UsfEvent;
use icalendar::{Calendar, Component, EventLike};

pub struct ParsedEvent(UsfEvent);

impl ParsedEvent {
    pub fn new(e: UsfEvent) -> Self {
        Self(e)
    }
}

impl From<ParsedEvent> for icalendar::Event {
    fn from(value: ParsedEvent) -> Self {
        let UsfEvent {
            name,
            time_start,
            time_end,
            location,
            source,
        } = value.0;
        let mut event = Self::new();

        event
            .summary(&name)
            .starts(time_start)
            .ends(time_end)
            .url(&source);

        if let Some(loc) = location {
            event.location(&loc);
        }

        event.done()
    }
}

pub fn export(events: Vec<impl Into<icalendar::Event>>) -> String {
    // let event: icalendar::Event = event.into();

    let mut calendar = Calendar::new();

    for event in events {
        calendar.push(event.into());
    }

    calendar.to_string()
}
