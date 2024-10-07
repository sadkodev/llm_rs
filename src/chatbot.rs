use crate::{
    prelude::*,
    prompts::read_user_input,
    providers::ollama::{Models, OllamaClient},
    utils::print_chunk_with_style,
};

use crossterm::style::Color;
use futures_util::StreamExt;
use tokio::sync::Mutex;

#[derive(Debug)]
pub enum ChatbotStatus {
    Ready,
    Running,
    Stopped,
}

#[derive(PartialEq)]
pub enum ChatbotAction {
    Continue,
    Stop,
}

#[allow(dead_code)]
pub enum ChatbotMessage {
    User(String),
    Assistant(String),
    System(String),
    Response(String),
}

pub enum ResponseProvider {
    Ollama,
}

pub struct ChatBot {
    status: Mutex<ChatbotStatus>,
    action: Mutex<ChatbotAction>,
    messages: Mutex<Vec<ChatbotMessage>>,
    response_provider: Mutex<ResponseProvider>,
}

impl ChatBot {
    pub fn new() -> Self {
        Self {
            status: Mutex::new(ChatbotStatus::Ready),
            action: Mutex::new(ChatbotAction::Continue),
            messages: Mutex::new(Vec::new()),
            response_provider: Mutex::new(ResponseProvider::Ollama),
        }
    }

    pub async fn run(&self) -> Result<()> {
        let mut messages = self.messages.lock().await;
        let mut status = self.status.lock().await;
        let mut action = self.action.lock().await;
        let response_provider = self.response_provider.lock().await;

        loop {
            match *status {
                ChatbotStatus::Ready => {
                    p_msg("Chatbot is ready!..", TypeMesssage::Success);
                    *status = ChatbotStatus::Running;
                    *action = ChatbotAction::Continue;
                }
                ChatbotStatus::Running => {
                    let input = read_user_input("User: ").await?;
                    stdout().write_all(b"llama:")?;
                    match *response_provider {
                        ResponseProvider::Ollama => {
                            let ollama_client = OllamaClient::new();
                            let mut response_stream = ollama_client
                                .make_call(input.as_str(), Models::default())
                                .await;

                            while let Some(chunk_result) = response_stream.next().await {
                                match chunk_result {
                                    Ok(chunk) => {
                                        print_chunk_with_style(chunk.as_str(), Color::Green)
                                            .await?;

                                        messages.push(ChatbotMessage::Response(chunk));
                                    }
                                    Err(err) => {
                                        p_msg("Error:", TypeMesssage::Error);
                                        p_msg(&err.to_string(), TypeMesssage::Error);
                                    }
                                }
                            }
                            stdout().write_all(b"\n")?;
                            stdout().flush()?;
                        }
                    }

                    if *action == ChatbotAction::Stop {
                        p_msg("Chatbot is stopped!..", TypeMesssage::Error);
                        *status = ChatbotStatus::Stopped;
                        break;
                    }
                }
                ChatbotStatus::Stopped => {
                    p_msg("Chatbot stopped!..", TypeMesssage::Error);
                    break;
                }
            }
        }

        Ok(())
    }
}
