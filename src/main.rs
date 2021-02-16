extern crate html_escape;

use serde::Deserialize;
use reqwest::Error;
use std::env;
use std::process;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Deserialize, Debug)]
struct Quiz {
    response_code: u32,
    results: Vec<Question>
}

#[derive(Deserialize, Debug)]
struct Question {
    category: String,
    question: String,
    difficulty: String,
    correct_answer: String,
    incorrect_answers: Vec<String>
}

struct Config {
    amount: String,
    category: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("please provide amount of questions and category type.");
        }

        let amount = args[1].clone();
        let category = args[2].clone();

        Ok(Config { amount, category })
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let request_url = format!(
        "https://opentdb.com/api.php?amount={amount}&category={category}&type=multiple",
        amount = config.amount,
        category = config.category
    );
    let response = reqwest::get(&request_url).await?;

    let quiz_response = response.json::<Quiz>().await?;
    let questions = quiz_response.results;

    for question in questions {
        println!("QUESTION: {}", html_escape::decode_html_entities(&question.question));
        
        println!("ANSWER: {}", html_escape::decode_html_entities(&question.correct_answer));
        let mut answers = vec![question.correct_answer];
        answers.extend_from_slice(&question.incorrect_answers);
        answers.shuffle(&mut thread_rng());
        println!("OPTIONS:");
        for answer in answers {
            println!("{}", html_escape::decode_html_entities(&answer));
        }
        println!();
    }

    Ok(())
}