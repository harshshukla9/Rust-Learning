

pub struct Rect {
    
   pub  height:f32,
   pub width:f32
}

//way to implement member 
impl Rect {

    pub fn area(&self) -> f32 {

        return self.width * self.height;

    }
    pub fn perimeter(&self) -> f32 {
        return 2.0 * (self.width + self.height);
    }

    //static function
    pub fn print_someting(a:u32) {
        println!("Static fuinction");
        println!("{}",a);
    }
}