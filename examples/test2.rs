//#![no_std]
extern crate heapless;
//use std::fmt::Write;
use heapless::{String, Vec};

fn takes_str(_s: &str) {}

fn main() {
    // test capacity
    let s: String<[u8; 4]> = String::new();
    assert!(s.capacity() == 4);

    // test from
    let s: String<[u8; 4]> = String::from("abc");
    assert!(s.len() == 3);
    println!("s = {:?}", s);

    let s: String<[_; 4]> = String::from("abcde");
    assert!(s.len() == 4);
    println!("s = {:?}", s);

    // test from_utf8
    let mut v: Vec<u8, [u8; 8]> = Vec::new();
    v.push('a' as u8).unwrap();
    v.push('b' as u8).unwrap();

    let s = String::from_utf8(v).unwrap();
    // v.push('c' as u8); // v has been moved to s (no copy)

    assert!(s.len() == 2);
    takes_str(&s);

    println!("s = {:?}", s);

    // test from_utf8
    let mut v: Vec<u8, [u8; 8]> = Vec::new();
    v.push(240).unwrap();
    v.push(159).unwrap();
    v.push(146).unwrap();
    v.push(150).unwrap();

    let s = String::from_utf8(v);
    println!("s =  {:?}", s);

    // test illegal from_utf8
    let mut v: Vec<u8, [u8; 8]> = Vec::new();
    v.push(0).unwrap();
    v.push(159).unwrap();
    v.push(146).unwrap();
    v.push(150).unwrap();

    let s = String::from_utf8(v);
    println!("s =  {:?}", s);
    assert!(s.is_err());

    // test from_utf8_unchecked
    let mut v: Vec<u8, [u8; 8]> = Vec::new();
    v.push(240).unwrap();
    v.push(159).unwrap();
    v.push(146).unwrap();
    v.push(150).unwrap();

    let s = unsafe { String::from_utf8_unchecked(v) };
    println!("s =  {:?}", s);
    assert!(s.len() == 4);

    // test into_bytes
    let s: String<[_; 4]> = String::from("ab");
    let b = s.into_bytes();
    assert!(b.len() == 2);

    assert_eq!(&['a' as u8, 'b' as u8], &b[..]);

    // test as_str
    let s: String<[_; 4]> = String::from("ab");
    assert!(s.as_str() == "ab");
    let s1 = s.as_str();
    // s.push('c'); // <- cannot borrow `s` as mutable because it is also borrowed as immutable

    // test as_mut_str
    let mut s: String<[_; 4]> = String::from("ab");
    let s = s.as_mut_str();
    s.make_ascii_uppercase();
    println!("s = {:?}", s);

    // test push_str
    let mut s: String<[u8; 8]> = String::new();
    s.push_str("ab");
    s.push_str("cd");
    println!("s = {:?}", s);

    // test push
    let mut s: String<[u8; 8]> = String::from("abc");

    s.push('1').unwrap();
    s.push('2').unwrap();
    s.push('3').unwrap();

    assert!("abc123" == s.as_str());
    println!("s = {:?}", s);
    // assert_eq!("abc123", s); // todo

    //
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
