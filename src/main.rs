use serde::Deserialize;
use reqwest::Error;

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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!(
        "https://opentdb.com/api.php?amount={amount}&category={category}&type=multiple",
        amount = "10",
        category = "0"
    );
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    let quiz_questions = response.json::<Quiz>().await?;
    println!("{:?}", quiz_questions);
    Ok(())
}