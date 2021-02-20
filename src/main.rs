mod quiz;

extern crate html_escape;

use reqwest::Error;
use std::env;
use rand::thread_rng;
use rand::seq::SliceRandom;
use quiz::functions::{Config, reset_screen, ask_answer, get_questions, 
    get_categories, show_score};

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