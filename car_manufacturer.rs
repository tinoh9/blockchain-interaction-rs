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

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    if car_quality(miles).0 == Age::Used {
        if roof {
            println!(
                "Prepare a used car: {:?}, {}, Hard top, {} miles",
                motor, color, miles
            )
        } else {
            println!(
                "Prepare a used car: {:?}, {}, Convertible top, {} miles",
                motor, color, miles
            )
        }
    } else {
        if roof {
            println!(
                "Prepare a new car: {:?}, {}, Hard top, {} miles",
                motor, color, miles
            )
        } else {
            println!(
                "Prepare a new car: {:?}, {}, Convertible top, {} miles",
                motor, color, miles
            )
        }
    }

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

    car = car_factory(String::from(colors[0]), engine, true, 0);
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
