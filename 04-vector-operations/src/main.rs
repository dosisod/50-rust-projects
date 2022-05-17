#[derive(Debug, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let mut nums = vec![1, 2, 3, 4];

    print_squares(&nums);
    nums.pop();
    print_squares(&nums);

    let colors = vec![Color::Red, Color::Green, Color::Blue];
    println!("{:?}", colors);

    let colors = filter_non_red_colors(colors);
    println!("{:?}", colors);

    let nums = vec![-1, 0, 1];

    match nums[..] {
        [1, ..] => println!("starts with 1"),
        [.., 2] => println!("ends with 2"),
        [] => println!("empty vector"),
        _ => println!("something else"),
    };

    let new_nums = get_all_but_first(&nums);
    println!("{:?}", nums);
    println!("{:?}", new_nums);

    slice_print(&new_nums);
    println!("{:?}", new_nums);
}

fn print_squares(nums: &Vec<i32>) {
    nums.iter().map(|x| x * x).for_each(|x| println!("{}", x));
}

fn filter_non_red_colors(colors: Vec<Color>) -> Vec<Color> {
    colors
        .clone()
        .into_iter()
        .filter(|color| match color {
            Color::Red => true,
            _ => false,
        })
        .collect::<Vec<_>>()
}

fn get_all_but_first(nums: &Vec<i32>) -> Vec<i32> {
    nums.get(1..).unwrap().to_vec()
}

fn slice_print(nums: &[i32]) {
    for i in nums {
        println!("{}", i);
    }
}
