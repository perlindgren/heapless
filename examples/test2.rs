//#![no_std]
extern crate heapless;
use std::fmt::Write;
use heapless::{String, Vec};

fn takes_str(s: &str) {}
///
/// let s = String::from("Hello");
///
/// takes_str(&s);
///
fn main() {
    let mut s: String<[u8; 4]> = String::new();
    assert!(s.capacity() == 4);

    let mut v: Vec<u8, [u8; 8]> = Vec::new();
    v.push('a' as u8);
    v.push('b' as u8);

    let s = String::from_utf8(v).unwrap();
    // v.push('c' as u8); // v has been moved to s (no copy)

    assert!(s.len() == 2);
    takes_str(&s);

    println!("{:?}", s);

    let mut v: Vec<u8, [u8; 8]> = Vec::new();
    v.push(240);
    v.push(159);
    v.push(146);
    v.push(150);

    let s = String::from_utf8(v);
    println!("s =  {:?}", s);

    let mut v: Vec<u8, [u8; 8]> = Vec::new();
    v.push(0);
    v.push(159);
    v.push(146);
    v.push(150);

    let s = String::from_utf8(v);
    println!("s =  {:?}", s);
    assert!(s.is_err());

    let mut v: Vec<u8, [u8; 8]> = Vec::new();
    v.push(240);
    v.push(159);
    v.push(146);
    v.push(150);

    let s = unsafe { String::from_utf8_unchecked(v) };
    println!("s =  {:?}", s);
    assert!(s.len() == 4);

    // assert!(s.len() == 3);
    // assert!(s.as_str() == "t 1");
    // write!(&mut s, "t {}", 1).unwrap();

    // println!("{:?} capacity {}, length {}", s, s.capacity(), s.len());

    // write!(&mut s, "2").unwrap();
    // println!("{:?} capacity {}, length {}", s, s.capacity(), s.len());

    // s.from_utf8(&v);
    // println!("from_utf8 s {:?}", s);

    // s.clear();
    // write!(&mut s, "t {}", 1).unwrap();
    // println!("written s {:?}", s);

    // s.clear();
    // println!("cleared s {:?}", s);

    // write!(&mut s, "1").unwrap();
    // write!(&mut s, "2").unwrap();
    // write!(&mut s, "3").unwrap();

    // println!("written 3 times s {:?}", s);

    // {
    //     let ss = s.as_str();
    //     println!("ss {:?}", ss);
    // }

    // {
    //     let ss = unsafe { s.as_mut_str().as_bytes_mut() };
    //     ss[0 as usize] = 'a' as u8;
    //     println!("in the inner ss {:?}", ss);
    //     let sss = &mut ss[0..1];
    //     sss[0] = 'b' as u8;
    //     println!("sss {:?}", str::from_utf8(&sss).unwrap());
    // }

    // println!("has now changed in outer s {:?}", s);

    // s.clear();

    // // s.from("t");
    // // let _ = s.write_char('e');
    // // let _ = s.write_str("sting"); // last chars lost

    // // println!("w {:?}", w);
    // // println!("s {:?}", s);
    // // println!("s {}", s);

    // // //    let f = format_args!("uotha {}", 1);
    // // //    write!(fmt::format, "etuho");
    // // //s = format!("nth");

    // // s.from("Test");
    // // println!("String {:?}, len {}", s, s.len());

    // // s.from("12345");
    // // println!("String {:?}, len {}", s, s.len());

    // // println!("ss {}", ss);
}
