use std::vec;

use async_trait::async_trait;
use rand::{seq::SliceRandom, Rng};
use tokio::net::TcpStream;

use crate::{
    operation::Question,
    prelude::{ServerError, ServerResult},
    util::{send_message, timed_read_answer},
};

#[derive(Clone)]
pub enum AlgebraicOperation {
    Simple,
    Quadratic,
    SystemOfEquations,
}

impl AlgebraicOperation {
    fn random() -> Self {
        let operations = [
            AlgebraicOperation::Simple,
            AlgebraicOperation::Quadratic,
            AlgebraicOperation::SystemOfEquations,
        ];
        operations.choose(&mut rand::thread_rng()).cloned().unwrap()
    }
}

pub struct AlgebraicQuestion {
    pub operation: AlgebraicOperation,
    pub text: String,
    pub answer: Vec<f64>,
}

#[async_trait]
impl Question for AlgebraicQuestion {
    async fn ask(&self, stream: &mut TcpStream) -> ServerResult<()> {
        match self.operation {
            AlgebraicOperation::SystemOfEquations => {
                send_message(stream, &self.text).await?;
                send_message(stream, "x = ").await?;
                let x = timed_read_answer(stream).await?.parse::<f64>()?;
                send_message(stream, "y = ").await?;
                let y = timed_read_answer(stream).await?.parse::<f64>()?;
                let x_valid = almost_equal(self.answer[0], x);
                let y_valid = almost_equal(self.answer[1], y);
                if !x_valid || !y_valid {
                    return Err(ServerError::InvalidAnswer);
                }

                println!("x: {}, y: {}", x, y);
            }
            _ => {
                send_message(stream, &self.text).await?;
                send_message(stream, "x = ").await?;
                let timed_answer = timed_read_answer(stream).await?.parse::<f64>()?;
                let is_valid = self
                    .answer
                    .iter()
                    .any(|&answer| almost_equal(timed_answer, answer));

                println!("is valid: {:?}", is_valid);

                if !is_valid {
                    return Err(ServerError::InvalidAnswer);
                }
            }
        }
        Ok(())
    }
}

fn almost_equal(number: f64, comparee: f64) -> bool {
    (number - comparee).abs() <= 0.01
}

pub struct AlgebraicQuestionBuilder {
    max_terms: usize,
}

impl AlgebraicQuestionBuilder {
    pub fn new() -> Self {
        Self { max_terms: 2 }
    }

    pub fn with_max_terms(mut self, max_terms: usize) -> Self {
        self.max_terms = max_terms;
        self
    }

    pub fn build(&self) -> AlgebraicQuestion {
        let operation = AlgebraicOperation::random();
        match operation {
            AlgebraicOperation::Simple => self.simple(),
            AlgebraicOperation::Quadratic => self.quadratic(),
            AlgebraicOperation::SystemOfEquations => self.system_of_equations(),
        }
    }

    pub fn quadratic(&self) -> AlgebraicQuestion {
        let mut rng = rand::thread_rng();

        let (a, b, c, d) = loop {
            let a: i64 = rng.gen_range(1..=10);
            let b: i64 = rng.gen_range(-10..=10);
            let c: i64 = rng.gen_range(-10..=10);
            let d: i64 = rng.gen_range(-10..=10);
            let discriminant = (b * b - 4 * a * (c - d)) as f64;
            if discriminant >= 0.0 {
                break (a, b, c, d);
            }
        };

        // effective coefficients based on d
        let effective_c = c - d;

        // roots based on effective coefficients
        let discriminant = (b * b - 4 * a * effective_c) as f64;
        let x1 = ((-b as f64) + discriminant.sqrt()) / (2.0 * a as f64);
        let x2 = ((-b as f64) - discriminant.sqrt()) / (2.0 * a as f64);

        let text = if rng.gen_bool(0.5) {
            format!("{}x^2 + {}x + {} = {}\n", a, b, c, d)
        } else {
            format!("{} = {}x^2 + {}x + {}\n", d, a, b, c)
        };

        AlgebraicQuestion {
            operation: AlgebraicOperation::Quadratic,
            text,
            answer: vec![x1, x2],
        }
    }

    pub fn simple(&self) -> AlgebraicQuestion {
        let mut rng = rand::thread_rng();

        let num_terms = rng.gen_range(1..=self.max_terms);
        let mut terms: Vec<i64>;
        let mut total_x: i64;
        let mut lhs = String::new();

        loop {
            terms = (0..num_terms).map(|_| rng.gen_range(1..=10)).collect();
            total_x = terms[0];
            lhs = format!("{}x", terms[0]); // Start building lhs here

            for &term in terms.iter().skip(1) {
                let sign = if rng.gen_bool(0.5) { '+' } else { '-' };
                lhs.push_str(&format!(" {} {}x", sign, term));
                total_x = if sign == '+' {
                    total_x + term
                } else {
                    total_x - term
                };
            }

            if total_x != 0 {
                break; // We have a valid total coefficient for x, exit the loop
            }
            // If total_x is 0, the loop continues, and a new set of terms is generated
        }

        let rhs: i64 = rng.gen_range(1..=100);
        let answer: f64 = rhs as f64 / total_x as f64;

        let question_text = if rng.gen_bool(0.5) {
            format!("{} = {}\n", lhs, rhs)
        } else {
            format!("{} = {}\n", rhs, lhs)
        };

        AlgebraicQuestion {
            operation: AlgebraicOperation::Simple,
            text: question_text,
            answer: vec![answer],
        }
    }

    pub fn system_of_equations(&self) -> AlgebraicQuestion {
        let mut rng = rand::thread_rng();

        let (a1, b1, c1, a2, b2, c2) = loop {
            let a1: i64 = rng.gen_range(1..=10);
            let b1: i64 = rng.gen_range(1..=10);
            let c1: i64 = rng.gen_range(1..=100);
            let a2: i64 = rng.gen_range(1..=10);
            let b2: i64 = rng.gen_range(1..=10);
            let c2: i64 = rng.gen_range(1..=100);
            if a1 * b2 - a2 * b1 != 0 {
                break (a1, b1, c1, a2, b2, c2);
            }
        };

        let determinant = a1 * b2 - a2 * b1;
        let x = (c1 * b2 - c2 * b1) as f64 / determinant as f64;
        let y = (a1 * c2 - a2 * c1) as f64 / determinant as f64;

        let text = format!("{}x + {}y = {}\n{}x + {}y = {}\n", a1, b1, c1, a2, b2, c2);

        AlgebraicQuestion {
            operation: AlgebraicOperation::SystemOfEquations,
            text,
            answer: vec![x, y],
        }
    }
}
