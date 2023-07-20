// @ time 00h:26:30

pub fn ternary_operator() {
    // time @26'
    let mut my_age = 47;

    let can_vote = if my_age >= 18 { true } else { false };
    println!("Can you vote ? {} ", can_vote);

    let can_vote = my_age >= 18;
    println!("Can you vote ? {} ", can_vote);
}
