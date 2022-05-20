use std::collections::HashMap;

fn main() {
    println!("Enter a roman numeral to convert:");

    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("could not read from stdin");

    line = line.trim().to_string();

    let converted = roman_to_arabic(&line);

    println!("{} -> {}", line, converted);
}

pub fn roman_to_arabic(str: &str) -> i32 {
    let conversions = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let nums = str
        .trim()
        .to_uppercase()
        .chars()
        .map(|x| *conversions.get(&x).unwrap())
        .collect::<Vec<_>>();

    let diffs = nums
        .windows(2)
        .map(|group| group[0] >= group[1])
        .chain([true])
        .collect::<Vec<_>>();

    nums.into_iter().zip(diffs).fold(
        0,
        |acc, (num, should_add)| if should_add { acc + num } else { acc - num },
    )
}

#[test]
fn test_single_character_conversion() {
    assert_eq!(roman_to_arabic("I"), 1);
}

#[test]
fn test_multiple_character_conversion() {
    assert_eq!(roman_to_arabic("III"), 3);
}

#[test]
fn test_use_correct_mapping_for_char() {
    assert_eq!(roman_to_arabic("V"), 5);
}

#[test]
fn test_handle_lower_case_mappings() {
    assert_eq!(roman_to_arabic("xvi"), 16);
}

#[test]
fn test_all_mappings_exist() {
    assert_eq!(
        roman_to_arabic("MDCLXVI"),
        1000 + 500 + 100 + 50 + 10 + 5 + 1
    );
}

#[test]
fn test_subtracting_when_lower_precedence() {
    assert_eq!(roman_to_arabic("IX"), 9);
}
