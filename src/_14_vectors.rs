// time @55':57"'

pub fn vectors() {
    //empty vector
    let vector_1: Vec<i32> = Vec::new();

    // mutable vector
    let mut vector_2 = vec![1, 2, 3, 4];

    vector_2.push(5);

    println!("1st item: {}", vector_2[0]);

    //-------------------------------------

    let vector_2_second_item = &vector_2[1];

    match vector_2.get(1) {
        Some(second) => println!("vector_2 2nd item : {}", vector_2_second_item),
        None => println!("No 2nd value found"),
    }

    //-------------------------------------

    // multiply all items in vector_2, just in printing
    for item in &vector_2 {
        println!("{}", item * 11)
    }

    println!("------------");

    // multiply all items in vector_2, permanently
    for item in &mut vector_2 {
        *item *= 3;
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
