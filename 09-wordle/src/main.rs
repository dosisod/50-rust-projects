use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, BufRead};

const MAX_GUESSES: usize = 6;
const DEFAULT_WORDLIST: &str = "./wordle.txt";

fn main() {
    let file = match File::open(DEFAULT_WORDLIST) {
        Err(_) => panic!("\"{}\" could not open", DEFAULT_WORDLIST),
        Ok(x) => x,
    };

    let wordlist = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    let mut state = build_game_state(&wordlist).unwrap();

    println!("Type a word to get started\n");

    while !state.is_out_of_guesses() {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("could not open stdin");

        line = line.trim().to_string();

        println!("{}", state.play(&line));
    }

    if !state.won {
        println!("The word was: \"{}\"", state.secret);
    }
}

struct GameState {
    words: Vec<String>,
    secret: String,
    guesses: usize,
    won: bool,
}

#[derive(Clone)]
enum Hint {
    Exact,
    WrongPosition,
    NotInWord,
}

enum GuessOutcome {
    Win,
    InvalidWord,
    Hint(Vec<Hint>),
}

impl GameState {
    fn play(&mut self, guess: &String) -> String {
        let outcome = self.make_guess(guess);

        self.format_outcome(outcome, guess)
    }

    fn make_guess(&mut self, guess: &String) -> GuessOutcome {
        if *guess == self.secret {
            self.won = true;
            return GuessOutcome::Win;
        }

        if !self.words.contains(guess) {
            return GuessOutcome::InvalidWord;
        }

        self.guesses += 1;

        let hints = (*guess)
            .chars()
            .zip(self.secret.chars())
            .map(|(lhs, rhs)| {
                if lhs == rhs {
                    Hint::Exact
                } else if self.secret.chars().any(|c| c == lhs) {
                    Hint::WrongPosition
                } else {
                    Hint::NotInWord
                }
            })
            .collect::<Vec<_>>();

        GuessOutcome::Hint(hints)
    }

    fn is_out_of_guesses(&self) -> bool {
        self.guesses >= MAX_GUESSES
    }

    fn format_outcome(&self, outcome: GuessOutcome, line: &String) -> String {
        match outcome {
            GuessOutcome::Win => {
                let mut tmp = String::new();

                for c in line.to_uppercase().chars() {
                    tmp = format!("{}{}", tmp, format_exact(c));
                }

                tmp.push_str("\nyou win\n");
                tmp
            }
            GuessOutcome::InvalidWord => format!("\"{}\" wasn't found in list\n", line),
            GuessOutcome::Hint(hints) => {
                let mut tmp = String::new();

                for hint in hints.iter().zip(line.to_uppercase().chars()) {
                    let square = match hint.0 {
                        Hint::Exact => format_exact(hint.1),
                        Hint::NotInWord => format_not_in_word(hint.1),
                        Hint::WrongPosition => format_wrong_position(hint.1),
                    };

                    tmp = format!("{}{}", tmp, square);
                }

                tmp.push('\n');
                tmp
            }
        }
    }
}

fn build_game_state(words: &Vec<String>) -> Result<GameState, String> {
    validate_wordlist(words)?;

    let mut rng = rand::thread_rng();
    let secret = words.choose(&mut rng).unwrap();

    Ok(GameState {
        words: words.to_vec(),
        secret: secret.to_string(),
        guesses: 0,
        won: false,
    })
}

fn validate_wordlist(words: &[String]) -> Result<(), String> {
    if words.is_empty() {
        return Err("Wordlist cannot be empty".to_string());
    }

    let expected_len = words[0].len();

    if words.iter().any(|word| word.len() != expected_len) {
        return Err("All words must be of same size".to_string());
    }

    if words
        .iter()
        .any(|word| word.chars().any(|x| !('a'..='z').contains(&x)))
    {
        return Err("Words must be in lowercase and in between 'a' and 'z'".to_string());
    }

    Ok(())
}

fn format_wrong_position(c: char) -> String {
    format!("\x1b[48;2;181;159;59m {} \x1b[0m", c)
}

fn format_not_in_word(c: char) -> String {
    format!("\x1b[48;2;58;58;60m {} \x1b[0m", c)
}

fn format_exact(c: char) -> String {
    format!("\x1b[48;2;83;141;78m {} \x1b[0m", c)
}

