#![allow(unused)]

use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::Add; // Add Trait --> allows to perform "+" operator with generics

//---------------------------------------------------
// mod _01_user_input;
// mod _02_constant_variables;
// mod _03_integer_types;
// mod _04_data_type;
// mod _05_random_numbers;
// mod _06_if_expressions;
// mod _07_ternary;
// mod _08_match;
// mod _09_arrays_and_operations;
// mod _10_tuples;
// mod _11_strings;
// mod _12_casting;
// mod _13_enums;
// mod _14_vectors;
// mod _15_functions;
// mod _16_generics;
mod _17_ownership;

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
    // _12_casting::casting();
    // _13_enums::enum_type();
    // _14_vectors::vectors();
    // _15_functions::functions_global();
    // _16_generics::generics();
    _17_ownership::ownership();
}
