use crate::prelude::*;
mod chatbot;
mod error;
mod prelude;
mod prompts;
mod providers;
mod utils;

use crate::chatbot::ChatBot;

#[tokio::main]
async fn main() -> Result<()> {
    let chatbot = ChatBot::new();
    chatbot.run().await?;
    Ok(())
}
