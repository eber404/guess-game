mod guesser;

use guesser::{ask_for_guess, compare_numbers, get_random_number, say_the_result};
use std::cmp::Ordering;

fn happy_flow(round: u32, secret_number: u32) {
    let is_another_round = round > 0;
    let guess = ask_for_guess(is_another_round);
    let has_match = compare_numbers(guess, secret_number);
    say_the_result(has_match);
    if has_match != Ordering::Equal {
        let incremented_round = round + 1;
        return happy_flow(incremented_round, secret_number);
    }
}

fn main() {
    println!("### Welcome to GUESS MY NUMBER THE GAME ### v0.1a");
    let round = 0;
    let secret_number = get_random_number(1, 10);
    happy_flow(round, secret_number);
}
