fn main() {
    let colors = ["Blue", "Green", "Red", "Silver"];
    
    car_factory(String::from(colors[0]), Transmission::Manual, false, 50);    

    car_factory(String::from(colors[1]), Transmission::Automatic, true, 0);

    car_factory(String::from(colors[2]), Transmission::SemiAuto, false, 5);
}

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

fn car_factory(color: String, motor: Transmission, convertible: bool, miles: u32) -> Car {
    let car = Car {
        color,
        motor,
        convertible,
        age: car_quality(miles)
    };

    if car.age.0 == Age::Used {
        println!("Preparing a used car: {}, {:?} motor, convertible: {} with mileage: {}", car.color, car.motor, car.convertible, car.age.1);
    } else {
        println!("Preparing a new car: {}, {:?} motor, convertible: {} with mileage: {}", car.color, car.motor, car.convertible, car.age.1);
    }    

    return car;
}

fn car_quality(miles: u32) -> (Age, u32) {

    if miles == 0 {
        return (Age::New, miles);
    }

    return (Age::Used, miles);
}