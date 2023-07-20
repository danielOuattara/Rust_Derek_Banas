// @ time 00h:23:09

use rand::Rng;

use rand::prelude::*;

pub fn random_numbers() {
    // time @23'
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random : {}", random_num);

    let random_num_2 = rand::thread_rng().gen::<f64>();
    println!("Random : {}", random_num_2);

    let random_num_3 = rand::thread_rng().gen::<u8>();
    println!("Random : {}", random_num_3);

    //-------------------------------------

    if rand::random() {
        // generates a boolean
        // Try printing a random unicode code point (probably a bad idea)!
        println!("char: {}", rand::random::<char>());
    }

    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen(); // generates a float between 0 and 1

    let mut nums: Vec<i32> = (1..100).collect();
    nums.shuffle(&mut rng);
}
