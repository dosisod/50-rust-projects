use rand::Rng;

fn main() {
    let alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let nums = "0123456789";
    let symbols = "`~!@#$%^&*()-_=+{}[]|\\:;\"',<.>/?";

    let charset = format!("{}{}{}", alpha, nums, symbols);

    const DEFAULT_PASSWORD_LEN: usize = 32;

    let password_length = match std::env::args().skip(1).next() {
        None => DEFAULT_PASSWORD_LEN,
        Some(arg) => arg.parse::<usize>().expect("Expected positive integer"),
    };

    let password = std::iter::repeat_with(|| rand::thread_rng().gen_range(0..charset.len()))
        .take(password_length)
        .map(|index| charset.chars().nth(index).unwrap())
        .collect::<String>();

    println!("{}", password);
}
