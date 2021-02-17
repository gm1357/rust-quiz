extern crate html_escape;

use serde::Deserialize;
use reqwest::Error;
use std::env;
use rand::thread_rng;
use rand::seq::SliceRandom;
use text_io::read;

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
    fn new(args: &[String]) -> Config {
        let amount: String;
        let category: String;

        match args.len() {
            3 => {
                amount = args[1].clone();
                category = args[2].clone();
            },
            2 => {
                amount = args[1].clone();
                category = ask_category();
            },
            _ => {
                amount = ask_amount();
                category = ask_category();
            }
        }

        Config { amount, category }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

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

fn ask_amount() -> String {
    let mut amount = String::from("");
    println!("What's the amount of questions to be asked?");

    while !amount.parse::<i32>().is_ok() {
        amount = read!("{}\n");

        if !amount.parse::<i32>().is_ok() {
            println!("Please enter a valid number.")
        }
    }
    amount
}

fn ask_category() -> String {
    let mut category = String::from("");
    println!("What's the category of the questions?");

    while !category.parse::<i32>().is_ok() {
        category = read!("{}\n");

        if !category.parse::<i32>().is_ok() {
            println!("Please enter a valid number.")
        }
    }
    category
}