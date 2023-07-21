// @tuples 39'27"

pub fn tuples() {
    let my_tuples: (u8, String, f64) = (39, "John Doe".to_owned(), 20.34434);
    println!("Name: {}", my_tuples.1);

    let (v0, v1, v2) = my_tuples;
    println!("Age: {:?}", v0);
}
