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
mod _10_tuples;
mod _11_strings;
mod _12_casting;
mod _13_enums;

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
    // _01_user_input::say_hello();
    // _02_constant_variables::constant_variables()
    // _03_integer_types::integer_types();
    // _04_data_type::data_types();
    // _05_random_numbers::random_numbers();
    // _06_if_expressions::if_expression();
    // _07_ternary::ternary_operator();
    // _08_match::match_operator();
    // _09_arrays_and_operations::array_operator();
    // _10_tuples::tuples();
    // _11_strings::strings();
    _12_casting::casting();
    _13_enums::enum_type();

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
