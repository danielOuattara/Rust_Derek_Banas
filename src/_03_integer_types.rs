// @ time 00h:15:40

pub fn integer_types() {
    // Unsigned integer: u8, u 16, u32, u64, u128, usize
    // Signed integer: i8, i16, i32, i64, i128, isize

    println!("Max u8 : {}, Min u8 : {}", u8::MAX, u8::MIN);
    println!("Max u16 : {}, Min u16 : {}", u16::MAX, u16::MIN);
    println!("Max u32 : {}, Min u32 : {}", u32::MAX, u32::MIN);
    println!("Max u64 : {}, Min u64 : {}", u64::MAX, u64::MIN);
    println!("Max u128 : {}, Min u128 : {}", u128::MAX, u128::MIN);
    println!("Max usize : {}, Min usize : {}", usize::MAX, usize::MIN);

    println!("-------");

    println!("Max i8 : {}, Min i8 : {}", i8::MAX, i8::MIN);
    println!("Max i16 : {}, Min i16 : {}", i16::MAX, i16::MIN);
    println!("Max i32 : {}, Min i32 : {}", i32::MAX, i32::MIN);
    println!("Max i64 : {}, Min i64 : {}", i64::MAX, i64::MIN);
    println!("Max i128 : {}, Min i128 : {}", i128::MAX, i128::MIN);
    println!("Max isize : {}, Min isize : {}", isize::MAX, isize::MIN);

    println!("-------");

    println!("Max f32 : {}, Min f32 : {}", f32::MAX, f32::MIN);
    println!("Max f64 : {}, Min f64 : {}", f64::MAX, f64::MIN);
}
