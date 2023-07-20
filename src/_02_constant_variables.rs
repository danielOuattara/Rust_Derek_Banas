// @ time 00h:12:30'

pub fn constant_variables() {
    const ONE_MILLION: u32 = 1_000_000;
    const PI: f32 = 3.141592;

    // shadowing in Rust: 2 variables with the same name but different type (see below)
    let age: &str = "48";
    println!("age is {}", age);

    let mut age: u32 = age
        .trim()
        .parse::<u32>()
        .expect("Age was not assigned a number");

    println!("Age = {}", age);
    //
    age = age + 1;
    println!("new Age = {}", age);

    println!("I'm {} y.o and I have {}€", age, ONE_MILLION);
}
