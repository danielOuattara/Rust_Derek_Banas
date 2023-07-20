// @ time 00h:24:10

pub fn if_expression() {
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
