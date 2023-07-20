#![allow(unused)]

use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::Add; // Add Trait --> allows to perform "+" operator with generics

//---------------------------------------------------
mod _01_user_input;
mod _02_constant_variables;
mod _03_integer_types;
mod _04_data_type;
mod _05_random_numbers;
mod _06_if_expressions;
mod _07_ternary;
mod _08_match;
mod _09_arrays_and_operations;

// @tuples 39'20"

fn tuples() {
    let my_tuples: (u8, String, f64) = (39, "Daniel".to_owned(), 20.34434);
    println!("Name: {}", my_tuples.1);

    let (v0, v1, v2) = my_tuples;
    println!("Age: {:?}", v0);
}
//----------------------------------------------------
// time @41'30"

fn strings() {
    let mut string_1 = String::new();
    string_1.push('D');
    string_1.push_str("aniel Ouat Tara");

    println!("string_1 : {:?}", string_1);

    for word in string_1.split_whitespace() {
        println!("{}", word);
    }

    let string_2 = string_1.replace("Ouat Tara", "Ouattara");
    println!("string_2 : {:?}", string_2);

    let string_3 = String::from("a a a  z e r t y u i o p");
    println!("string_3 :{}", string_3);

    let mut vector_1: Vec<char> = string_3.chars().collect();
    vector_1.sort();
    vector_1.dedup();

    println!("vector _1 : {:?}", vector_1);

    let string_4: &str = "Random String";

    let mut string_5: String = string_4.to_string();
    println!("string_5 : {}", string_5);

    let byte_array_1 = string_5.as_bytes();

    let string_6 = &string_5[0..6];
    println!(
        "string_6 : {}, its length is {:?} ",
        string_6,
        string_6.len()
    );

    string_5.clear();
    println!(
        "empty string_5 is {}, its length is {:?} ",
        string_5,
        string_5.len()
    );

    let string_6 = String::from("Just some");
    let string_7 = String::from(" another string");

    let string_8 = string_6 + &string_7;
    println!("string_8 : {}", string_8);

    for char in string_8.bytes() {
        print!(" {}", char);
    }
}
//----------------------------------------------------
// time @50'28"

fn casting() {
    let int_1_u8: u8 = 5;
    let int_2_u8: u8 = 4;

    // casting using 'as'
    let int_3_u32: u32 = (int_1_u8 as u32) + (int_2_u8 as u32);
}

//----------------------------------------------------
// time @52'
fn enum_type() {
    enum Days {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Days {
        fn is_weekend(&self) -> bool {
            match self {
                Days::Saturday | Days::Sunday => true,
                _ => false,
            }
        }
    }

    // ------

    let today: Days = Days::Thursday;
    println!("is today a weekend day ?: {}", today.is_weekend());

    let tomorrow: Days = Days::Friday;
    println!("is tomorrow a weekend day ?: {}", tomorrow.is_weekend());

    let day_after_tomorrow: Days = Days::Saturday;
    println!(
        "is the day after tomorrow a weekend day ?: {}",
        day_after_tomorrow.is_weekend()
    );

    //------

    let today = Days::Friday;

    match today {
        Days::Monday => println!("Monday"),
        Days::Tuesday => println!("Tuesday"),
        Days::Wednesday => println!("Wednesday"),
        Days::Thursday => println!("Thursday"),
        Days::Friday => println!("Friday"),
        Days::Saturday => println!("Saturday"),
        Days::Sunday => println!("Sunday"),
        _ => println!("Where are yoy from ?"),
    }

    //------

    println!("Is today hte weekend ?: {:?}", today.is_weekend());

    let now = Utc::now();
    println!("{}", now);
}

//----------------------------------------------------
// time @55'50"

fn vectors() {
    //empty vector
    let vector_1: Vec<i32> = Vec::new();

    // mutable vector
    let mut vector_2 = vec![1, 2, 3, 4];

    vector_2.push(5);

    println!("1st item: {}", vector_2[0]);

    let vector_2_second_item = &vector_2[1];
    match vector_2.get(1) {
        Some(second) => println!("vector_2 2nd item : {}", vector_2_second_item),
        None => println!("No 2nd value found"),
    }

    // multiply all items in vector_2, just in printing
    for item in &vector_2 {
        println!("{}", item * 2)
    }

    println!("------------");

    for item in &vector_2 {
        println!("{}", item)
    }

    println!("------------");

    // multiply all items in vector_2, permanently
    for item in &mut vector_2 {
        *item *= 2;
    }

    // check
    for item in &vector_2 {
        println!("{}", item)
    }

    // Vector 2 length

    println!("vector_2 length = {:?}", vector_2.len());

    println!("pop of on vector_2  = {:?}", vector_2.pop());

    println!("vector_2 length = {:?}", vector_2.len());
}

//----------------------------------------------------
// time @1H:00':40": FUNCTIONS

fn _function() {
    println!("Say Hello");
}

fn get_sum(x: i32, y: i32) {
    println!("{} + {} = {} ", x, y, x + y);
}

fn get_sum_2(x: i32, y: i32) -> i32 {
    x + y
}

fn get_sum_3(x: i32, y: i32) -> i32 {
    return x + y;
}

fn multiple_returns(x: i32) -> (i32, i32) {
    return (x * 2, x * 30);
}

fn sum_list_item_1(list: &[i32]) -> i32 {
    let mut sum = 0;
    for item in list {
        sum += item
    }
    return sum;
}

fn sum_list_item_2(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &item in list.iter() {
        sum += &item
    }
    return sum;
}

//----------------------------------------------------
// time @1H:07':49": GENERICS

fn generics_get_sum<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

//----------------------------------------------------

//----------------------------------------------------

/*  Stopped time @1:   */

fn main() {
    _01_user_input::say_hello();
    // _02_constant_variables::constant_variables()
    // _03_integer_types::integer_types();
    // _04_data_type::data_types();
    // _05_random_numbers::random_numbers();
    // _06_if_expressions::if_expression();
    // _07_ternary::ternary_operator();
    // _08_match::match_operator();
    _09_arrays_and_operations::array_operator();

    // tuples();
    // strings();
    // casting();
    // enum_type();
    // vectors();
    // _function();
    // get_sum(1, 4);
    // println!(" {:?} ", get_sum_2(1, 4));
    // println!(" {:?} ", get_sum_3(1, 4));
    // println!(" {:?} ", multiple_returns(4));
    // let (value_1, value_2) = multiple_returns(5);

    // println!("Items from multiple_return(5): {},{}", value_1, value_2);

    // let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    // println!("Sum of items in list = {}", sum_list_item_1(&list));
    // println!("Sum of items in list = {}", sum_list_item_2(&list));
    // ---- end of functions

    //----- start generics
    // println!("5 + 4 = {}", generics_get_sum(5, 4));
    // println!("5.34 + 4.43 = {}", generics_get_sum(5.34, 4.43));
    //----- end generics

    //--------- start ownership
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

    */

    // let str_1 = String::from("World");
    // let str_2 = String::from("World");

    // stop @01H15:17

    //--------- end ownership
}
