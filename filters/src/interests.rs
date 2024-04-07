use std::{env, vec};

use crate::{Event, EventFilter};
use async_openai::error::OpenAIError;
use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    ChatCompletionResponseFormat, ChatCompletionResponseFormatType,
};
use async_openai::Chat;
use async_openai::{types::CreateChatCompletionRequestArgs, Client};
use backend::Event as backendEvent;
use chrono::DateTime;
use chrono::Local;
use serde_json::Value;
use url::Url;

//The float returned will be between 0.0 and 1.0 inclusive on both sides
pub async fn relevance(event: Event, user_preferences: String) -> anyhow::Result<f32> {
    // Create a OpenAI client with api key from env var OPENAI_API_KEY and default base url.
    let client = Client::new();
    // Create request using builder pattern
    // Every roequest struct has companion builder struct with same name + Args suffix

    let fmt = ChatCompletionResponseFormat {
        r#type: ChatCompletionResponseFormatType::JsonObject,
    };

    let system: String = "You output json that contains a relevance score from 0.0 to 1.0 based on the user preferences.
    The json looks like this: 
    {
        \"relevance\": 0.75
    }"
    .to_string();

    let message = format!(
        "Please rate the relevance of the event: '{}' based on your preferences of: '{}'.",
        event,
        user_preferences.join(", ")
    );

    //make request, system message instructs json standard. message instructs preferences and event details
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo-0125")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(system)
                .build()
                .map_err(OpenAIError::from)?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(message)
                .build()
                .map_err(OpenAIError::from)?
                .into(),
        ])
        .max_tokens(40_u16)
        .response_format(fmt)
        .build()
        .map_err(OpenAIError::from)?;

    // Call API
    let response = client.chat().create(request).await?;

    // get serde json value
    let json_value: Value = serde_json::from_str(
        &<Option<std::string::String> as Clone>::clone(&response.choices[0].message.content)
            .unwrap(),
    )?;
    // make f64
    let relevance = json_value["relevance"]
        .as_f64()
        .ok_or_else(|| anyhow::anyhow!("Invalid JSON format"))?;

    // clamp the relevence value to 0.0 and 1.0 inclusive
    let relevance = relevance.max(0.0).min(1.0);

    Ok(relevance as f32)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
    use google_maps::directions::TravelMode;

    use crate::Distance;

    use super::*;

    #[tokio::test]
    async fn test_relevance() -> anyhow::Result<()> {
        // Create test event and user preferences
        let start_time = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 4, 7).unwrap(),
            NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        );

        let end_time = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 4, 7).unwrap(),
            NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
        );

        let event = Event {
            name: String::from("Asian Art Museum: Free Admission Day (Every First Sunday)"),
            start_time: chrono::Local.from_local_datetime(&start_time).unwrap(),
            end_time: chrono::Local.from_local_datetime(&end_time).unwrap(),
            location: String::from("200 Larkin Street San Francisco, CA"),
            desc: String::from("Through the power of art, the Asian Art Museum in San Francisco brings the diverse cultures of Asia to life."),
            price: 0,
            tags: vec![String::from("art"), String::from("asian"), String::from("culture")],
            source: Url::parse("https://sf.funcheap.com/asian-art-museum-free-admission-day-every-first-sunday-35/").unwrap(),
        };

        let filter = EventFilter {
            home_location: String::from("2345 Golden Gate Ave, San Francisco, CA 94118"),
            transit_method: TravelMode::Transit,
            max_radius_distance: Distance::from_kilometers(10.0),
            max_radius_time: Duration::from_secs(1800),
            interests: vec![String::from("dog fighting"), String::from("violent crime")],
        };

        // Call the relevance function
        let result = relevance(event, filter.interests).await?;
        println!("Result: {}", result);
        // Assert that the result is within the expected range
        assert!(result >= 0.0 && result <= 1.0);

        Ok(())
    }
}
