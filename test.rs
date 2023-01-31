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

//// Calculate an area of a triangle
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

//// To-do list management
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

//// Read and count the total number of characters in a file (error handling, read filesystem)
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

//// SOME OF FEATURES IN RUST
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

//// Reverses the lines
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

//// Key-value store
use std::collections::HashMap;

fn main() {
    // A simple key-value store implemented using a hash map
    let mut store = HashMap::new();

    // Insert some key-value pairs into the store
    store.insert("apple", 0.99);
    store.insert("banana", 0.59);
    store.insert("orange", 0.79);

    // Look up the value of a key in the store
    let price = store.get("banana");
    println!("The price of a banana is {:?}", price);
}

//// Borrow checker
fn main() {
    let mut a = 10;
    let b = &mut a; // borrow mutable "a" as "b"
    *b = *b + 5; // update "a" reference to new value
    println!("a = {}", a);

    let c = &a; // borrow immutable "a" (cannot borrow as mutable "a" because allow only 1 mutable variable borrowed at a time)
    println!("c = {}", c);

    drop(b); // "b" goes out of scope, return mutable "a"

    let d = &mut a; // borrow mutable "a" as "d" again
    *d += 5; // update "a" reference to new value
    println!("a = {}", a);
}

//// Channel & Concurrency
use std::thread;
use std::sync::mpsc;

fn main() {
    // Create a channel to communicate between threads
    let (tx, rx) = mpsc::channel();

    // Create a new thread and move a closure into it
    let handle = thread::spawn(move || {
        // Send a message through the channel
        tx.send(5).unwrap();
    });

    // Wait for the spawned thread to finish
    handle.join().unwrap();

    // Receive the message from the channel
    let received = rx.recv().unwrap();
    println!("Received {} from the channel", received);
}

//// Parallel programming
use rayon::prelude::*;

fn main() {
    let a = vec![1, 2, 3, 4, 5];
    let b = a.par_iter().sum();
    println!("{}", b);
}

//// Functional programming
let vec_1 = vec![1, 2, 3, 4, 5];
let result_1: Vec<i32> = vec_1.iter()
    .map(|x| x * 2)
    .filter(|x| x > &3)
    .collect();

println!("{:?}", result_1);

let vec_2 = vec![1, 2, 3, 4, 5];
let result_2 = vec_2.iter().fold(0, |acc, &x| acc + x);

println!("{}", result_2);

//// Smart pointers & Ownership
use std::rc::Rc;

struct List {
    value: i32,
    next: Option<Rc<List>>,
}

fn main() {
    let a = Rc::new(List { value: 1, next: None });
    let b = Rc::new(List { value: 2, next: Some(Rc::clone(&a)) });
    let c = Rc::new(List { value: 3, next: Some(Rc::clone(&b)) });

    println!("a.value = {}", a.value);
    println!("b.value = {}", b.value);
    println!("c.value = {}", c.value);
}

//// Declarative macros
macro_rules! debug_println {
    ($($arg:tt)*) => {
        {
            if std::env::var("DEBUG").is_ok() {
                println!($($arg)*);
            }
        }
    };
}

fn main() {
    debug_println!("debug message");
}
    
//// Procedural macros
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, DataStruct, Field};

#[proc_macro_derive(Serialize)] // Custom attribute
pub fn serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match &input.data {
        Data::Struct(DataStruct { fields, .. }) => fields,
        _ => panic!("#[derive(Serialize)] can only be used with structs"),
    };
    let field_names = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap())
        .map(|i| i.to_string())
        .collect::<Vec<_>>();

    let expanded = quote! {
        impl Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut state = serializer.serialize_struct(stringify!(#name), #field_names.len())?;
                #(state.serialize_field(#field_names, &self.#field_names)?;)*
                state.end()
            }
        }
    };
    expanded.into()
}

//// Error handling
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::Path;

enum MyError {
    FileError(Error),
    ParseError(std::num::ParseIntError),
    NegativeNumberError,
}

