extern crate html_escape;
extern crate ferris_says;

use reqwest::Error;
use text_io::read;
use std::io;
use std::str;
use std::io::{ stdout, BufWriter, Write };
use ferris_says::say;
use serde::Deserialize;

#[derive(Deserialize)]
struct Quiz {
    pub response_code: u32,
    pub results: Vec<Question>
}

#[derive(Deserialize)]
pub struct Question {
    pub category: String,
    pub question: String,
    pub difficulty: String,
    pub correct_answer: String,
    pub incorrect_answers: Vec<String>
}

#[derive(Deserialize)]
struct Categories {
    pub trivia_categories: Vec<Category>
}

#[derive(Deserialize)]
pub struct Category {
    pub id: i32,
    pub name: String
}

pub struct Config {
    pub amount: String,
    pub category: String,
}

impl Config {
    pub fn new(args: &[String], categories: Vec<Category>) -> Config {
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

pub fn reset_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().unwrap();
}

pub fn print_same_line(s: &str) {
    print!("{}", s);
    io::stdout().flush().unwrap();
}

pub fn ask_amount() -> String {
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

pub fn ask_category(categories: Vec<Category>) -> String {
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

pub fn ask_answer(answers: &Vec<String>) -> usize {
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

pub async fn get_questions(amount: &str, category: &str) -> Result<Vec<Question>, Error> {
    let request_url = format!(
        "https://opentdb.com/api.php?amount={amount}&category={category}&type=multiple",
        amount = amount,
        category = category
    );
    let response = reqwest::get(&request_url).await?;
    let quiz_response = response.json::<Quiz>().await?;

    return Ok(quiz_response.results);
}

pub async fn get_categories() -> Result<Vec<Category>, Error> {
    let request_url = format!("https://opentdb.com/api_category.php");
    let response = reqwest::get(&request_url).await?;
    let categories = response.json::<Categories>().await?;

    return Ok(categories.trivia_categories);
}

pub fn show_score(total: i32, correct_guesses: i32) {
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

pub fn print_categories(categories: &Vec<Category>) {
    let mut data: Vec<Vec<String>> = vec![];

    for category in categories {
        let caregory_vec = vec![category.id.to_string(), category.name.clone()];
        data.push(caregory_vec);
    }

    text_tables::render(&mut io::stdout(), data).unwrap();
}