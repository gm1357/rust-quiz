use serde::Deserialize;
use reqwest::Error;
use std::env;
use std::process;

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

    let quiz_questions = response.json::<Quiz>().await?;
    println!("{:?}", quiz_questions);
    Ok(())
}