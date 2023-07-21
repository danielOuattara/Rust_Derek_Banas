// @ time 00h:32:40

pub fn array_operator() {
    //
    let arr_1 = [1, 2, 3, 4];
    println!("first item in arr_1 is : {}", arr_1[0]);
    println!("length of arr_1 is : {}", arr_1.len());
    println!("--------------------");

    // loop in array
    //
    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_index = 0;

    println!("arr_2");

    loop {
        println!("index = {loop_index} , value = {}", arr_2[loop_index]);
        if loop_index + 1 == arr_2.len() {
            println!("-------------------- Done !");
            break;
        }
        loop_index += 1;
    }

    //-------

    println!("arr_3");

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

        println!("index = {} , value = {loop_index}", arr_2[loop_index]);
        loop_index += 1;
    }

    //------------------
    // for loop in array
    //

    println!("arr_4");

    let arr_4 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_index = 0;

    for item in arr_4.iter() {
        println!("index = {} , value = {loop_index}", arr_4[loop_index]);
        if loop_index + 1 == arr_4.len() {
            println!("-------------------- Done !");
            break;
        }
        loop_index += 1;
    }

    //-------

    println!("arr_5");

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
    //------------------
    // while loop in array
    //

    println!("arr_6");

    let arr_6 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_index = 0;

    while loop_index + 1 < arr_6.len() {
        println!("index = {} , value = {}", loop_index, arr_6[loop_index]);
        loop_index += 1;
    }
    println!("-------------------- Done !");

    //-------

    println!("arr_7");

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
