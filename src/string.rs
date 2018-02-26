use core::marker::Unsize;
use core::{fmt, ops, str};
use core::str::Utf8Error;

use {BufferFullError, Vec};

/// A String, backed by a fixed size array
///
/// String: https://doc.rust-lang.org/std/string/struct.String.html

pub struct String<A>
where
    // FIXME(rust-lang/rust#44580) use "const generics" instead of `Unsize`
    A: Unsize<[u8]>,
{
    vec: Vec<u8, A>,
}

impl<A> String<A>
where
    A: Unsize<[u8]>,
{
    /// Constructs a new, empty `String` backed by a Vec<u8,[u8;N]>
    ///
    /// Notice, in comparison to std:String, heapless strings are
    /// limited to ASCII/utf8 encoding.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// let mut s: String<[u8; 4]> = String::new();
    ///
    pub const fn new() -> Self {
        String { vec: Vec::new() }
    }

    /// Returns the maximum number of elements the String can hold
    ///
    /// # Examples
    ///
    /// let mut s: String<[u8; 4]> = String::new();
    /// assert!(s.capacity() == 4);
    ///
    pub fn capacity(&self) -> usize {
        let buffer: &[u8] = unsafe { self.vec.buffer.as_ref() };
        buffer.len()
    }

    /// Converts a vector of bytes to a `String`.
    ///
    /// A string slice ([`&str`]) is made of bytes ([`u8`]), and a vector of bytes
    /// ([`Vec<u8>`]) is made of bytes, so this function converts between the
    /// two. Not all byte slices are valid `String`s, however: `String`
    /// requires that it is valid UTF-8. `from_utf8()` checks to ensure that
    /// the bytes are valid UTF-8, and then does the conversion.
    ///
    /// See std::String for further information.
    ///
    /// # Examples
    ///
    ///
    pub fn from_utf8(vec: Vec<u8, A>) -> Result<String<A>, Utf8Error> {
        {
            let buffer: &[u8] = unsafe { vec.buffer.as_ref() };
            str::from_utf8(&buffer[0..vec.len])?;
        }
        Ok(String { vec: vec })
    }

    /// Exa
    pub fn into_bytes(self) -> Vec<u8, A> {
        self.vec
    }

    ///
    pub fn as_str(&self) -> &str {
        let buffer: &[u8] = unsafe { self.vec.buffer.as_ref() };
        unsafe { str::from_utf8_unchecked(&buffer[..self.vec.len]) }
    }

    ///
    pub fn as_mut_str(&mut self) -> &mut str {
        let buffer: &mut [u8] = unsafe { self.vec.buffer.as_mut() };
        unsafe { str::from_utf8_unchecked_mut(&mut buffer[..self.vec.len]) }
    }

    ///
    pub fn push_str(&mut self, s: &str) {
        let buffer: &mut [u8] = unsafe { self.vec.buffer.as_mut() };
        let start = self.vec.len;
        let end = start.saturating_add(s.len());
        let new_len = end.min(buffer.len());
        self.vec.len = new_len;
        buffer[start..self.vec.len].copy_from_slice(
            &s.as_bytes()[0..self.vec.len.saturating_sub(start)],
        );
    }

    ///
    pub fn push(&mut self, c: char) -> Result<(), BufferFullError> {
        self.vec.push(c as u8)
    }

    ///
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { self.vec.buffer.as_ref() }
    }

    ///
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe { self.vec.buffer.as_mut() }
    }

    ///
    pub fn truncate(&mut self, new_len: usize) {
        self.vec.truncate(new_len)
    }

    ///
    pub fn pop(&mut self) -> Option<char> {
        match self.vec.pop() {
            Some(c) => Some(c as char),
            None => None,
        }
    }

    ///
    pub fn remove(&mut self, idx: usize) -> char {
        unimplemented!();
    }

    ///
    pub fn insert(&mut self, idx: usize, ch: char) {
        unimplemented!();
    }

    ///
    pub fn insert_str(&mut self, idx: usize, string: &str) {
        unimplemented!();
    }

    ///
    pub fn as_mut_vec(&mut self) -> &mut Vec<u8, A> {
        &mut self.vec
    }

    ///
    pub fn is_empty(&self) -> bool {
        self.vec.len == 0
    }

    ///
    // pub fn split_off<B>(&mut self, at: usize) -> String<B>
    // where
    //     B: Unsize<[u8]>,
    // {
    //     let buffer: &mut [u8] = unsafe { self.vec.buffer.as_mut() };
    //     if self.vec.len <= at {
    //         let (mut b1, mut b2) = buffer.split_at_mut(at);
    //         //self.vec.buffer = UntaggedOption::some(A);

    //         String {
    //             vec: Vec {
    //                 _marker: PhantomData,
    //                 buffer: UntaggedOption::none(),
    //                 len: b2.len(),
    //             },
    //         }
    //     } else {
    //         String {
    //             vec: Vec {
    //                 _marker: PhantomData,
    //                 buffer: UntaggedOption::none(),
    //                 len: 0,
    //             },
    //         }
    //     }
    // }

    ///
    pub fn from<'a>(&mut self, s: &'a str) -> Result<(), BufferFullError> {
        match self.vec.len <= s.len() {
            true => {
                let buffer: &mut [u8] = unsafe { self.vec.buffer.as_mut() };
                let len = s.len().min(buffer.len());
                self.vec.len = len;
                buffer[0..len].copy_from_slice(&s.as_bytes()[0..len]);
                Ok(())
            }
            _ => Err(BufferFullError),
        }
    }

    ///
    pub fn len(&self) -> usize {
        self.vec.len
    }
}

impl<A> fmt::Debug for String<A>
where
    A: Unsize<[u8]>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let slice: &str = match str::from_utf8(&*self.vec) {
            Ok(s) => s,
            Err(_) => "could not convert to String",
        };
        slice.fmt(f)
    }
}

impl<A> fmt::Write for String<A>
where
    A: Unsize<[u8]>,
{
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.push_str(s);
        Ok(())
    }

    // fn write_char(&mut self, c: char) -> Result<(), fmt::Error> {
    //     self.push(c);
    //     Ok(())
    // }
}

impl<A> ops::Deref for String<A>
where
    A: Unsize<[u8]>,
{
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl<A> ops::DerefMut for String<A>
where
    A: Unsize<[u8]>,
{
    fn deref_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}

impl<A, B> PartialEq<String<B>> for String<A>
where
    A: Unsize<[u8]>,
    B: Unsize<[u8]>,
{
    fn eq(&self, rhs: &String<B>) -> bool {
        PartialEq::eq(&**self, &**rhs)
    }
}

impl<A> Eq for String<A>
where
    A: Unsize<[u8]>,
{
}
