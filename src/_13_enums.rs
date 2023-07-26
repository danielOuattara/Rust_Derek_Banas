// time @52'

use std::arch::x86_64::_MM_FROUND_NEARBYINT;

use chrono::{DateTime, Duration, Utc};

pub fn enum_type() {
    enum DaysOfWeek {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl DaysOfWeek {
        fn is_weekend(&self) -> bool {
            match self {
                DaysOfWeek::Saturday | DaysOfWeek::Sunday => true,
                _ => false,
            }
        }
    }

    // ------

    let today: DaysOfWeek = DaysOfWeek::Thursday;
    println!("is today a weekend day ?: {}", today.is_weekend());

    let tomorrow: DaysOfWeek = DaysOfWeek::Friday;
    println!("is tomorrow a weekend day ?: {}", tomorrow.is_weekend());

    let day_after_tomorrow: DaysOfWeek = DaysOfWeek::Saturday;
    println!(
        "is the day after tomorrow a weekend day ?: {}",
        day_after_tomorrow.is_weekend()
    );

    //------

    let today = DaysOfWeek::Friday;

    match today {
        DaysOfWeek::Monday => println!("Monday"),
        DaysOfWeek::Tuesday => println!("Tuesday"),
        DaysOfWeek::Wednesday => println!("Wednesday"),
        DaysOfWeek::Thursday => println!("Thursday"),
        DaysOfWeek::Friday => println!("Friday"),
        DaysOfWeek::Saturday => println!("Saturday"),
        DaysOfWeek::Sunday => println!("Sunday"),
        _ => println!("Where are you from ?"),
    }

    //-----------

    println!("Is today the weekend ?: {:?}", today.is_weekend());

    //------------

    let now = Utc::now();
    println!("{}", now);
    println!("{:?}", now.format("%d/%m/%Y %H:%M").to_string());
}