impl From<Error> for MyError {
    fn from(error: Error) -> Self {
        MyError::FileError(error)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(error: std::num::ParseIntError) -> Self {
        MyError::ParseError(error)
    }
}

fn read_file(file_name: &str) -> Result<i32, MyError> {
    let path = Path::new(file_name);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(error) => {
            if error.kind() == ErrorKind::NotFound {
                return Err(MyError::FileError(error));
            } else {
                return Err(error.into());
            }
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let number: i32 = contents.trim().parse()?;
    if number < 0 {
        return Err(MyError::NegativeNumberError);
    } else {
        Ok(number)
    }
}

//// Traits
trait Drawable {
    fn draw(&self);
}

trait Canvas {
    fn set_color(&mut self, color: &str);
    fn draw_shape(&mut self, shape: &dyn Drawable);
}

struct Rectangle {
    width: f64,
    height: f64,
    color: String,
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle with width {} and height {} and color {}",
                 self.width, self.height, self.color);
    }
}

struct Circle {
    radius: f64,
    color: String,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {} and color {}",
                 self.radius, self.color);
    }
}

struct Square {
    side: f64,
    color: String,
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a square with side {} and color {}",
                 self.side, self.color);
    }
}

struct Screen {
    color: String,
}

impl Canvas for Screen {
    fn set_color(&mut self, color: &str) {
        self.color = color.to_string();
    }

    fn draw_shape(&mut self, shape: &dyn Drawable) {
        shape.draw();
    }
}

fn main() {
    let mut screen = Screen { color: "black".to_string() };
    let rect = Rectangle { width: 3.0, height: 4.0, color: "red".to_string() };
    let circle = Circle { radius: 5.0, color: "blue".to_string() };
    let square = Square { side: 2.0, color: "green".to_string() };
    screen.set_color("white");
    screen.draw_shape(&rect);
    screen.draw_shape(&circle);
    screen.draw_shape(&square);
}

//// Unsafe Rust (not recommend to use this feature regularly, use safety alternatives instead)
struct MyStruct {
    data: [i32; 10],
}

fn main() {
    let my_struct = MyStruct { data: [0; 10] };
    let my_struct_ptr: *const MyStruct = &my_struct;
    let my_data_ptr: *const i32 = my_struct_ptr as *const i32;
    let my_data_slice: &[i32] = unsafe {
        std::slice::from_raw_parts(my_data_ptr, 10)
    };
    println!("My data slice: {:?}", my_data_slice);
    unsafe {
        *my_data_ptr = 1;
        println!("My data slice: {:?}", my_data_slice);
    }
}

//// Generics
struct MyStruct<T> {
    data: [T; 10],
}

impl<T> MyStruct<T> {
    fn new(item: T) -> MyStruct<T> {
        MyStruct { data: [item; 10] }
    }

    fn update(&mut self, index: usize, item: T) {
        self.data[index] = item;
    }

    fn get(&self, index: usize) -> &T {
        &self.data[index]
    }
}

fn main() {
    let mut my_struct = MyStruct::new(0);
    my_struct.update(5, 10);
    println!("Data at index 5: {:?}", my_struct.get(5));

    let mut my_struct_2 = MyStruct::new(String::from("Hello"));
    my_struct_2.update(5, String::from("World"));
    println!("Data at index 5: {:?}", my_struct_2.get(5));
}

//// Lifetimes
struct MyStruct<'a, T: 'a> {
    data: &'a T,
    data_vec: &'a Vec<T>,
}

fn my_function<'a, T>(my_struct: &'a MyStruct<'a, T>) -> &'a T
where T: 'a
{
    my_struct.data
}

fn main() {
    let my_vec = vec![1, 2, 3];
    let my_struct = MyStruct {
        data: &my_vec[0],
        data_vec: &my_vec,
    };

    let data = my_function(&my_struct);
    println!("Data: {:?}", data);
}

//// Multi-threaded TCP server
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn handle_client(stream: TcpStream) {
    let mut buffer = [0; 512];

    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(bytes) => bytes,
            Err(_) => return,
        };

        if bytes_read == 0 {
            return;
        }

        match stream.write(&buffer[..bytes_read]) {
            Ok(_) => (),
            Err(_) => return,
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream)
                });
            }
            Err(_) => { /* connection failed */ }
        }
    }
}

