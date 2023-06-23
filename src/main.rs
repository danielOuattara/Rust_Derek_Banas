#![allow(unused)]

use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::Add; // Add Trait --> allows to perform "+" operator with generics

//---------------------------------------------------

fn say_hello() {
    println!("Hello world of Rust !");

    println!("Enter your name :");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Did Not Receive Input");

    let greetings: &str = "Nice to meet you";

    println!("Hello {} ! {} ", name.trim_end(), greetings);
}

//----------------------------------------------------

fn constant_variables() {
    const ONE_MILLION: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    // shadowing in Rust: 2 variables with the same name but different type (see below)
    let age = "48";
    println!("age is {}", age);

    let mut age = age
        .trim()
        .parse::<u32>()
        .expect("Age was not assigned a number");
    println!("Age = {}", age);
    age = age + 1;
    println!("new Age = {}", age);

    println!("I'm {} y.o and I have {}€", age, ONE_MILLION);
}

//----------------------------------------------------

fn integer_types() {
    // Unsigned integer: u8, u 16, u32, u64, u128, usize
    // Signed integer: i8, i16, i32, i64, i128, isize

    println!("Max u8 : {}, Min u8 : {}", u8::MAX, u8::MIN);
    println!("Max u16 : {}, Min u16 : {}", u16::MAX, u16::MIN);
    println!("Max u32 : {}, Min u32 : {}", u32::MAX, u32::MIN);
    println!("Max u64 : {}, Min u64 : {}", u64::MAX, u64::MIN);
    println!("Max u128 : {}, Min u128 : {}", u128::MAX, u128::MIN);
    println!("Max usize : {}, Min usize : {}", usize::MAX, usize::MIN);

    println!("-------");

    println!("Max i8 : {}, Min i8 : {}", i8::MAX, i8::MIN);
    println!("Max i16 : {}, Min i16 : {}", i16::MAX, i16::MIN);
    println!("Max i32 : {}, Min i32 : {}", i32::MAX, i32::MIN);
    println!("Max i64 : {}, Min i64 : {}", i64::MAX, i64::MIN);
    println!("Max i128 : {}, Min i128 : {}", i128::MAX, i128::MIN);
    println!("Max isize : {}, Min isize : {}", isize::MAX, isize::MIN);

    println!("-------");

    println!("Max f32 : {}, Min f32 : {}", f32::MAX, f32::MIN);
    println!("Max f64 : {}, Min f64 : {}", f64::MAX, f64::MIN);
}

//----------------------------------------------------

fn data_types() {
    // time @18'30"
    let is_true = true;
    let is_snowing = false;

    let my_grade = 'A';
    println!("my grade is : {}", my_grade);

    let num_1: f32 = 1.1111111111111111111;
    println!("f32 : {}", num_1 + 1.1111111111111111111);

    let num_2: f64 = 1.1111111111111111111;
    println!("f64 : {}", num_2 + 1.1111111111111111111);

    println!("-------");

    let mut num_3: f32 = 5.0;
    let mut num_4: f32 = 4.0;

    println!("5 + 4  = {}", num_3 + num_4);
    println!("5 - 4  = {}", num_3 - num_4);
    println!("5 * 4  = {}", num_3 * num_4);
    println!("5 / 4  = {}", (num_3 / num_4));
    println!("5 ** 4  = {}", num_3.powf(num_4));
    println!("5 % 4  = {}", num_3 % num_4);

    println!("-------");

    let mut num_3: u32 = 5;
    let mut num_4: u32 = 4;

    println!("5 + 4  = {}", num_3 + num_4);
    println!("5 - 4  = {}", num_3 - num_4);
    println!("5 * 4  = {}", num_3 * num_4);
    println!("5 / 4  = {}", (num_3 / num_4));
    println!("5 ** 4  = {}", num_3.pow(num_4));
    println!("5 % 4  = {}", num_3 % num_4);
}

//----------------------------------------------------

fn random_numbers() {
    // time @23'
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random : {}", random_num);

    let random_num_2 = rand::thread_rng().gen::<f64>();
    println!("Random : {}", random_num_2);
}

