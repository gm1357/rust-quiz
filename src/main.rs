extern crate html_escape;

use serde::Deserialize;
use reqwest::Error;
use std::env;
use rand::thread_rng;
use rand::seq::SliceRandom;
use text_io::read;
use std::io;
use std::io::Write;

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
    reset_screen();

    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    reset_screen();

    let request_url = format!(
        "https://opentdb.com/api.php?amount={amount}&category={category}&type=multiple",
        amount = config.amount,
        category = config.category
    );
    let response = reqwest::get(&request_url).await?;

    let quiz_response = response.json::<Quiz>().await?;
    let questions = quiz_response.results;
    let mut correct_guesses = 0;

    for question in questions {
        println!("{}", html_escape::decode_html_entities(&question.question));
        
        let mut answers = vec![question.correct_answer.clone()];
        answers.extend_from_slice(&question.incorrect_answers);
        answers.shuffle(&mut thread_rng());
        println!();
        for (index, answer) in answers.iter().enumerate() {
            println!("{}) {}", index + 1, html_escape::decode_html_entities(&answer));
        }
        println!();

        let index_player_answer = ask_answer(&answers);

        if answers.get(index_player_answer).unwrap().to_string() == question.correct_answer {
            println!("Good one! That was correct!");
            correct_guesses += 1;
        } else {
            println!("That's wrong. :(");
            println!("Answer was: {}", question.correct_answer);
        }
        println!();
    }

    println!("You got {} correct answers out of {}.", correct_guesses, config.amount);
    println!();

    Ok(())
}

fn reset_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().unwrap();
}

fn print_same_line(s: &str) {
    print!("{}", s);
    io::stdout().flush().unwrap();
}

fn ask_amount() -> String {
    let mut amount = String::from("");
    print_same_line("What's the amount of questions to be asked? ");

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
    print_same_line("What's the category of the questions? ");

    while !category.parse::<i32>().is_ok() {
        category = read!("{}\n");

        if !category.parse::<i32>().is_ok() {
            println!("Please enter a valid number.")
        }
    }
    category
}

fn ask_answer(answers: &Vec<String>) -> usize {
    let mut player_answer: String = read!("{}\n");
    let mut answer_index: usize;
    match player_answer.parse::<i32>() {
        Ok(index) => answer_index = (index - 1) as usize,
        Err(_) => answer_index = 4
    }

    while answers.get(answer_index).is_none() {
        println!("Enter the number of one of the alternatives (1, 2, 3, 4).");
        player_answer = read!("{}\n");
        match player_answer.parse::<i32>() {
            Ok(index) => answer_index = (index - 1) as usize,
            Err(_) => answer_index = 4
        }
    }
    answer_index
}