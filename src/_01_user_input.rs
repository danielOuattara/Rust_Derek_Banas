use std::io;

// @time : 00h07'
pub fn say_hello() {
    println!("Hello world of Rust !\n");

    println!("Enter your name :");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Did Not Receive Input");

    let greetings: &str = "Nice to meet you";

    println!("Hello {} ! {} ", name.trim_end(), greetings);
}
