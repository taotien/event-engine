use async_openai::error::OpenAIError;
use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    ChatCompletionResponseFormat, ChatCompletionResponseFormatType,
};
use backend::Event;

use async_openai::{types::CreateChatCompletionRequestArgs, Client};
use serde_json::Value;

/// Calculates the relevance of an event based on user preferences.
///
/// Requires enviornment variable of OPENAI_API_KEY to be set
/// Rates the relevency of a an event's name, description, and tags to specified interests on a scale of 0.0 to 1.0 using gpt-3.5-turbo-0125
///
/// # Arguments
///
/// * `event` - The event for which the relevance needs to be calculated.
/// * `user_preferences` - The user's preferences as a string.
///
/// # Returns
///
/// Returns a `Result` containing the relevance score as a `f32` between 0.0 and 1.0 if the calculation is successful, or an `anyhow::Error` if an error occurs.
///
/// # Example
///
/// ```rust
/// use backend::Event;
/// use async_openai::error::OpenAIError;
/// use async_openai::types::{
///     ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
///     ChatCompletionResponseFormat, ChatCompletionResponseFormatType,
/// };
/// use async_openai::{types::CreateChatCompletionRequestArgs, Client};
/// use serde_json::Value;
///
/// async fn example() -> anyhow::Result<()> {
///     let event = Event {
///         // event details
///     };
///
///     let user_preferences = "art and culture".to_string();
///
///     let relevance = relevance(&event, &user_preferences).await?;
///
///     println!("Relevance: {}", relevance);
///
///     Ok(())
/// }
/// ```
pub(crate) async fn relevance(event: &Event, user_preferences: &String) -> anyhow::Result<f32> {
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
        event_to_relevent_string(event),
        user_preferences
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

fn event_to_relevent_string(event: &Event) -> String {
    let name = &event.name;
    let description = &event.description;
    let tags = event.tags.join(", ");

    format!(
        "name: {}, description: {}, tags: [{}]",
        name, description, tags
    )
}
