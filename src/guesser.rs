use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

pub fn ask_for_guess(is_another_round: bool) -> u32 {
  let text = match is_another_round {
    true => "please input another guess: ",
    false => "please input your guess: ",
  };

  println!("{}", text);

  let mut guess = String::new();
  stdin().read_line(&mut guess).expect("failed to read line");
  let guess: u32 = guess
    .trim()
    .parse()
    .expect("please, type a positive number");

  return guess;
}

pub fn get_random_number(a: u32, b: u32) -> u32 {
  let random_number = rand::thread_rng().gen_range(a..b);
  return random_number;
}

pub fn compare_numbers(guess: u32, random_number: u32) -> Ordering {
  return guess.cmp(&random_number);
}

pub fn say_the_result(res: Ordering) {
  match res {
    Ordering::Less => println!("too small 🤏"),
    Ordering::Greater => println!("too big 🍆"),
    Ordering::Equal => println!("you win!!! 🎉"),
  }
}
