// time @1H:11:40 OWNERSHIP

/*
Stack : Stores values in a last in / first out format
        Data on the stack must have a defined fixed size

Heap : When putting data on the heap you request a certain
       amount of space. The OS finds space available and
       returns an address for that space called a pointer

Rules
1. each value has a variable that is called its owner
2. there is only one owner at a time for a value
3. when the owner goes out of scope the value disappears


Automatic deallocation of memory by Rust compiler is great
but it can sometime bring problem:

*/

fn print_string(x: String) {
    println!(" 1 : A string {}", x);
}

fn print_string_return(string: String) -> String {
    println!(" 2 : A string {}", string);
    return string;
}

fn print_string_completed_and_return(string: String) -> String {
    return String::from(" 3 : A string from ") + &string.to_owned();
}

//-------------------------------------

fn print_string_updated(x: &mut String) {
    x.push_str(" is happy");
    println!(" 4 :A string {:?}", x);
}

fn print_return_string_updated(string: &mut String) -> String {
    println!(" 5 : A string {:?}", string.push_str(" can fly !"));
    return string.to_string();
}

fn print_return_string_completing_and_return(string: &mut String) -> String {
    return String::from(" 6 : A string from ") + &string.to_owned();
}

// fn change_string_value(string: &mut String)

pub fn ownership() {
    let str_1 = String::from("World");

    // let str_2 = str_1;
    // println!("Hello {}", str_1); // ERROR  borrow of moved value: `str_1` value borrowed here after move

    // OK
    let str_2 = &str_1;
    println!("Hello {}", str_1);

    // OK
    let str_2 = str_1.clone();
    println!("Hello {}", str_1);

    println!("-------------------------------");

    print_string("hello".to_owned());

    print_string_return("Rust".to_owned());
    println!("{}", print_string_return("Mario".to_owned()));

    print_string_completed_and_return("Ninja".to_owned());
    println!(
        "{}",
        print_string_completed_and_return("Donald duck".to_owned())
    );

    println!("-------------------------------");

    let mut donald = String::from("Donald");

    print_string_updated(&mut donald);

    println!(
        "{}",
        print_string_completed_and_return("Donald duck".to_owned())
    );
}
