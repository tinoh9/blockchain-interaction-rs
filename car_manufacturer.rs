#[derive(Debug, PartialEq)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(Debug, PartialEq)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(Debug, PartialEq)]
enum Age {
    Used,
    New,
}

fn car_quality(miles: u32) -> (Age, u32) {
    if miles > 0 {
        return (Age::Used, miles);
    } else {
        return (Age::New, miles);
    }
}

fn car_factory(order: u32, miles: u32) -> Car {
    let colors = ["Red", "Green", "Blue", "Yellow"];
    let mut color = order as usize;
    if color > 4 {
        color = color - 4;
    }

    let mut motor = Transmission::Manual;
    let mut roof = true;
    if order % 3 == 0 {
        motor = Transmission::SemiAuto;
    } else if order % 2 == 0 {
        motor = Transmission::Automatic;
        roof = false;
    }

    Car {
        color: String::from(colors[(color - 1) as usize]),
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    }
}

fn main() {
    use std::collections::HashMap;

    let mut orders: HashMap<u32, Car> = HashMap::new();
    let mut order = 1;
    let mut car: Car;

    car = car_factory(order, 0);
    orders.insert(order, car);
    println!("Order {}: {:?}", order, orders.get(&order));

    order = order + 1;
    car = car_factory(order, 100);
    orders.insert(order, car);
    println!("Order {}: {:?}", order, orders.get(&order));

    order = order + 1;
    car = car_factory(order, 0);
    orders.insert(order, car);
    println!("Order {}: {:?}", order, orders.get(&order));

    order = order + 1;
    car = car_factory(order, 1000);
    orders.insert(order, car);
    println!("Order {}: {:?}", order, orders.get(&order));
}
