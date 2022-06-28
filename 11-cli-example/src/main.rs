use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt()]
struct CliOptions {
    #[structopt(long)]
    name: String,

    #[structopt(short = "n", default_value = "10")]
    repeat: u32,

    #[structopt(long)]
    append: Option<Vec<String>>,
}

fn main() {
    let args = CliOptions::from_args();

    for _ in 0..args.repeat {
        println!("Hello {:?}", args.name);
    }

    if args.append.is_some() {
        for append in args.append.unwrap() {
            println!("{}", append);
        }
    }
}