#[test]
fn test_validate_wordlist_check_non_empty() {
    match validate_wordlist(&[]) {
        Ok(_) => assert!(false),
        Err(err) => assert_eq!("Wordlist cannot be empty", err),
    }
}

#[test]
fn test_validate_wordlist_words_all_same_len() {
    match validate_wordlist(&vec![String::from("x"), String::from("xxx")]) {
        Ok(_) => assert!(false),
        Err(err) => assert_eq!("All words must be of same size", err),
    }
}

#[test]
fn test_validate_wordlist_only_allow_lowercase() {
    match validate_wordlist(&vec![String::from("X")]) {
        Ok(_) => assert!(false),
        Err(err) => assert_eq!("Words must be in lowercase and in between 'a' and 'z'", err),
    }
}

#[test]
fn test_validate_wordlist_only_allow_a_to_z() {
    match validate_wordlist(&vec![String::from("1")]) {
        Ok(_) => assert!(false),
        Err(err) => assert_eq!("Words must be in lowercase and in between 'a' and 'z'", err),
    }
}

#[test]
fn test_build_game_state() {
    let state = build_game_state(&vec![String::from("hello"), String::from("world")]).unwrap();

    assert_eq!(state.words.len(), 2);
    assert_eq!(state.words[0], "hello");
    assert_eq!(state.words[1], "world");

    assert!(state.words.contains(&state.secret));

    assert_eq!(state.guesses, 0);
}

#[test]
fn test_guess_correct_word_is_a_win() {
    let mut state = build_game_state(&vec![String::from("word")]).unwrap();

    match state.make_guess(&"word".to_string()) {
        GuessOutcome::Win => (),
        _ => assert!(false),
    };

    assert!(state.won);
}

#[test]
fn test_guess_incorrect_word_is_incorrect_word() {
    let mut state = build_game_state(&vec![String::from("word")]).unwrap();

    match state.make_guess(&"not_in_wordlist".to_string()) {
        GuessOutcome::InvalidWord => (),
        _ => assert!(false),
    };
}

#[test]
fn test_guesses_incremented_when_hint_is_returned() {
    let mut state = build_game_state(&vec![String::from("a"), String::from("b")]).unwrap();
    state.secret = "a".to_string();

    match state.make_guess(&"b".to_string()) {
        GuessOutcome::Hint(_) => (),
        _ => assert!(false),
    };

    assert_eq!(state.guesses, 1);
}

#[test]
fn test_hint_returned_if_word_is_in_wordlist() {
    let mut state = build_game_state(&vec![String::from("a"), String::from("b")]).unwrap();

    state.secret = "a".to_string();

    match state.make_guess(&"b".to_string()) {
        GuessOutcome::Hint(x) => match x[..] {
            [Hint::NotInWord] => (),
            _ => assert!(false),
        },
        _ => assert!(false),
    };
}

#[test]
fn test_hint_exact_when_same_char_is_in_exact_position() {
    let mut state = build_game_state(&vec![String::from("abc"), String::from("xxc")]).unwrap();

    state.secret = "abc".to_string();

    match state.make_guess(&"xxc".to_string()) {
        GuessOutcome::Hint(x) => match x[..] {
            [Hint::NotInWord, Hint::NotInWord, Hint::Exact] => (),
            _ => assert!(false),
        },
        _ => assert!(false),
    };
}

#[test]
fn test_hint_char_in_wrong_place() {
    let mut state = build_game_state(&vec![String::from("abc"), String::from("cba")]).unwrap();

    state.secret = "abc".to_string();

    match state.make_guess(&"cba".to_string()) {
        GuessOutcome::Hint(x) => match x[..] {
            [Hint::WrongPosition, Hint::Exact, Hint::WrongPosition] => (),
            _ => assert!(false),
        },
        _ => assert!(false),
    };
}

#[test]
fn test_is_out_of_guesses_after_guessing_too_much() {
    let mut state = build_game_state(&vec![String::from("a"), String::from("b")]).unwrap();
    state.secret = "a".to_string();

    for _ in 1..MAX_GUESSES {
        state.make_guess(&"b".to_string());
        assert!(!state.is_out_of_guesses());
    }

    state.make_guess(&"b".to_string());
    assert!(state.is_out_of_guesses());
}
