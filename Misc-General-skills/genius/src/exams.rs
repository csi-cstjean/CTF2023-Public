use rand::{rngs::StdRng, Rng, SeedableRng};
use tokio::net::TcpStream;

use crate::{
    algebra::AlgebraicQuestionBuilder,
    operation::{OperationQuestionBuilder, Question},
    prelude::ServerResult,
    util::send_message,
    NUM_QUESTION,
};

pub async fn first_exam(stream: &mut TcpStream) -> ServerResult<()> {
    let builder = OperationQuestionBuilder::new().with_max_operands(2);

    for i in 1..=NUM_QUESTION {
        let question = builder.build();
        send_message(stream, &format!("Question {} de {}\n", i, NUM_QUESTION)).await?;
        question.ask(stream).await?;
    }

    Ok(())
}

pub async fn second_exam(stream: &mut TcpStream) -> ServerResult<()> {
    let builder = OperationQuestionBuilder::new().with_max_operands(6);

    for i in 1..=NUM_QUESTION {
        let question = builder.build();
        send_message(stream, &format!("Question {} de {}\n", i, NUM_QUESTION)).await?;
        question.ask(stream).await?;
    }

    Ok(())
}

pub async fn third_exam(stream: &mut TcpStream) -> ServerResult<()> {
    let builder = AlgebraicQuestionBuilder::new().with_max_terms(2);

    for i in 1..=NUM_QUESTION {
        let question = builder.simple();
        send_message(stream, &format!("Question {} de {}\n", i, NUM_QUESTION)).await?;
        question.ask(stream).await?;
    }

    Ok(())
}

pub async fn fourth_exam(stream: &mut TcpStream) -> ServerResult<()> {
    let builder = AlgebraicQuestionBuilder::new().with_max_terms(6);

    for i in 1..=NUM_QUESTION {
        let question = builder.system_of_equations();
        send_message(stream, &format!("Question {} de {}\n", i, NUM_QUESTION)).await?;
        question.ask(stream).await?;
    }

    Ok(())
}

pub async fn fifth_exam(stream: &mut TcpStream) -> ServerResult<()> {
    let builder = AlgebraicQuestionBuilder::new().with_max_terms(6);

    for i in 1..=NUM_QUESTION {
        let question = builder.quadratic();
        send_message(stream, &format!("Question {} de {}\n", i, NUM_QUESTION)).await?;
        question.ask(stream).await?;
    }

    Ok(())
}

pub async fn final_exam(stream: &mut TcpStream) -> ServerResult<()> {
    let algebra_builder = AlgebraicQuestionBuilder::new().with_max_terms(6);
    let operation_builder = OperationQuestionBuilder::new().with_max_operands(6);

    let mut rng = StdRng::from_entropy();

    for i in 1..=NUM_QUESTION {
        let question_type = rng.gen_range(0..=1);
        send_message(stream, &format!("Question {} de {}\n", i, NUM_QUESTION)).await?;

        match question_type {
            0 => {
                let question = algebra_builder.build();
                question.ask(stream).await?;
            }
            1 => {
                let question = operation_builder.build();
                question.ask(stream).await?;
            }
            _ => {}
        }
    }

    Ok(())
}
