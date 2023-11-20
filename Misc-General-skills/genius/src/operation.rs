use async_trait::async_trait;
use rand::{seq::SliceRandom, Rng};
use tokio::net::TcpStream;

use crate::{
    prelude::{ServerError, ServerResult},
    util::{send_message, timed_read_answer},
};

#[derive(Clone)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
}

impl Operation {
    fn random() -> Self {
        let operations = [Operation::Add, Operation::Subtract, Operation::Multiply];
        operations.choose(&mut rand::thread_rng()).cloned().unwrap()
    }
}

#[async_trait]
pub trait Question {
    async fn ask(&self, stream: &mut TcpStream) -> ServerResult<()>;
}

pub struct OperationQuestion {
    pub text: String,
    pub answer: i64,
}

#[async_trait]
impl Question for OperationQuestion {
    async fn ask(&self, stream: &mut TcpStream) -> ServerResult<()> {
        send_message(stream, &self.text).await?;
        let answer = timed_read_answer(stream).await?.parse::<i64>()?;
        if self.answer != answer {
            return Err(ServerError::InvalidOperationAnswer(self.answer));
        }
        Ok(())
    }
}

pub struct OperationQuestionBuilder {
    max_operands: usize,
}

impl OperationQuestionBuilder {
    pub fn new() -> Self {
        Self { max_operands: 2 }
    }

    pub fn with_max_operands(mut self, max_operands: usize) -> Self {
        self.max_operands = max_operands;
        self
    }

    pub fn build(&self) -> OperationQuestion {
        let mut rng = rand::thread_rng();

        let num_operands = rng.gen_range(2..=self.max_operands);

        let operands: Vec<i64> = (0..num_operands).map(|_| rng.gen_range(1..=1000)).collect();

        let operations: Vec<Operation> =
            (0..num_operands - 1).map(|_| Operation::random()).collect();

        let mut text = operands[0].to_string();
        let mut answer = operands[0];
        for (index, &operand) in operands.iter().enumerate().skip(1) {
            match operations[index - 1] {
                Operation::Add => {
                    text.push_str(&format!(" + {}", operand));
                    answer += operand;
                }
                Operation::Subtract => {
                    text.push_str(&format!(" - {}", operand));
                    answer -= operand;
                }
                Operation::Multiply => {
                    text.push_str(&format!(" * {}", operand));
                    answer *= operand;
                }
            }
        }

        text = format!("{} = ?\n", text.trim_start());

        OperationQuestion { text, answer }
    }
}
