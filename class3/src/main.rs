use chrono::{Utc, Local};
use dotenv::dotenv;
use std::env;
use std::ops::Add;



trait Baddie { //there are the just sama as implemtatin in java onlu we have to give signature
    
    fn baddie(&self) ->u32;
}

struct RedheadBaddie{

    thigh:u32
}

struct BlueheadBaddie {

    thigh:u32
}

impl Baddie for RedheadBaddie{
    fn baddie(&self) ->u32 {
        return 10;
    }
}
struct Rect<T> {
    width: T,
    height: T,
}

impl<T: std::ops::Mul<Output = T> + Copy> Rect<T> {
    fn area(&self) -> T {
        self.width * self.height
    }
}

fn main() {
    dotenv().ok(); // Load environment variables

    let utc = Utc::now();
    let local = Local::now();
    println!("{}", utc);
    println!("{}", local);

    let var = env::var("REDIS_ADDRESS");

    match var {
        Ok(value) => println!("{}", value),
        Err(_) => println!("Error while reading REDIS_ADDRESS"),
    }

    print_variable("Harsh");

    let r = Rect {
        width: 10,
        height: 10,
    };
    let r1 = Rect {
        width:10.5,
        height:11.5
    };
    println!("Area of rectangle: {}", r.area());
    println!("Area of rectangle: {}", r1.area());
}

fn sum<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn print_variable<T: std::fmt::Display>(a: T) {
    println!("{}", a);
}
