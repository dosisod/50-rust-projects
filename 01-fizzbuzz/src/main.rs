fn basic_version() {
    for i in 1..=100 {
        if i % 15 == 0 {
            println!("fizzbuzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }
}

fn alternate_version() {
    for i in 1..=100 {
        let is_multiple_of_3 = i % 3 == 0;
        let is_multiple_of_5 = i % 5 == 0;

        if is_multiple_of_3 {
            print!("fizz");
        }
        if is_multiple_of_5 {
            print!("buzz");
        }

        if is_multiple_of_3 || is_multiple_of_5 {
            println!();
        } else {
            println!("{}", i);
        }
    }
}

fn match_version() {
    for i in 1..=100 {
        match i {
            i if i % 15 == 0 => println!("fizzbuzz"),
            i if i % 3 == 0 => println!("fizz"),
            i if i % 5 == 0 => println!("buzz"),
            _ => println!("{}", i),
        };
    }
}

fn iter_version() {
    (1..=100)
        .map(|i| {
            if i % 15 == 0 {
                "fizzbuzz".to_string()
            } else if i % 3 == 0 {
                "fizz".to_string()
            } else if i % 5 == 0 {
                "buzz".to_string()
            } else {
                format!("{}", i)
            }
        })
        .for_each(|x| println!("{}", x));
}

fn main() {
    let line_break = || println!("-----------");

    basic_version();
    line_break();

    alternate_version();
    line_break();

    match_version();
    line_break();

    iter_version();
}
