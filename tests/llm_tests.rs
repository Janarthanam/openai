use open_ai::{self, CompletionRequest, Message, complete};

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

#[tokio::test]
async fn complete() {
    let request = complete!("AI assistant", "Summarize gpt", "gpt-4");
    let completion = open_ai::ask_llm(&request).await.unwrap();
    println!("{:#?}", completion)
}