//----------------------------------------------------

fn if_expression() {
    let age: i8 = 8;
    if age >= 1 && age <= 18 {
        println!("Important birthday, section 1");
    } else if age == 21 || age == 50 {
        println!("Important birthday section 2");
    } else if age >= 65 {
        println!("Important birthday, section 3");
    } else {
        println!("Not Important birthday");
    }
}

//----------------------------------------------------

fn ternary_operator() {
    // time @26'
    let mut my_age = 47;

    let can_vote = if my_age >= 18 { true } else { false };
    println!("Can you vote ? {} ", can_vote);

    let can_vote = my_age >= 18;
    println!("Can you vote ? {} ", can_vote);
}

//----------------------------------------------------

fn match_operator() {
    // time @27'
    let mut my_age: i8 = 48;
    match my_age {
        1..=18 => println!("Important Birthday, section 1"),
        21 | 50 => println!("Important Birthday, section 2"),
        65..=i8::MAX => println!("Important Birthday, section 3"),
        _ => println!("Not Important Birthday"),
    };

    let my_age = 18;
    let voting_age = 21;
    match my_age.cmp(&voting_age) {
        Ordering::Equal => println!("you can vote"),
        Ordering::Greater => println!("you can vote"),
        Ordering::Less => println!("you can NOT vote"),
    };
}
//----------------------------------------------------

fn array_operator() {
    // time @32'
    let arr_1 = [1, 2, 3, 4];

    println!("first item in arr_1 is : {}", arr_1[0]);
    println!("length of arr_1 is : {}", arr_1.len());
    println!("--------------------");

    // loop in array
    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_index = 0;
    loop {
        println!("index = {} , value = {}", loop_index, arr_2[loop_index]);
        if loop_index + 1 == arr_2.len() {
            println!("-------------------- Done !");
            break;
        }
        loop_index += 1;
    }

    //-------

    let arr_3 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_index = 0;
    loop {
        if loop_index + 1 == arr_3.len() {
            println!("------------------- Done !");
            break;
        }
        if arr_2[loop_index] % 2 == 0 {
            loop_index += 1;
            continue;
        }

        println!("index = {} , value = {}", loop_index, arr_2[loop_index]);
        loop_index += 1;
    }

    //-----------

    let arr_4 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_index = 0;

    for item in arr_4 {
        println!("index = {} , value = {}", loop_index, arr_4[loop_index]);
        if loop_index + 1 == arr_4.len() {
            println!("-------------------- Done !");
            break;
        }
        loop_index += 1;
    }

    //-------

    let arr_5 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_index = 0;

    for item in arr_5 {
        if loop_index + 1 == arr_5.len() {
            println!("------------------- Done !");
            break;
        }
        if arr_5[loop_index] % 2 == 0 {
            loop_index += 1;
            continue;
        }
        println!("index = {} , value = {}", loop_index, arr_5[loop_index]);
        loop_index += 1;
    }
    //-----------

    let arr_6 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_index = 0;

    while loop_index + 1 < arr_6.len() {
        println!("index = {} , value = {}", loop_index, arr_6[loop_index]);
        loop_index += 1;
    }
    println!("-------------------- Done !");

    //-------

    let arr_7 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_index = 0;

    while loop_index + 1 < arr_7.len() {
        if arr_7[loop_index] % 2 == 0 {
            loop_index += 1;
            continue;
        }
        println!("index = {} , value = {}", loop_index, arr_7[loop_index]);
        loop_index += 1;
    }
    println!("------------------- Done !");
}

//----------------------------------------------------
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
    // say_hello();
    // constant_variables();
    // integer_types();
    // data_types();
    // random_numbers();
    // if_expression();
    // ternary_operator();
    // match_operator();
    // array_operator();
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
    println!("5 + 4 = {}", generics_get_sum(5, 4));
    println!("5.34 + 4.43 = {}", generics_get_sum(5.34, 4.43));
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

    let str_1 = String::from("World");
    let str_2 = String::from("World");

    // stop @01H15:17

    //--------- end ownership
}
