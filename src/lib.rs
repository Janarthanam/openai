use std::collections::HashMap;
use std::env;

use reqwest::{self, Error, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const CHAT_COMPLETION_URL: &str = "https://api.openai.com/v1/chat/completions";

///structs for json response and requets

///open ai function
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
pub struct Function {
    pub name: String,
    pub description: String,
    pub parameters: Vec<FunctionParameters>,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
pub struct FunctionParameters {
    #[serde(alias = "type")]
    pub t: String,
    pub properties: Value,
}

///OpenAI completion request
/// This is not still not complete
#[derive(Debug,Deserialize, Serialize, Default, PartialEq)]
pub struct CompletionRequest {
    pub model: &'static str,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<Function>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    //no clear doc on this. skippping now.
    //stop: Option<Stop>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequence_penalty: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
pub struct Choice {
    pub index: u16,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
pub struct Usage {
    pub prompt_tokens: u16,
    pub completion_tokens: u16,
    pub total_tokens: u16,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
pub struct Completion {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

///ask llm with a prompt and let it respond
pub async fn ask_llm(completion: &CompletionRequest) -> Result<Completion,Error> {
    let secret =  match env::var("OPENAI_API_KEY") {
        Ok(res) => res,
        Err(_) => panic!("No OpenAI api key")
    };

    let client = reqwest::Client::builder().build()?;

    let res = client
        .post(CHAT_COMPLETION_URL)
        .bearer_auth(secret)
        .json(&completion)
        .send()
        .await;

    match res {
        Ok(response) => match response.status() {
            StatusCode::OK => response.json::<Completion>().await,
            //todo: how do we handle more than one type of error in rust 
            // without introducing a new enum!!?
            _ => panic!("{:#?}", response.text().await?) 
        }
        Err(err) => Err(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_completion_serialization() {
        let completion = r#"
        {
            "model": "gpt-3.5-turbo",
            "messages": [
              {
                "role": "system",
                "content": "You are a helpful assistant."
              },
              {
                "role": "user",
                "content": "Hello!"
              }
            ]
        }"#;

        let completion_request: CompletionRequest = serde_json::from_str(completion).unwrap();
        assert_eq!("gpt-3.5-turbo", completion_request.model);
        assert_eq!("system", completion_request.messages[0].role);
        assert_eq!("Hello!", completion_request.messages[1].content);
    }

    #[test]
    fn test_ser_de() {
        let request = CompletionRequest {
            ..Default::default()
        };
       serde_json::to_string(&request).unwrap();
    }
}
