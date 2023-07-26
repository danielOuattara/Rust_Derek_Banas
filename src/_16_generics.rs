// time @1H:07':45": GENERICS

use std::ops::Add; // Add Trait --> allows to perform "+" operator with generics

// Definition ERROR: missing detail about possibility of '+' in generics
// fn generics_get_sum_0<T>(x: T, y: T) -> T {
//     return x + y;
// }

// solution of previous here
fn generics_get_sum<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

pub fn generics() {
    //----- start generics
    println!("5 + 4 = {}", generics_get_sum(5, 4));
    println!("5.34 + 4.43 = {}", generics_get_sum(5.34, 4.43));
    //----- end generics
}
