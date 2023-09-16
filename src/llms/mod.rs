pub mod open_ai;

pub mod llms {
    use async_trait::async_trait;
    use crate::complete;
    use crate::llms::open_ai::ask_llm;
    use crate::llms::open_ai::CompletionRequest;
    use crate::llms::open_ai::Message;

    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error("Reqwest error occurred")]
        RequestError(reqwest::Error),
        #[error("LLM error occurred")]
        LLMError(String)        
    }

    //todo: most llms return multiple types.
    // currently only string type is supported.
    #[derive(Debug)]
    pub enum Response {
        TEXT {res: String},
    }

    #[async_trait]
    pub trait LangModel {
        async fn complete(&self, prompt: String) -> Result<Response,Error>;
    }

    pub struct Gpt {
        pub prompt_template: Option<String>
    }

    #[async_trait]
    impl LangModel for Gpt {
        async fn complete(&self, prompt: String) -> Result<Response,Error> {
            let eventual_prompt = match &self.prompt_template {
                Some(p) => p.to_owned() + &prompt,
                None => prompt
            };

            let request = complete!("Useful AI assistant, factually correct.", eventual_prompt, "gpt-4");
            let response = ask_llm(&request).await;

            match response {
                Ok(completion) => Ok(Response::TEXT { res: completion.choices[0].message.content.clone()}),
                Err(err) => Err(Error::RequestError(err))
            }
        }
    }
}