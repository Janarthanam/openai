use open_ai::complete;
use open_ai::open_ai::ask_llm;
use open_ai::open_ai::{CompletionRequest, Message};

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
    
    let completion = ask_llm(&request).await.unwrap();
    println!("{:?}", completion);
}

#[tokio::test]
async fn test_complete() {
    let request = complete!("AI assistant", "Summarize gpt", "gpt-4");
    let completion = ask_llm(&request).await.unwrap();
    println!("{:#?}", completion)
}
