

macro_rules! whisper {

    ($msg:expr)=>{
        println!("🔥 {} 🔥", $msg)
    }
    
}

fn todo(it: &str) {
    println!("{} ",it)
}

fn main() {
    println!("Hello, world!");
    whisper!(10+20-90);
    todo("this is harsh")
}
