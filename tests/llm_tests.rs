use open_ai::llms::Gpt;
use open_ai::llms::LangModel;
use open_ai::llms::Response;
use open_ai::llms::open_ai::chat_completion;
use open_ai::llms::open_ai::{CompletionRequest, Message};
use open_ai::complete;

#[tokio::test]
async fn test_ask_llm() {
    let request = CompletionRequest {
        model : "gpt-4",
        messages: vec![Message{
            role: String::from("system"),
            content: String::from("You are an AI system, of course")
        },
        Message{
            role: String::from("user"),
            content: String::from("Hello!")
        }],
        ..Default::default()
    };
    
    let completion = chat_completion(&request).await.unwrap();
    println!("{:?}", completion);
}

#[tokio::test]
async fn test_macro() {
    let request = complete!("AI assistant", "Summarize gpt", "gpt-4");
    let completion = chat_completion(&request).await.unwrap();
    println!("{:#?}", completion)
}

#[tokio::test]
async fn test_gpt_complete() {
    let gpt = Gpt {
        prompt_template: None,
    };
    
    let result = gpt.complete(String::from("Summarize this great library")).await;
    assert!(result.is_ok());
    let Response::TEXT{res} = result.unwrap();
    assert!(!res.is_empty());   
}
