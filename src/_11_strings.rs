// time @41'30"

pub fn strings() {
    // empty growable string:
    let mut string_1 = String::new();

    string_1.push('D'); // pushing character at the end
    string_1.push_str("aniel Ouat Tara"); // pushing string at the end

    println!("string_1 : {:?}", string_1);

    // iterate on each string word
    for word in string_1.split_whitespace() {
        println!("{}", word);
    }

    // replace string
    let string_2 = string_1.replace("Ouat Tara", "Ouattara");
    println!("string_2 : {:?}", string_2);

    // string from random characters
    let string_3 = String::from("a a a  z e r t y u i o p");
    println!("string_3 :{}", string_3);

    // convert previous String to Vector
    let mut vector_1: Vec<char> = string_3.chars().collect();
    println!("vector _1 : {vector_1:?}");

    vector_1.sort();
    println!("vector _1 : {vector_1:?}");

    vector_1.dedup();
    println!("vector _1 : {vector_1:?}");

    // create a string literal
    let string_4: &str = "Random String";

    // convert previous string to hype allocated string: String
    let mut string_5: String = string_4.to_string();
    println!("string_5 : {}", string_5);

    // convert String to array of bytes
    let byte_array_1 = string_5.as_bytes();
    println!("string_5 : {}", string_5);

    // slice a String from 0 to 6 excluded
    let string_6 = &string_5[0..6];
    println!("string_6  {:?} ", string_6,);

    // get String length
    println!("string_6 length is {:?} ", string_6.len());

    // clear String if mutable
    string_5.clear();
    println!(
        "empty string_5 is {}, its length is {:?} ",
        string_5,
        string_5.len()
    );

    // create String again
    let string_7 = String::from("Just some");

    // create String again
    let string_8 = String::from(" another string");

    // combine Strings
    let string_9 = string_7 + &string_8;

    // No borrow (&) on string_7: so it is completely "absorbed" in string_9; check by display
    //println!("string_7 : {}", string_7); // Error: borrow of moved value: `string_7`

    // string_8 is still exist. Can be used again
    println!("string_8 : {}", string_8); // OK

    // cycle through letter in string and print them out as unicode.
    for char in string_9.bytes() {
        print!(" {}", char);
    }
}
