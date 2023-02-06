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
    New,
    Used,
}

fn car_quality(miles: u32) -> (Age, u32) {
    let quality = (Age::New, miles);
    quality
}

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    }
}

fn main() {
    let colors = ["Red", "Green", "Blue", "Yellow"];
    let mut car: Car;
    let mut engine = Transmission::Manual;

    car = car_factory(String::from(colors[0]), engine, true, 10);
    println!(
        "This {:?} car has {} color, {:?} transmission, roof: {}, {} miles",
        car.age.0, car.color, car.motor, car.roof, car.age.1
    );

    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[1]), engine, false, 100);
    println!(
        "This {:?} car has {} color, {:?} transmission, roof: {}, {} miles",
        car.age.0, car.color, car.motor, car.roof, car.age.1
    );

    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[2]), engine, false, 200);
    println!(
        "This {:?} car has {} color, {:?} transmission, roof: {}, {} miles",
        car.age.0, car.color, car.motor, car.roof, car.age.1
    );
}
