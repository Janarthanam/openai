use open_ai::{self, CompletionRequest, Message};

#[tokio::test]
async fn ask_llm() {
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
    
    let completion = open_ai::ask_llm(&request).await.unwrap();
    println!("{:?}", completion);
}