// @ time 00h:27:51

use std::cmp::Ordering;

pub fn match_operator() {
    // time @27'
    let mut my_age: i8 = 48;
    match my_age {
        1..=18 => println!("Important Birthday, section 1"),
        21 | 50 => println!("Important Birthday, section 2"),
        65..=i8::MAX => println!("Important Birthday, section 3"),
        _ => println!("Not Important Birthday"),
    };

    //--------------------------------------

    let my_age = 18;
    let voting_age = 21;
    match my_age.cmp(&voting_age) {
        Ordering::Equal => println!("you can vote"),
        Ordering::Greater => println!("you can vote"),
        Ordering::Less => println!("you can NOT vote"),
    };
}
