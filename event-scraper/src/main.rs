use event_scraper::{pages, scrape, USFCA_EVENTS};
use reqwest::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();

    for postfix in pages(&client).await? {
        let page = format!("{}{}", USFCA_EVENTS, postfix);
        let events = scrape(&client, page.parse()?).await?;

        println!("{:#?}", events);
    }

    Ok(())
}
