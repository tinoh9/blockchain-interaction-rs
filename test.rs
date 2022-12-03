let x: u64 = 100;

let y = 50;
let y = y + 10;

let pair = ('abc', 99);
pair.0;
pair.1;

let couple: (u64, char) = (90, 'a');

let smallest_1 = std::cmp::min(2, 9);

use std::cmp::min;
let smallest_2 = min(2, 9);

use std::cmp::{min, max};
use std::{cmp::min, cmp::max};
use std::cmp::*;

let a = "tino".len();
let b = std::len("tino");

let v1 = Vec::new();
let v2 = std::vec::Vec::new();

use std::prelude::v1::*;

struct items {
    x: f64,
    y: f64,
}

let s = Vec2 {x: 1.0, y: 2.0};
let s2 = Vec2 {y: 4.0, x: 3.0};

let s3 = Vec2 {
    x: 10,0,
    ..s2
}

let s4 = Vec2 {..s3};

fn one_number_1() -> i32 {
    5
}

fn one_number_2() -> i32 {
    return 5;
}

fn dice_roll_1() -> i32 {
    if so_lucky {
        6
    } else {
        1
    }
}

fn dice_roll_2() -> i32 {
    match stat {
        true => 6
        false => 2
    }
}

fn swap(a: i32, b: i32) -> (i32, i32) {
    return (b, a);
}

fn main_swap() {
    let result = swap(456, 789);
    println!("{} {}", result.0, result.1);

    let (x, y) = swap(result.0, result.1);
    println!("{} {}", x, y);
}

fn main_cond() {
    let x = 100;
    if x < 100 {
        println!("x is less than {}", x);
    } else if x == 100 {
        println!("x is {}", x);
    } else {
        println!("x is greater than {}", x);
    }
    
}

fn main_loop() {
    let mut x = 0;
    loop {
        x += 1 ;
        if x == 50 {
            break;
        }
    }
    println!("{}", x);
}

fn main_while() {
    let mut x = 0;
    while x < 50 {
        x += 1;
    }
    println!("{}", x);
}

fn main_iter() {
    for x in 0..10 {
        println!("{}", x);
    }
    for x in 0..=10 {
        println!("{}", x);
    }
}

 fn main_match() {
    let x = 100;
    match x {
        0 => {
            println!("found nothing");
        }
        5 | 10 => {
            println!("found 5 or 10");
        }
        5..=10 => {
            println!("found number 5 to 10 inclusively")
        }
        match_num @ 5..=100 => {
            println!("found {} number from 5 to 100 inclusively", match_num);
        }
        _ => {
            println!("found something else");
        }
    }
 }

 fn main_loop_break() {
    let mut x = 0;
    let a = loop {
        x += 1;
        if x == 50 {
            break "got 50";
        }
    };
    println!("the loop: {}", a);
 }

fn test_1() -> i32 {
    let a = 60;
    let b = if a < 60 {1} else {-1};
    println!("{}", b);

    let food = "pizza";
    let result = match food {
        "pizza" => {"is the pizza"},
        _ => {"is not the pizza"},
    };
    println!("{}", result);

    let b = {
        let x1 = 5;
        let x2 = 10;
        x1 + x2
    };
    println!("{}", b);

    b + 10
}

fn main_test_1() {
    println!("{}", test_1());
}
////
struct CarSpec {
    model: String,
    wheels: i32,
    seats: i32,
    colour: String,
}

fn main_car() {
    let toyota = CarSpec {
        model: String::from("Camry"),
        wheels: 4,
        seats: 4,
        colour: String::from("black"),
    };

    let honda = CarSpec {
        model: String::from("Civic"),
        wheels: 4,
        seats: 4,
        colour: String::from("white"),
    };

    println!(
        "I have a {} model with {} wheels, {} seats and {} colour",
        toyota.model, toyota.wheels, toyota.seats, toyota.colour
    );

    println!(
        "My brother has a {} model with {} wheels, {} seats and {} colour",
        honda.model, honda.wheels, honda.seats, honda.colour
    );
}
////
struct CarSpecAlt(String, i32, i32, String);

fn main_car_1() {
    let toyota = CarSpecAlt("Camry".to_string(), 4, 4, "black".to_string());
    let honda = CarSpecAlt("Civic".to_string(), 4, 4, "white".to_string());

    println!(
        "I have a {} model with {} wheels, {} seats and {} colour",
        toyota.0, toyota.1, toyota.2, toyota.3
    );

    println!(
        "My brother has a {} model with {} wheels, {} seats and {} colour",
        honda.0, honda.1, honda.2, honda.3
    );
}
////
struct CoinBag<T> {
    item: T,
}

fn main_gene() {
    let red_coin = CoinBag::<i32> {item: 10};
    let blue_coin = CoinBag::<i32> {item: 20};
    let red_coin_name = CoinBag::<String> {item: "abc".to_string()};
    let blue_coin_name = CoinBag::<String> {item: "def".to_string()};
    let red_coin_size = CoinBag::<f64> {item: 5.0};
    let blue_coin_size = CoinBag::<f64> {item: 10.0};
    println!(
        "The Coin Bag has {} red coin & {} blue coin, their names are {} & {}, and they have the size of {} & {} respectively",
        red_coin.item, blue_coin.item, red_coin_name.item, blue_coin_name.item, red_coin_size.item, blue_coin_size.item
    )
}
////
struct BookShelf<T> {
    book: Option<T>,
}

fn main_opt() {
    let yellow_book = BookShelf::<i32> {
        book: Some(10)
    };
    if yellow_book.book.is_some() {
        println!("The bookshelf has yellow books")
    } else {
        println!("The bookshelf has none yellow books")
    }

    let yellow_book = BookShelf::<i32> {
        book: None
    };
    if yellow_book.book.is_none() {
        println!("The bookshelf has none yellow books")
    } else {
        println!("The bookshelf has yellow books")
    }

    match yellow_book.book {
        Some(v) => println!("There is {} yellow books in the bookshelf", v),
        None => println!("There is none yellow books in the bookshelf"),
    }
}
////
fn result_fail(a:i32) -> Result<String, String> {
    if a == 100 {
        Ok("100%".to_string())
    } else {
        Err(String::from("This is not correct"))
    }
}

fn main_res() {
    let result = result_fail(50);

    match result {
        Ok(v) => println!("Match: {}", v),
        Err(e) => println!("Error: {}", e),
    }
}
////
fn error_handling(a: i32) -> Result<i32, String> {
    if a == 50 {
        Ok(a)
    } else {
        Err("This is incorrect".to_string())
    }
}

fn main_error_handling() -> Result<(), String> {
    let result = error_handling(10)?;
    println!("found: {}", result);
    Ok(())
} 
////
fn ugly_option(a: i32) -> Result<i32, String> {
    if a == -50 {
        Ok(a)
    } else {
        Err("This is incorrect".to_string())
    }
}

fn main_ugly_option() -> Result<(), String> {
    let result = ugly_option(-50).unwrap();
    println!("found: {}", result);

    let result = ugly_option(-40).unwrap();
    println!("found: {}", result);

    Ok(())
}
////
fn main_vector() {
    let mut vector_1 = Vec::<i32>::new();
        vector_1.push(1);
        vector_1.push(2);
        vector_1.push(3);

    let mut vector_2 = Vec::new();
        vector_2.push(1.0);
        vector_2.push(2.0);
        vector_2.push(3.0);

    let string_vector = vec!["Hi".to_string(), "Good".to_string(), "Morning!".to_string()];
    for word in string_vector.iter() {
        println!("{}", word);
    }
}