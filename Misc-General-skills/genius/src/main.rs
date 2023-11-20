use std::process::exit;

use crate::{
    prelude::*,
    util::{read_message, send_message},
};
use algebra::{AlgebraicQuestion, AlgebraicQuestionBuilder};
use choice::{read_user_choice, UserChoice};
use exams::{fifth_exam, final_exam, first_exam, fourth_exam, second_exam, third_exam};
use tera::{Context, Tera};
use tokio::net::{TcpListener, TcpStream};
use tracing::error;
use tracing::info;

mod algebra;
mod choice;
mod error;
mod exams;
mod operation;
mod prelude;
mod util;

const NUM_EXAMS: usize = 6;
const NUM_QUESTION: usize = 1000;

static EXAMS: [&'static str; NUM_EXAMS] = [
    "L'initiation d'Eugène",
    "Le progrès d'Eugène",
    "Les secrets du séminaire",
    "La recommandation de l'abbé",
    "L'avant-garde de la Polytechnique",
    "La conscience d'Eugène",
];

static EXAM_KEYS: [&str; NUM_EXAMS] = [
    "UNLOCKED",
    "NaQnXVGdSRhVCjobmw1W",
    "dF83jAcMKZLSUrrVGvAX",
    "syibBOl88cm545dcER5R",
    "Jyi9r1XYtDetTP8zG1Xg",
    "2YlrLWgTROaunowe8tYo",
];

static EXAM_FLAGS: [&str; NUM_EXAMS] = [
    "cst{L1n1714710n_d3u9en3_8c8ada}",
    "cst{L3_pr09re$_d3u9en3_e6f247}",
    "cst{L3$_$3cr37$_du_$em1n41r3_9efe1c}",
    "cst{L4_r3c0mm4nd4710n_d3_l4bbe_034d96}",
    "cst{L4v4n7_94rd3_d3_l4_P0ly73chn1qu3_b5e567}",
    "cst{L4_c0n$c13nc3_d3u9en3_a1ee7a}",
];

async fn list_exams(stream: &mut TcpStream, unlocked_exams: u8) -> ServerResult<()> {
    let mut list = String::from("\nListe des examens\n\n");
    for (i, exam) in EXAMS.iter().enumerate() {
        list += &format!(
            "{}. {} [{}]\n",
            i + 1,
            exam,
            if (i as u8) < unlocked_exams {
                "Déverrouillé"
            } else {
                "Verrouillé"
            }
        );
    }
    list += "\n";

    send_message(stream, &list).await
}

async fn process_exam_choice(stream: &mut TcpStream, unlocked_exams: u8) -> ServerResult<u8> {
    send_message(
        stream,
        "Entrez le numéro de l'examen que vous souhaitez passer :\n",
    )
    .await?;
    let exam_number = read_message(stream).await?.parse::<u8>()?;
    if exam_number > unlocked_exams && !prompt_for_key(stream, exam_number).await? {
        return Ok(unlocked_exams);
    }

    send_message(stream, "Vous avez 10 secondes par question!\n").await?;
    send_message(stream, "Il faut juste 100% pour passer l'examen\n\n").await?;

    match exam_number {
        1 => first_exam(stream).await?,
        2 => second_exam(stream).await?,
        3 => third_exam(stream).await?,
        4 => fourth_exam(stream).await?,
        5 => fifth_exam(stream).await?,
        6 => final_exam(stream).await?,
        _ => {
            send_message(stream, "Le numéro de l'examen n'existe pas").await?;
            return Ok(unlocked_exams);
        }
    }

    exam_success(stream, exam_number).await?;
    Ok(exam_number)
}

