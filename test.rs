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

fn main_con() {
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

