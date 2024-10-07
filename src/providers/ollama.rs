use crate::prelude::*;
use futures_core::stream::BoxStream;
use futures_util::StreamExt;
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::Stream;

#[derive(Debug)]
pub struct OllamaClient {
    llm: Ollama,
}

pub enum Models {
    Llama3_2_latest(String),
    Llama3_2_7b_chat(String),
}

impl Models {
    pub fn default() -> Self {
        Self::Llama3_2_latest("llama3.2:latest".to_string())
    }

    pub fn get_model(&self) -> String {
        match self {
            Models::Llama3_2_latest(model) => model.to_string(),
            Models::Llama3_2_7b_chat(model) => model.to_string(),
        }
    }
}

impl OllamaClient {
    pub fn new() -> Self {
        Self {
            llm: Ollama::default(),
        }
    }

    pub async fn make_call(
        &self,
        prompt: &str,
        model: Models,
    ) -> BoxStream<'static, Result<String>> {
        let (tx, rx) = mpsc::channel(10);

        let llm = self.llm.clone();
        let prompt = prompt.to_string();
        let model = model.get_model().to_string();

        tokio::spawn(async move {
            let mut stream = llm
                .generate_stream(GenerationRequest::new(model, prompt))
                .await
                .expect("Failed to generate stream");

            while let Some(res) = stream.next().await {
                match res {
                    Ok(chunks) => {
                        for chunk in chunks {
                            if tx.send(Ok(chunk.response)).await.is_err() {
                                break;
                            }
                        }
                    }
                    Err(err) => {
                        let _ = tx.send(Err(err.into())).await;
                        break;
                    }
                }
            }
        });

        ReceiverStream::new(rx).boxed()
    }
}