#[tracing::instrument(skip(stream))]
async fn handle_client(mut stream: TcpStream) -> ServerResult<()> {
    let mut unlocked_exams = 1;
    loop {
        send_message(&mut stream, include_str!("../static/menu.txt")).await?;
        send_message(&mut stream, "> ").await?;
        match read_user_choice(&mut stream).await {
            Ok(UserChoice::ListExams) => {
                if let Err(e) = list_exams(&mut stream, unlocked_exams).await {
                    handle_error(&mut stream, e).await?;
                }
            }
            Ok(UserChoice::TakeExam) => {
                match process_exam_choice(&mut stream, unlocked_exams).await {
                    Ok(new_unlocked) => unlocked_exams = new_unlocked,
                    Err(e) => handle_error(&mut stream, e).await?,
                }
            }
            Ok(UserChoice::Quit) => {
                send_message(&mut stream, "Au revoir !\n").await?;
                return Ok(());
            }
            Err(e) => handle_error(&mut stream, e).await?,
        }
    }
}

async fn handle_error(stream: &mut TcpStream, error: ServerError) -> ServerResult<()> {
    match error {
        ServerError::Io(_) => {
            error!("Error with IO: {}", error);
            send_message(
                stream,
                "\nUne erreur est survenue lors de la communication.\n",
            )
            .await?;
            Err(error)
        }
        ServerError::Utf8(_) => {
            send_message(stream, "\nErreur de conversion UTF-8.\n").await?;
            Err(error)
        }
        ServerError::InvalidNumber(_) => {
            send_message(stream, "\nNuméro invalide.\n").await?;
            Err(error)
        }
        ServerError::InvalidFloatNumber(_) => {
            send_message(stream, "\nFloat invalide.\n").await?;
            Err(error)
        }

        ServerError::TimeElapsed(_) => {
            send_message(stream, "\nLe temps est écoulé.\n").await?;
            Err(error)
        }
        ServerError::InvalidAnswer => {
            send_message(stream, "\nLa réponse est incorrecte.\n").await?;
            Err(error)
        }
        ServerError::TemplateParsing(_) => {
            error!("Error while parsing template: {}", error);
            send_message(stream, "\nErreur lors de l'analyse du modèle.\n").await?;
            Err(error)
        }
        ServerError::InvalidOperationAnswer(answer) => {
            send_message(
                stream,
                &format!(
                    "\nLa réponse est incorrecte. Voici la bonne réponse : {}\n",
                    answer
                ),
            )
            .await?;
            Err(error)
        }
    }
}

async fn exam_success(stream: &mut TcpStream, exam_number: u8) -> ServerResult<()> {
    let mut context = Context::new();
    let index = exam_number as usize;
    let has_next = index != NUM_EXAMS;
    if has_next {
        context.insert("next_key", EXAM_KEYS[index]);
    }
    context.insert("has_next", &has_next);
    context.insert("exam_name", EXAMS[index - 1]);
    context.insert("flag", EXAM_FLAGS[index - 1]);
    let result = Tera::one_off(include_str!("../static/success.txt"), &context, false)?;
    send_message(stream, &result).await?;
    Ok(())
}

async fn prompt_for_key(stream: &mut TcpStream, exam_number: u8) -> ServerResult<bool> {
    send_message(
        stream,
        &format!(
            "L'examen {} est verrouillé. Entrez la clé pour le déverrouiller :\n",
            exam_number
        ),
    )
    .await?;
    let key = read_message(stream).await?;

    if key == EXAM_KEYS[(exam_number - 1) as usize] {
        send_message(
            stream,
            "Clé correcte! L'examen est maintenant déverrouillé.\n\n",
        )
        .await?;
        Ok(true)
    } else {
        send_message(stream, "Clé incorrecte. Veuillez réessayer.\n\n").await?;
        Ok(false)
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    match run_server().await {
        Ok(()) => info!("Server is gracefully shutdown."),
        Err(e) => error!("Server encountered an error: {}", e),
    }
}

#[tracing::instrument]
async fn run_server() -> ServerResult<()> {
    let address = "0.0.0.0:1234";
    let listener = TcpListener::bind(&address).await?;

    info!("Server is listening on {}", address);

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(handle_client(stream));
            }
            Err(e) => {
                error!("Failed to accept the connection: {}", e);
            }
        }
    }
}
