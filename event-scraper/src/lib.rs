use chrono::{NaiveDateTime, NaiveTime, Timelike};
use scraper::{Html, Selector};

#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
pub struct UsfEvent {
    pub name: String,
    pub time_start: NaiveDateTime,
    pub time_end: NaiveDateTime,
    pub location: Option<String>,
    // price: String,
    // tags: Vec<String>,
    // description: String,
    // checklist: Vec<String>,
    pub source: String,
}

pub const USFCA_EVENTS_URL: &str = "https://www.usfca.edu/life-usf/events";

pub fn pages(html: &str) -> anyhow::Result<Vec<String>> {
    let document = Html::parse_document(html);

    let pager = Selector::parse("li.pager__item > a").unwrap();
    let mut pages: Vec<_> = document
        .select(&pager)
        .map(|page| page.value().attr("href").unwrap().to_owned())
        .collect();

    pages.sort();

    Ok(pages)
}

pub fn scrape(html: &str) -> anyhow::Result<Vec<UsfEvent>> {
    let document = Html::parse_document(html);

    let listing = Selector::parse(
        "div.lr--main > div.cc--events-listing > div.c--events-listing > div.f--field-components > section.cc--events-listing-component > div.c--events-listing-component > div.text-container",
    )
    .unwrap();
    let events = document.select(&listing);

    let title_selector = Selector::parse("div.f--cta-title > h3 > a").unwrap();
    let time_selector = Selector::parse("div.f--time-string").unwrap();
    let location_selector = Selector::parse("div.event-location").unwrap();

    let events = events
        .map(|event| {
            let name = event.select(&title_selector).next().unwrap();
            let source = name.attr("href").unwrap().into();
            let name = name.inner_html();
            let time_str = event
                .select(&time_selector)
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap()
                .trim();
            let (time_start, time_end) = parse_time(time_str).unwrap();

            let location = match event.select(&location_selector).next() {
                Some(l) => l.text().next().map(|s| s.trim().into()),
                None => None,
            };

            UsfEvent {
                name,
                time_start,
                time_end,
                location,
                source,
            }
        })
        .collect();

    Ok(events)
}

pub fn parse_time(time: &str) -> anyhow::Result<(NaiveDateTime, NaiveDateTime)> {
    // let dt = NaiveDateTime::parse_from_str(time, "%B %e, %Y")
    let split: Vec<&str> = time.split_whitespace().collect();

    match split.len() {
        // date range
        9 => {
            let start = NaiveDateTime::parse_from_str(
                &format!("{} {} {} {}", split[0], split[1], split[5], split[6]),
                "%B %e %Y %l:%M%p",
            )?;
            let end = NaiveDateTime::parse_from_str(
                &format!("{} {} {} {}", split[3], split[4], split[5], split[8]),
                "%B %e, %Y %l:%M%p",
            )?;
            Ok((start, end))
        }
        // time range
        6 => {
            let (start, time) = NaiveDateTime::parse_and_remainder(time, "%B %e, %Y %l:%M%p")?;
            let time = time.strip_prefix(" - ").unwrap();
            let (end_time, _time) = NaiveTime::parse_and_remainder(time, "%l:%M%p")?;

            let end = NaiveDateTime::from(start);
            let end = end.with_hour(end_time.hour()).unwrap();
            let end = end.with_minute(end_time.minute()).unwrap();

            Ok((start, end))
        }
        _ => {
            unreachable!()
        }
    }
}
