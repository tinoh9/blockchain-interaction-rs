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