pub mod open_ai;

use async_trait::async_trait;
use crate::complete;
use crate::llms::open_ai::chat_completion;
use crate::llms::open_ai::CompletionRequest;
use crate::llms::open_ai::Message;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Reqwest error occurred")]
    RequestError(reqwest::Error),
    #[error("LLM error occurred")]
    LLMError(String),
}

//todo: most llms return multiple types.
// currently only string type is supported.
#[derive(Debug)]
pub enum Response {
    TEXT { res: String },
}

#[async_trait]
pub trait LangModel {
    async fn complete(&self, prompt: String) -> Result<Response, Error>;
}

pub struct Gpt {
    pub system_prompt: String
}

#[async_trait]
impl LangModel for Gpt {
    async fn complete(&self, prompt: String) -> Result<Response, Error> {
        let request = complete!(&self.system_prompt, prompt, "gpt-4");
        let response = chat_completion(&request).await;

        match response {
            Ok(completion) => Ok(Response::TEXT {res: completion.choices[0].message.content.clone()}),
            Err(err) => Err(err)
        }
    }
}