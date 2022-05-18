use std::fs::File;
use std::io::Read;
use std::path::Path;

fn run_app() -> Result<(), std::io::Error> {
    for filename in std::env::args().skip(1) {
        let path = Path::new(&filename);

        let mut file = File::open(&path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        print!("{}", contents);
    }

    Ok(())
}

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(x) => {
            eprintln!("{}", x);
            1
        }
    });
}
