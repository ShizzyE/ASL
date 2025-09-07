use chrono::{Local, Utc};

fn main() {
    let greeting: &str = "Hello ASL!";
    println!("{}", greeting);

    let time_local = Local::now();
    println!("Date: {}", time_local);
}