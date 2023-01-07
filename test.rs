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
////
struct Fruit {
    fruit: String,
}

struct Table {
    basket: Fruit,
}

fn main_drop() {
    let table = Table {basket: Fruit {fruit: "apple".to_string()}};
    println!("{}", table.basket.fruit);
}
////
struct Car {
    bmw: String,
}

fn owner(c: Car) {
    println!("{}", c.bmw);
}

fn main_move() {
    let car = Car {bmw: "M3".to_string()};
    owner(car);
}
////
struct Pen {
    pen: i32,
}

fn main_borrow() {
    let a = Pen {pen: 5};
    let b = &a;
    println!("{}", b.pen);
}
////
struct Material {
    material: String,
}

fn owner(a: Material) {
    println!("{}", a.material);
}

fn main_mut_borrow() {
    let mut b = Material {material: "wood".to_string()};
    let a = &mut b;
    a.material = "steel".to_string();
    println!("{}", b.material);
    b.material = "glass".to_string();
    owner(b);
}
////
fn main_deref() {
    let mut a = 50;
    let b = &mut a;
    let c = *b;
    *b = 100;
    println!("{}", c);
    println!("{}", a);
}
////
fn main_concat_join() {
    let goodmorning = ["Good", " ", "Morning", "!"].concat();
    let goodbye = ["Good", "Bye", "!"].join("_");
    println!("{}", goodmorning);
    println!("{}", goodbye);
}
////
struct Animal {
    cat: u32,
    pub dog: u32,
}

impl Animal {
    pub fn main_pub_method(&self) -> &u32 {
        &self.cat
    }
}

fn main_encap() {
    let owner = Animal {cat: 5};
    println!("{}", owner.main_pub_method());
}
//// INCOMPLETED
use std::io;
use rand::{Rng::distributions::{Distribution, Standard}};

struct Alphabet {
    answer: char;
}

fn guess_alphabet() {
    println!("Welcome to the game");
    println!("Please type any alphabet");
    let mut guess = String::new();
    let answer = 'a';
    impl Distribution<char> for Alphabet {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
            "abcdefghijklmnopqrstuvwxyz".choose(rng).unwrap() as char
        }
    }
    io::stdin().read_line =  (&mut guess).unwrap();
    println!("You guessed: {}", guess)
    if guess = 'a' {
        println!("That's correct!");
    } else {
        println!("That's incorrect, please type another alphabet");
    }
}
////
fn main_pointer() {
    let x = -1.5;
    let memory_location = &x as *const f64 as usize;
    println!("X has the data of {}", memory_location);
}
////
struct Animal {
    chicken: u32
}

impl Animal {
    fn heap_chicken(&self) -> &u32 {
        &self.chicken
    } 
}

fn main() {
    let box_chicken = Box::new(Animal {chicken: 5});
    println!("There are {} chickens moved from the stack to the heap", box_chicken.heap_chicken());
}

// Calculate an area of a triangle
fn main() {
    println!("Please enter the base of the triangle:");
    let mut base = String::new();
    std::io::stdin()
        .read_line(&mut base)
        .expect("Failed to read the number");
    let base: usize = base.trim().parse().expect("Please enter the number");

    println!("Please enter the height of the triangle:");
    let mut height = String::new();
    std::io::stdin()
        .read_line(&mut height)
        .expect("Failed to read the number");
    let height: usize = height.trim().parse().expect("Please enter the number");

    let area: usize = (base * height) / 2;
    println!("The area of the triangle is {}", area);
}

// To-do list management
struct Task {
    title: String,
    description: String,
    completed: bool,
}

fn main() {
    let mut tasks = Vec::new();

    loop {
        println!("1. Add a task");
        println!("2. Mark a task as completed");
        println!("3. View all tasks");
        println!("4. View only incomplete tasks");
        println!("5. View only completed tasks");
        println!("6. Delete a task");
        println!("7. Exit");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        match input {
            "1" => add_task(&mut tasks),
            "2" => mark_task_completed(&mut tasks),
            "3" => view_tasks(&tasks),
            "4" => view_incomplete_tasks(&tasks),
            "5" => view_completed_tasks(&tasks),
            "6" => delete_task(&mut tasks),
            "7" => break,
            _ => println!("Invalid input"),
        }
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    println!("Enter the title of the task:");
    let mut title = String::new();
    std::io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");
    let title = title.trim();

    println!("Enter the description of the task:");
    let mut description = String::new();
    std::io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");
    let description = description.trim();

    let task = Task {
        title: title.to_string(),
        description: description.to_string(),
        completed: false,
    };
    tasks.push(task);
}

fn mark_task_completed(tasks: &mut Vec<Task>) {
    println!("Enter the index of the task to mark as completed:");
    let mut index = String::new();
    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    if index < tasks.len() {
        tasks[index].completed = true;
    } else {
        println!("Invalid task index");
    }
}

fn view_tasks(tasks: &Vec<Task>) {
    for (index, task) in tasks.iter().enumerate() {
        println!("{}. {} - {}", index, task.title, task.description);
    }
}

fn view_incomplete_tasks(tasks: &Vec<Task>) {
    for (index, task) in tasks.iter().enumerate() {
        if !task.completed {
            println!("{}. {} - {}", index, task.title, task.description);
        }
    }
}

fn view_completed_tasks(tasks: &Vec<Task>) {
    for (index, task) in tasks.iter().enumerate() {
        if task.completed {
            println!("{}. {} - {}", index, task.title, task.description);
        }
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    println!("Enter the index of the task to delete:");
    let mut index = String::new();
    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    if index < tasks.len() {
        tasks.remove(index);
        println!("Task deleted successfully");
    } else {
        println!("Invalid task index");
    }
}

// Read and count the total number of characters in a file (error handling, read filesystem)
use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(From::from("missing argument: file name"));
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename)?;

    let mut count = 0;
    for line in content.lines() {
        count += line.len();
    }

    println!("Total number of characters in {}: {}", filename, count);

    Ok(())
}

