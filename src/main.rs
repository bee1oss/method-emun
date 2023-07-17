use std::io;

/*enum Person {
    Adult,
    Underage,
}
    let person = Person::Underage;

    match person {
        Person::Adult => println!("Zahodi!!"),
        Person::Underage => println!("Tebe Nelzya!!")
    }*/

enum MathOperations {
    Add(f64, f64),
    Sub(f64, f64),
    Mul(f64, f64),
    Div(f64, f64),
}

impl MathOperations {
    fn math_operations(&self) -> f64 {
        match self {
            &MathOperations::Add(a, b) => a + b,
            &MathOperations::Sub(a, b) => a - b,
            &MathOperations::Mul(a, b) => a * b,
            &MathOperations::Div(a, b) => a / b
        }
    }
}

fn main() {
    let mo = MathOperations::Add(18.0, 9.0);
    let result: f64 = mo.math_operations();

    println!("Result is: {result}")
}



/*fn my_trainee(){
    let mut person;
    let mut st_age: String = String::new();
    println!("Enter the age:");
    match io::stdin().read_line(&mut st_age) {
        Ok(_) => {}
        Err(err) => println!("Not occurred {}", err)
    }
    let mut age: i8 = st_age.trim().parse().unwrap();
    if age >= 18 {
        person = Person::Adult
    } else {
        person = Person::Underage
    }
    match person {
        Person::Adult => println!("Zahodi!!"),
        Person::Underage => println!("Tebe Nelzya!!")
    }
}*/