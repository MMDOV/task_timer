use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::Error;
use std::io::Write;
use std::io::prelude::*;
use std::time;

#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
    time_spent: u64,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let descripion = args[1].clone();
    let now = time::Instant::now();

    pause(&descripion);

    let time_spent = now.elapsed().as_secs();

    let task = Task {
        description: descripion.to_owned(),
        time_spent: time_spent.to_owned(),
    };

    match save_to_json(&task) {
        Ok(_) => (),
        Err(_) => println!("Error trying to save to json"),
    }

    let time_spent_minutes = task.time_spent / 60;
    let time_spent_seconds = task.time_spent - (time_spent_minutes * 60);
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

fn save_to_json(task: &Task) -> Result<(), Error> {
    let json_file = serde_json::json!(&task);
    let mut tasks: HashMap<String, Value> = if let Ok(mut file) = File::open("tasks.json") {
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        serde_json::from_str(&contents).unwrap_or_default()
    } else {
        HashMap::new()
    };

    tasks.insert(task.description.to_string(), json_file);

    let json_string = serde_json::to_string_pretty(&tasks);
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("tasks.json");
    let result = file?.write_all(json_string?.as_bytes());
    result
}
