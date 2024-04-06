use std::{env, vec};

use crate::Event;
use async_openai::error::OpenAIError;
use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    ChatCompletionResponseFormat, ChatCompletionResponseFormatType,
};
use async_openai::Chat;
use async_openai::{types::CreateChatCompletionRequestArgs, Client};
use chrono::DateTime;
use chrono::Local;
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

    let system: String = 
    "You output json that contains a relevance score from 0.0 to 1.0 based on the user preferences.
    The json looks like this: 
    {
        \"relevance\": 0.75
    }"
    .to_string();

    let message = format!(
        "Please rate the relevance of the event '{}' based on your preferences '{}'.",
        event, user_preferences
    );

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

println!("\nResponse:\n");
    for choice in response.choices {
        println!(
            "{}: Role: {}  Content: {:?}",
            choice.index, choice.message.role, choice.message.content
        );
    }
    return Ok(0.0);
}
