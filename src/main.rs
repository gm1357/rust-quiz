extern crate html_escape;
extern crate ferris_says;

use serde::Deserialize;
use reqwest::Error;
use std::env;
use rand::thread_rng;
use rand::seq::SliceRandom;
use text_io::read;
use std::io;
use std::str;
use std::io::{ stdout, BufWriter, Write };
use ferris_says::say;

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

#[derive(Deserialize, Debug)]
struct Categories {
    trivia_categories: Vec<Category>
}

#[derive(Deserialize, Debug)]
struct Category {
    id: i32,
    name: String
}

struct Config {
    amount: String,
    category: String,
}

impl Config {
    fn new(args: &[String], categories: Vec<Category>) -> Config {
        let amount: String;
        let category: String;

        match args.len() {
            3 => {
                amount = args[1].clone();
                category = args[2].clone();
            },
            2 => {
                amount = args[1].clone();
                category = ask_category(categories);
            },
            _ => {
                amount = ask_amount();
                category = ask_category(categories);
            }
        }

        Config { amount, category }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    reset_screen();

    let categories = get_categories().await?;
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args, categories);
    reset_screen();

    let questions = get_questions(&config.amount, &config.category).await?;
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
            println!("Answer was: {}", html_escape::decode_html_entities(&question.correct_answer));
        }
        println!();
    }

    match config.amount.parse::<i32>() {
        Ok(total) => show_score(total, correct_guesses),
        Err(_) => show_score(0, 0)
    }

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

fn ask_category(categories: Vec<Category>) -> String {
    let mut category = String::from("");
    print_same_line("What's the category of the questions? (type '?' to see all)");
    println!();

    while !category.parse::<i32>().is_ok() {
        category = read!("{}\n");

        if category == "?" {
            print_categories(&categories);
        } else if !category.parse::<i32>().is_ok() {
            println!("Please enter a valid category number.")
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

async fn get_questions(amount: &str, category: &str) -> Result<Vec<Question>, Error> {
    let request_url = format!(
        "https://opentdb.com/api.php?amount={amount}&category={category}&type=multiple",
        amount = amount,
        category = category
    );
    let response = reqwest::get(&request_url).await?;
    let quiz_response = response.json::<Quiz>().await?;

    return Ok(quiz_response.results);
}

async fn get_categories() -> Result<Vec<Category>, Error> {
    let request_url = format!("https://opentdb.com/api_category.php");
    let response = reqwest::get(&request_url).await?;
    let categories = response.json::<Categories>().await?;

    return Ok(categories.trivia_categories);
}

fn show_score(total: i32, correct_guesses: i32) {
    println!("You got {} correct answers out of {}.", correct_guesses, total);
    println!();
    let average = correct_guesses as f32 / total as f32;
    let out: &[u8];

    if average == 1.0 {
        out = b"Congrats! You did perfectly.";
    } else if average > 0.7 {
        out = b"Well done! almost got all.";
    } else if average > 0.4 {
        out = b"You did okay.";
    } else if average > 0.0 {
        out = b"Better than zero.";
    } else {
        out = b"Surely you can do better, right?";
    }

    let width = 24;
    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap();
}

fn print_categories(categories: &Vec<Category>) {
    let mut data: Vec<Vec<String>> = vec![];

    for category in categories {
        let caregory_vec = vec![category.id.to_string(), category.name.clone()];
        data.push(caregory_vec);
    }

    text_tables::render(&mut io::stdout(), data).unwrap();
}