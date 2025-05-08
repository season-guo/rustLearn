// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

//const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    let length : usize = secret_word.len();
    let mut guessed_letters = String::new();
    let mut guessed_word = vec!['-'; length];
    let mut chance : usize = 5;
    let mut guessed : usize = 0;
    let mut letter : char = 'a';
    let mut input = String::new();
    let mut has = false;
    while chance != 0 && guessed != length {
       println!("{}", guessed);
       print!("The word so far is {}
        You have guessed the following letters: {}
        You have {} guesses left
        Please guess a letter: ", guessed_word.iter().collect::<String>(), guessed_letters, chance);
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).expect("read err");
        if input.len() != 2 {
            println!("wrong input");
            continue ;
        }else{
            if let Some(i) = input.chars().nth(0){
                letter = i;
            };
        }
        for (index, val) in secret_word_chars.iter().enumerate() {
            if *val == letter {
                has = true;
                if guessed_word[index] == '-' {
                    guessed += 1;
                }
                guessed_word[index] = *val;
            }
        }
        if !has {
            chance -= 1;
        }
        has = false;
        guessed_letters.push(letter);
    }
    println!("Congratulation!")
}
