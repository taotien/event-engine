use scraper::{Html, Selector};

#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq, Hash)]
pub struct UsfEvent {
    pub name: String,
    // start_time: String,
    // end_time: String,
    pub time: String,
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
            let time = event
                .select(&time_selector)
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap()
                .trim()
                .into();
            let location = match event.select(&location_selector).next() {
                Some(l) => l.text().next().map(|s| s.trim().into()),
                None => None,
            };

            UsfEvent {
                name,
                time,
                location,
                source,
            }
        })
        .collect();

    Ok(events)
}
