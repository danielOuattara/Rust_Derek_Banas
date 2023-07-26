// time @1H:00':34": FUNCTIONS

fn say_hello() {
    println!("Say Hello");
}

fn get_sum(x: i32, y: i32) {
    println!("{x} + {y} = {} ", x + y);
}

fn get_sum_2(x: i32, y: i32) -> i32 {
    x + y
}

fn get_sum_3(x: i32, y: i32) -> i32 {
    return x + y;
}

fn multiple_returns(x: i32) -> (i32, i32) {
    (x * 2, x * 30)
}
fn multiple_returns_2(x: i32) -> (i32, i32) {
    return (x * 2, x * 30);
}

fn sum_list_item_1(list: &[i32]) -> i32 {
    let mut sum = 0;
    for item in list {
        sum += item;
    }
    return sum;
}

//OR

fn sum_list_item_2(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &item in list.iter() {
        sum += &item;
    }
    return sum;
}

pub fn functions_global() {
    say_hello();
    get_sum(1, 4);
    println!(" {:?} ", get_sum_2(1, 4));
    println!(" {:?} ", get_sum_3(1, 4));

    println!(" {:?} ", multiple_returns(4));
    let (value_1, value_2) = multiple_returns(5);

    println!(" {:?} ", multiple_returns_2(5));
    let (value_1, value_2) = multiple_returns_2(5);

    println!("Items from multiple_return(5): {},{}", value_1, value_2);

    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("Sum of items in list = {}", sum_list_item_1(&list));
    println!("Sum of items in list = {}", sum_list_item_2(&list));
}
