use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn ask_for_a_guess() -> u32 {
    println!("please input your guess");

    let mut guess = String::new();

    stdin().read_line(&mut guess).expect("failed to read line");

    let guess: u32 = guess.trim().parse().expect("please, type a number");

    return guess;
}

fn get_random_number(a: u32, b: u32) -> u32 {
    let random_number = rand::thread_rng().gen_range(a..b);
    return random_number;
}

fn compare_numbers(guess: u32, random_number: u32) -> Ordering {
    return guess.cmp(&random_number);
}

fn say_the_result(res: Ordering) -> bool {
    match res {
        Ordering::Less => {
            println!("too small 🤏");
            return false;
        }
        Ordering::Greater => {
            println!("too big 🍆");
            return false;
        }
        Ordering::Equal => println!("you win!!! 🎉"),
    }

    return true;
}

fn happy_flow() {
    println!("### Welcome to GUESS THE NUMBER ### v0.1a");

    let guess = ask_for_a_guess();
    println!("you guessed: {}", guess);

    let random_number = get_random_number(1, 10);

    let comparison = compare_numbers(guess, random_number);

    if comparison == Ordering::Equal {
        say_the_result(comparison);
        return happy_flow();
    }

    let should_rerun = say_the_result(comparison);

    if should_rerun {
        happy_flow();
    }
}

fn main() {
    happy_flow();
}
