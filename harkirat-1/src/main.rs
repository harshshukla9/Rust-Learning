
fn main() {
    println!("Hello, world!");
    let ans:u32 = sum( 1,2);
    println!("{}",ans);

    let ans2:u32 = subract(10,4);
    println!("{}",ans2);

    let ans3:u32 = multiply(10,29);
    println!("{}",ans3);

    let ans4:bool = is_even(2);
    println!("{}",ans4);

    let name: String = String::from("Harsh");
    println!("furstname-{}",name);

    let v = vec![1,2,3];
    println!("{:?}",v);
    legacy();


}

// c++ int int64 long long

fn sum(a:u32,b:u32) -> u32 {
    return a+b;
}
fn subract(a:u32,b:u32) -> u32 {
    return a-b;

}
fn multiply(a:u32,b:u32) -> u32 {
    return  a*b;

}

fn is_even(a:u32)-> bool {
    return a%2==0;
}



//rust memeory safe rust is fast and safe something like ownership rules why string recure memeory mangement 

//developer are dump ownership rule will protect legacy of rust
fn legacy() {

    let name = String::from("harsh");
    let name2:String= name;
    println!("{}",name2)
    
}