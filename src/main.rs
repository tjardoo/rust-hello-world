use std::collections::HashMap;

fn main() {
    let colors = ["Blue", "Green", "Red", "Silver"];

    let mut orders: HashMap<u32, Car> = HashMap::new();

    let mut car: Car;
    let mut miles = 0;

    for order in 1..6 {
        car = car_factory(order, String::from(colors[0]), Transmission::Manual, false, miles);
        orders.insert(order, car);

        if miles > 20 {
            miles = 0;
        } else {
            miles = miles + 5;
        }

    }

    let order_id: u32 = 2;
    println!("\nCar for order #{}: {:?}", order_id, orders.get(&order_id));

    // let big_birds = ["ostrich", "peacock", "stork"];
    // for bird in big_birds.iter() {
    //     println!("The {} is a big bird.", bird);
    // }
}

#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    convertible: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

fn car_factory(order: u32, color: String, motor: Transmission, convertible: bool, miles: u32) -> Car {
    let car = Car {
        color,
        motor,
        convertible,
        age: car_quality(miles)
    };

    if car.age.0 == Age::Used {
        println!("Order #{}: preparing a used car: {}, {:?} motor, convertible: {} with mileage: {}", order, car.color, car.motor, car.convertible, car.age.1);
    } else {
        println!("Order #{}: preparing a new car: {}, {:?} motor, convertible: {} with mileage: {}", order, car.color, car.motor, car.convertible, car.age.1);
    }    

    return car;
}

fn car_quality(miles: u32) -> (Age, u32) {
    if miles == 0 {
        return (Age::New, miles);
    }

    return (Age::Used, miles);
}