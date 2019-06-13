use std::io;

fn main() {
    print_header();

    let mut user_guess = String::new();

    io::stdin().read_line(&mut user_guess).expect("Failed to read line");

    print_line_breaker();

    print_result(user_guess);
}

fn print_header(){
    print_line_breaker();

    println!("#################");
    println!("  Guessing Game  ");
    println!("#################");

    print_line_breaker();

    println!("What is your guess?");

    print_line_breaker();
}

fn print_line_breaker() {
    println!(" ");
}

fn print_result(user_guess: String) {
    println!("You guessed: {}", user_guess);
}
