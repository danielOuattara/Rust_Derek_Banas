// @ time 00h:18:30

pub fn data_types() {
    // time @18'30"
    let is_true = true;
    let is_snowing = false;

    let my_grade = 'A';
    println!("my grade is : {}", my_grade);

    let num_1: f32 = 1.1111111111111111111;
    println!(
        "f32 : 1.1111111111111111111 + 1.1111111111111111111 =  {}",
        num_1 + 1.1111111111111111111
    );

    let num_2: f64 = 1.1111111111111111111;
    println!(
        "f64 : 1.1111111111111111111 + 1.1111111111111111111 =  {}",
        num_2 + 1.1111111111111111111
    );

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

    /*
    Other operators :
    +=
    -=
    *=
    /=
    %=
    ...etc
    */

    println!("-------");

    let x = 2.0_f32;
    let abs_difference = (x.powf(2.0) - (x * x)).abs();

    assert!(abs_difference <= f32::EPSILON);
    assert!(1 == 1); // OK
}
