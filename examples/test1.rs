extern crate heapless;
//use heapless;

fn main() {
    let mut s: heapless::String<[u8; 4]> = heapless::String::new();

    let mut ss = String::from("new(");
    let ss = format!("uhtaou");
    //s = format!("nth");

    s.from("Test");
    println!("String {:?}, len {}", s, s.len());

    s.from("12345");
    println!("String {:?}, len {}", s, s.len());

    println!("ss {}", ss);
}
