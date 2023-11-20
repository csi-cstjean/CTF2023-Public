use async_recursion::async_recursion;
use tokio::net::TcpStream;

use crate::{prelude::ServerResult, util::{read_message, send_message}};

pub enum UserChoice {
    ListExams,
    TakeExam,
    Quit,
}

#[async_recursion]
pub async fn read_user_choice(stream: &mut TcpStream) -> ServerResult<UserChoice> {
    let choice = read_message(stream).await?.parse::<u8>()?;

    match choice {
        1 => Ok(UserChoice::ListExams),
        2 => Ok(UserChoice::TakeExam),
        3 => Ok(UserChoice::Quit),
        _ => {
            send_message(stream, "Choix non valide. Veuillez essayer à nouveau.\n").await?;
            read_user_choice(stream).await
        }
    }
}

