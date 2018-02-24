//#![no_std]
extern crate heapless;
//use heapless;
//use std::fmt;
use std::fmt::Write;
// use std::io::Write;
//use heapless::String;

fn main() {
    // let mut s: heapless::String<[u8; 4]> = heapless::String::new();
    //let s: heapless::String<[u8; 4]> = format!("1 {}", "23");
    // let s = format!("1 {}", "23");

    // let mut ss = String::from("new(");
    // let ss = format!("1 {}", "23");

    // let mut w: Vec<u8> = Vec::new();
    // write!(&mut w, "{}", 1).unwrap();

    let mut s: heapless::String<[u8; 4]> = heapless::String::new();
    write!(&mut s, "t {}", 1).unwrap();
    // s.from("t");
    // let _ = s.write_char('e');
    // let _ = s.write_str("sting"); // last chars lost

    // println!("w {:?}", w);
    println!("s {:?}", s);
    println!("s {}", s);

    //    let f = format_args!("uotha {}", 1);
    // //    write!(fmt::format, "etuho");
    // //s = format!("nth");

    // s.from("Test");
    // println!("String {:?}, len {}", s, s.len());

    // s.from("12345");
    // println!("String {:?}, len {}", s, s.len());

    // println!("ss {}", ss);
}
