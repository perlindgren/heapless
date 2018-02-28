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
    let _s1 = s.as_str();
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

    // test as_bytes
    let s: String<[u8; 8]> = String::from("hello");
    assert_eq!(&[104, 101, 108, 108, 111], s.as_bytes());

    // test as_truncate
    let mut s: String<[u8; 8]> = String::from("hello");
    s.truncate(2);
    assert_eq!("he", s);
    assert_eq!(s, "he");

    // test pop
    let mut s: String<[u8; 8]> = String::from("foo");

    assert_eq!(s.pop(), Some('o'));
    assert_eq!(s.pop(), Some('o'));
    assert_eq!(s.pop(), Some('f'));
    assert_eq!(s.pop(), None);

    // test is_empty
    let mut v: String<[u8; 8]> = String::new();
    assert!(v.is_empty());
    let _ = v.push('a');
    assert!(!v.is_empty());

    // test clear
    let mut s: String<[u8; 8]> = String::from("foo");
    s.clear();
    assert!(s.is_empty());
    assert_eq!(0, s.len());
    assert_eq!(8, s.capacity());
}
