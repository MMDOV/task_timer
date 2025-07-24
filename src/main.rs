use std::env;
use std::io;
use std::io::prelude::*;
use std::time;

fn main() {
    let args: Vec<String> = env::args().collect();
    let descripion = args[1].clone();
    let now = time::Instant::now();

    pause(&descripion);

    let time_spent = now.elapsed().as_secs();
    let time_spent_minutes = time_spent / 60;
    let time_spent_seconds = time_spent - (time_spent_minutes * 60);
    println!(
        "Done: {descripion}\nDuration: {time_spent_minutes} minutes and {time_spent_seconds} seconds"
    );
}

fn pause(description: &str) {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "{description}\nPress Enter To End").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}
