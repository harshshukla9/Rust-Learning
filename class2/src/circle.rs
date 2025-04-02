 


 pub struct Circle {

    pub radius:f32
 }

impl Circle {

    pub fn area(&self) ->f32 {
        return 3.14 * (self.radius * self.radius);
    }

    pub fn print_circle() {
        println!("I am a cicle");
    }
}