use std::io::{self, Write};
use std::{env, process};

use smallvec::SmallVec;

mod cli;

fn main() {
    let ns: SmallVec<[u128; 10]> = match env::args().skip(1).map(|s| cli::parse_u128(&s)).collect()
    {
        Ok(ns) => ns,
        Err(err) => {
            eprintln!("error: {}", err);
            process::exit(1);
        }
    };

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    for n in ns {
        let fib = fib::fibonacci(n);
        if let Err(err) = writeln!(stdout, "{}", fib) {
            panic!("failed writing to stdout: {}", err);
        }
    }
}
