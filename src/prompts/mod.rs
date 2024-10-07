use crate::{error::ErrorLLM, prelude::*};

pub async fn read_user_input(prompt: &str) -> Result<String> {
    stdout().write_all(prompt.as_bytes())?;
    stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();

    if input.is_empty() {
        p_msg("Input is empty!..", TypeMesssage::Error);
        return Err(ErrorLLM::EmptyInput);
    }

    Ok(input)
}
