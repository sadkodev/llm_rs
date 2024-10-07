mod debug;
use crate::prelude::*;
use tokio::io::{stdout, AsyncWriteExt};

use crossterm::style::{style, Color, Stylize};

#[derive(Debug)]
pub enum TypeMesssage {
    Error,
    Info,
    Debug,
    Success,
}

impl TypeMesssage {
    pub fn icon(&self) -> &'static str {
        match self {
            TypeMesssage::Error => "󱏚 ",
            TypeMesssage::Info => "󰕥 ",
            TypeMesssage::Debug => "󰻌 ",
            TypeMesssage::Success => "󰕥 ",
        }
    }
    pub fn color(&self) -> Color {
        match self {
            TypeMesssage::Error => Color::Red,
            TypeMesssage::Info => Color::Blue,
            TypeMesssage::Debug => Color::Yellow,
            TypeMesssage::Success => Color::Green,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            TypeMesssage::Error => "Error",
            TypeMesssage::Info => "Info",
            TypeMesssage::Debug => "Debug",
            TypeMesssage::Success => "Success",
        }
    }
}

pub fn p_msg(message: &str, type_message: TypeMesssage) {
    println!(
        "{}{} : {}",
        style(type_message.icon()).with(type_message.color()).bold(),
        style(type_message.label())
            .with(type_message.color())
            .bold(),
        style(message).with(type_message.color())
    );
}

pub async fn print_chunk_with_style(chunk: &str, color: Color) -> Result<()> {
    let styled_chunk = style(chunk).with(color).to_string();

    let mut out = stdout();
    out.write_all(styled_chunk.as_bytes()).await?;
    out.flush().await?;

    Ok(())
}