//// Bubble sort algorithm for ascending order of integers in a vector
fn main(mut v: Vec<i32>) -> Vec<i32> {
    let mut n = v.len();
    loop {
        let mut new_n = 0;
        for i in 1..n {
            if v[i - 1] > v[i] {
                v.swap(i - 1, i);
                new_n = i;
            }
        }
        n = new_n;
        if n == 0 {
            break;
        }
    }
    v
}

//// Non-ASCII alphabetic characters filter (using iterator)
struct AsciiAlphabetic<I> {
    iter: I,
}

impl<I> Iterator for AsciiAlphabetic<I>
where
    I: Iterator<Item = u8>,
{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(b) = self.iter.next() {
            if (b >= b'a' && b <= b'z') || (b >= b'A' && b <= b'Z') {
                return Some(b);
            }
        }
        None
    }
}

fn ascii_alphabetic<I>(iter: I) -> AsciiAlphabetic<I>
where
    I: Iterator<Item = u8>,
{
    AsciiAlphabetic { iter }
}

//// Problem: Write a function that reverses the input string and returns the reversed string
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("world"), "dlrow");
    }
}

//// Problem: Write a function that takes a vector of integers as input and returns a new vector
//// containing only the unique elements in the input vector, preserving the original order.

// use std::collections::HashSet;

fn unique_elements(v: Vec<i32>) -> Vec<i32> {
    let mut unique = Vec::new();
//  let mut unique = HashSet::new();
    for i in v {
        if !unique.contains(&i) {
            unique.push(i);
//          unique.insert(i); 
        }
    }
    unique
//  unique.into_iter().collect()  
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_elements() {
        assert_eq!(unique_elements(vec![1, 2, 3, 2, 1]), vec![1, 2, 3]);
        assert_eq!(unique_elements(vec![1, 1, 1, 1, 1]), vec![1]);
    }
}

//// Least Recently Used (LRU) cache
use std::collections::HashMap;

struct LRUCache {
    capacity: usize,
    cache: HashMap<i32, i32>,
    keys: Vec<i32>,
}

impl LRUCache {
    fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            cache: HashMap::new(),
            keys: Vec::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(value) = self.cache.get(&key) {
            let index = self.keys.iter().position(|&k| k == key).unwrap();
            self.keys.remove(index);
            self.keys.push(key);
            return *value;
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.cache.contains_key(&key) {
            let index = self.keys.iter().position(|&k| k == key).unwrap();
            self.keys.remove(index);
        } else if self.keys.len() == self.capacity {
            let oldest_key = self.keys.remove(0);
            self.cache.remove(&oldest_key);
        }
        self.keys.push(key);
        self.cache.insert(key, value);
    }
}

//// Return the number of set bits (bits with value 1)
fn count_bits(n: u32) -> u32 {
    let mut count = 0;
    let mut x = n;
    while x > 0 {
        count += x & 1;
        x >>= 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bits() {
        assert_eq!(count_bits(0), 0);
        assert_eq!(count_bits(1), 1);
        assert_eq!(count_bits(2), 1);
        assert_eq!(count_bits(3), 2);
        assert_eq!(count_bits(4), 1);
        assert_eq!(count_bits(5), 2);
    }
}

//// Check if the string is palindrome or not
use std::collections::HashMap;

fn is_palindrome(s: &str) -> bool {
    let mut char_count = HashMap::new();
    for c in s.chars() {
        *char_count.entry(c).or_default() += 1;
    }

    let mut num_odd = 0;
    for (_, count) in char_count.iter() {
        if count % 2 != 0 {
            num_odd += 1;
        }
        if num_odd > 1 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("level"));
        assert!(!is_palindrome("hello"));
    }
}

//// Return the largest continuous sum of subarrays from a list of integers
fn largest_continuous_sum(numbers: Vec<i32>) -> i32 {
    let mut max_sum = numbers[0];
    let mut current_sum = numbers[0];

    for i in 1..numbers.len() {
        current_sum = std::cmp::max(numbers[i], current_sum + numbers[i]);
        max_sum = std::cmp::max(max_sum, current_sum);
    }

    max_sum
}
