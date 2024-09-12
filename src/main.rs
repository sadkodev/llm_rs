use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use ollama_rs::{
    generation::{completion::request::GenerationRequest, options::GenerationOptions},
    Ollama,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ollama = Ollama::default();
    let model = "llama3.1:latest".to_string();
    let prompt = get_prompt()?;
    let options = GenerationOptions::default()
        .temperature(0.1)
        .top_p(0.5)
        .top_k(80)
        .num_thread(10)
        .num_gpu(1);
    let res = ollama
        .generate(GenerationRequest::new(model, prompt).options(options))
        .await?;
    generate_files(res.response).await?;
    Ok(())
}

fn get_prompt() -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open("src/data/prompts.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

async fn generate_files(content: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("src/data/words.json")?;

    println!("{:?}", file);
    file.write_all(content.as_bytes())?;
    println!(" successs: {:?}", file);
    Ok(())
}

fn fetch_words() -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open("src/data/words.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
