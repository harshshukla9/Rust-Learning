use rect::Rect;
use circle::Circle;
use std::fs;
use std::time::Instant;

//importing
pub mod rect;

//importing
pub mod circle;

enum Direction {
    North,
    South,
    East,
    West,
}
//enum with value
enum Shape {
    Square(f32),
    Circle(f32),
    Rectangle(f32,f32)
}

//enum with option /genraric


enum Option1 {
    Some(u32),
    None
}





fn main() {
    println!("Hello, world!");
    // let mut  str:String = String::from("Harsh");

   //imutable refernce

//    let s2:&String= &str;
//    let s3:&String= &str;
//    let s4:&String= &str;
//   println!("{} {} {} {}",str,s2,s3,s4);
//     we can only have single mitable refernce warna niche error aayegi 
    
   
    // str.push_str("shukla");
    // let (len,str)= get_length(str);
    // println!("{}",len);

    //struct


    let r = Rect {
        width:10.0,
        height:10.0
    };
    println!("{} {}",r.width,r.height);
    println!("{}",r.area());
    Rect::print_someting(199990);
    println!("{}",r.perimeter());

    let c = Circle {
        radius:2.0,
    };

    println!("{}",c.area());
    Circle::print_circle();

    let direction = Direction::West;

    steer(direction);

    let shape_square:Shape = Shape::Square(10.0);
    let shape_circle:Shape = Shape::Circle(19.0);
    let shape_rectangle:Shape = Shape::Rectangle(12.4,12.9);

    identify_shape(shape_square);
    identify_shape(shape_circle);
    identify_shape(shape_rectangle);
    let start = Instant::now(); // Start timing

   let contents = fs::read_to_string("bigfile.txt");

 

    match contents {
        Ok(contents) => println!("File read successfully {} ",contents),
        Err(_) => println!("Error while reading file"),
    }
    let duration = start.elapsed(); // Stop timing
    println!("{:?}",duration);
    let ans= find_first_a(String::from("Hrsh"));

    match ans {
        None => println!("Value not found"),
        Some(val)=>println!("a found at index {}",val),
    }

    
}

//write a function take shape as an input and print it area;

fn identify_shape(shape:Shape) {
    
    
        match shape {
            Shape::Square(side) => println!("Shape is a square with side: {} area is {}", side,side*side),
            Shape::Circle(radius) => println!("Shape is a circle with radius: {} area is {}", radius,3.14 * (radius *radius)),
            Shape::Rectangle(width, height) => println!("Shape is a rectangle with width: {} and height: {} area is {}", width, height , width* height),
        }
    

}


fn steer(dir:Direction) {

    match dir {
        Direction::North => println!("North"),
        Direction::East => println!("East"),
        _ => println!("Fuck off other direction")
    }
}



fn get_length(str:String) -> (usize,String) {
    return (str.len(),str);

}

fn find_first_a(str:String) -> Option<u32> {
    //number ,null Option::None,Option::Some

    let mut index =0;
    for c in str.chars() {
        index = index+1;
        if c == 'a' {
            return Some(index);
        }

    }
    None
}