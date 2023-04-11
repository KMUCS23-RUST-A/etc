use std::io::{self, Write};
use rand::Rng;

fn main() {
    let words = vec!["apple", "banana", "cherry", "date", "elderberry"];
    let mut rng = rand::thread_rng();
    let word = words[rng.gen_range(0..words.len())];
    let mut guessed_letters = vec![];
    let mut num_guesses = 0;

    loop {
        println!("Guess a letter:");
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess = guess.trim().chars().next().unwrap();

        if guessed_letters.contains(&guess) {
            println!("You already guessed that letter!");
            continue;
        }

        guessed_letters.push(guess);

        if !word.contains(guess) {
            num_guesses += 1;
        }

        let mut guessed_word = String::new();
        for letter in word.chars() {
            if guessed_letters.contains(&letter) {
                guessed_word.push(letter);
            } else {
                guessed_word.push('_');
            }
        }

        println!("Guessed so far: {}", guessed_word);

        if !guessed_word.contains('_') {
            println!("You win!");
            break;
        }

        if num_guesses >= 6 {
            println!("You lose! The word was {}", word);
            break;
        }
    }
}
