let x: u64 = 100;

let y = 50;
let y = y + 10;

let pair = ('abc', 99);
pair.0;
pair.1;

let couple: (u64, char) = (90, 'a');

let smallest = std::cmp::min(2,9);

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