// SOME OF FEATURES IN RUST
fn main() {
    // Declare a variable binding
    let x = 5;

    // The type of `x` is inferred to be `i32`
    println!("The value of x is: {}", x);

    // Mutable variables are declared with the `mut` keyword
    let mut y = 10;
    y = 20;
    println!("The value of y is: {}", y);

    // Shadowing allows us to re-use a variable's name
    // while giving it a new type or value
    let y = "hello";
    println!("The value of y is now: {}", y);

    // Rust has a number of primitive types, including
    // integers, floating-point numbers, Booleans, and characters
    let a: i32 = 5;
    let b: f64 = 3.14;
    let c: bool = true;
    let d: char = 'a';

    // Rust also has a number of compound types, including
    // tuples and arrays
    let tup: (i32, f64, bool) = (5, 3.14, true);
    let arr = [1, 2, 3, 4, 5];

    // Destructuring lets us easily extract values from a tuple
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    // We can access elements of an array using indexing
    println!("The value of the first element of arr is: {}", arr[0]);

    // Rust has both if expressions and if statements
    let n = 5;
    if n < 10 {
        println!("n is less than 10");
    } else {
        println!("n is greater than or equal to 10");
    }

    // Rust also has a number of looping constructs, including
    // loop, while, and for
    let mut i = 0;
    loop {
        if i > 10 {
            break;
        }
        println!("i is {}", i);
        i += 1;
    }

    let mut j = 0;
    while j < 10 {
        println!("j is {}", j);
        j += 1;
    }

    for k in 0..10 {
        println!("k is {}", k);
    }

    // Rust has a powerful pattern matching system that allows
    // us to easily destructure values and bind variables
    let x = 5;
    match x {
        0 => println!("x is zero"),
        1 | 2 => println!("x is one or two"),
        3..=10 => println!("x is between three and ten"),
        _ => println!("x is something else"),
    }

    // Rust has both functions and closures
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    let sum = add(1, 2);
    println!("The sum is {}", sum);

    let add_closure = |a: i32, b: i32| -> i32 {
        a + b
    };
    let sum_closure = add_closure(1, 2);
    println!("The sum from the closure is {}", sum_closure);
    
    // Rust has a number of built-in types and traits for working
    // with collections, such as Vec<T>, Option<T>, and Iterator
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    
    for i in &v {
        println!("i is {}", i);
    }
    
    let first = v.get(0);
    match first {
        Some(x) => println!("The first element is {}", x),
        None => println!("The vector is empty"),
    }
    
    // Rust has a powerful macro system that allows us to easily
    // define custom control flow constructs
    macro_rules! say_hello {
        () => {
            println!("Hello!");
        };
    }
    say_hello!();
    
    // Rust has a number of powerful features for working with
    // concurrent and parallel code, such as threads, channels,
    // and atomic variables
    let handle = std::thread::spawn(|| {
        println!("Hello from the spawned thread!");
    });
    handle.join().unwrap();
    
    let (tx, rx) = std::sync::mpsc::channel();
    tx.send(5).unwrap();
    let received = rx.recv().unwrap();
    println!("Received {} from the channel", received);
    
    let atomic_x = std::sync::atomic::AtomicUsize::new(5);
    atomic_x.store(10, std::sync::atomic::Ordering::SeqCst);
    let x = atomic_x.load(std::sync::atomic::Ordering::SeqCst);
    println!("The value of atomic_x is {}", x);
    
}

// Reverses the lines
use std::io;

fn main() {
    // Read lines from standard input
    let mut lines = Vec::new();
    loop {
        let mut line = String::new();
        let result = io::stdin().read_line(&mut line);
        if result.is_err() || line.is_empty() {
            break;
        }
        lines.push(line);
    }

    // Reverse the lines
    lines.reverse();

    // Print the reversed lines to standard output
    for line in lines {
        println!("{}", line);
    }
}