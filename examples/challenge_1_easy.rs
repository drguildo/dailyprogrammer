use std::io::{stdin, stdout, Write};

fn main() {
    print_prompt("Name: ");
    let name = read_input();

    print_prompt("Age: ");
    let age = read_input();

    print_prompt("Reddit Username: ");
    let reddit_username = read_input();

    let response = format!(
        "Your name is {}, you are {} years old, and your Reddit username is {}.\n",
        name.trim(),
        age.trim(),
        reddit_username.trim()
    );

    print!("{}", response);

    let filename = "challenge_1_easy.txt";
    if !std::fs::exists(filename).unwrap_or(true) {
        std::fs::write(filename, response).unwrap();
    }
}

fn print_prompt(prompt: &str) {
    print!("{}", prompt);
    stdout().flush().unwrap();
}

fn read_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}
