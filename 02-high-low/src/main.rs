use rand::Rng;
use std::cmp::Ordering;

fn main() {
    loop {
        play_game();

        if !do_play_again() {
            break;
        }
    }
}

fn play_game() {
    let goal = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("guess a number");

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        let guess = match line.trim().parse::<i32>() {
            Ok(x) => x,
            Err(_) => {
                println!("please enter a positive integer");
                continue;
            }
        };

        match guess.cmp(&goal) {
            Ordering::Greater => println!("too high"),
            Ordering::Less => println!("too low"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        };
    }
}

fn do_play_again() -> bool {
    println!("play again [Y/n]?");

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    match line.trim().to_lowercase().chars().next() {
        Some('y') | None => true,
        Some('n') => false,
        Some(_) => {
            println!("invalid option");
            do_play_again()
        }
    }
}
