use std::time::Duration;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::timeout,
};

use crate::prelude::ServerResult;

pub async fn send_message(stream: &mut TcpStream, message: &str) -> ServerResult<()> {
    Ok(stream.write_all(message.as_bytes()).await?)
}

pub async fn read_message(stream: &mut TcpStream) -> ServerResult<String> {
    let mut buffer = [0u8; 1024];
    let bytes_read = stream.read(&mut buffer).await?;
    let message = std::str::from_utf8(&buffer[..bytes_read])?.trim();
    Ok(message.to_string())
}

pub async fn timed_read_answer(stream: &mut TcpStream) -> ServerResult<String> {
    let message_future = read_message(stream);
    timeout(Duration::from_secs(10), message_future).await?
}
