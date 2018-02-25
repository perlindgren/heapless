use core::marker::{PhantomData, Unsize};
use core::{fmt, ops, ptr, slice, str};
use core::str::Utf8Error;

use {BufferFullError, Vec};

/// A String, backed by a fixed size array
///
/// String: https://doc.rust-lang.org/std/string/struct.String.html

pub struct String2<A>
where
    // FIXME(rust-lang/rust#44580) use "const generics" instead of `Unsize`
    A: Unsize<[u8]>,
{
    vec: Vec<u8, A>,
}

use untagged_option::UntaggedOption;

impl<A> String2<A>
where
    A: Unsize<[u8]>,
{
    /// Constructs a new, empty `String` backed by the array `A`
    pub const fn new() -> Self {
        String2 { vec: Vec::new() }
    }

    /// Returns the maximum number of elements the String can hold
    pub fn capacity(&self) -> usize {
        let buffer: &[u8] = unsafe { self.vec.buffer.as_ref() };
        buffer.len()
    }

    // /// Clears the string by setting lenght to zero
    // pub fn clear(&mut self) {
    //     self.len = 0;
    // }

    ///
    ///
    pub fn from_utf8(vec: Vec<u8, A>) -> Result<String2<A>, Utf8Error> {
        {
            let buffer: &[u8] = unsafe { vec.buffer.as_ref() };
            str::from_utf8(&buffer[0..vec.len])?;
        }
        Ok(String2 { vec: vec })
    }

    ///
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
        buffer[start..self.vec.len]
            .copy_from_slice(&s.as_bytes()[0..self.vec.len.saturating_sub(start)]);
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
    // pub fn split_off<B>(&mut self, at: usize) -> String2<B>
    // where
    //     B: Unsize<[u8]>,
    // {
    //     let buffer: &mut [u8] = unsafe { self.vec.buffer.as_mut() };
    //     if self.vec.len <= at {
    //         let (mut b1, mut b2) = buffer.split_at_mut(at);
    //         //self.vec.buffer = UntaggedOption::some(A);

    //         String2 {
    //             vec: Vec {
    //                 _marker: PhantomData,
    //                 buffer: UntaggedOption::none(),
    //                 len: b2.len(),
    //             },
    //         }
    //     } else {
    //         String2 {
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

impl<A> fmt::Debug for String2<A>
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

impl<A> fmt::Write for String2<A>
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

impl<A> ops::Deref for String2<A>
where
    A: Unsize<[u8]>,
{
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl<A> ops::DerefMut for String2<A>
where
    A: Unsize<[u8]>,
{
    fn deref_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}

impl<'a, A> IntoIterator for &'a String2<A>
where
    A: Unsize<[u8]>,
{
    type Item = &'a u8;
    type IntoIter = slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.iter()
    }
}

impl<'a, A> IntoIterator for &'a mut String2<A>
where
    A: Unsize<[u8]>,
{
    type Item = &'a mut u8;
    type IntoIter = slice::IterMut<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.iter_mut()
    }
}

impl<A, B> PartialEq<String2<B>> for String2<A>
where
    A: Unsize<[u8]>,
    B: Unsize<[u8]>,
{
    fn eq(&self, rhs: &String2<B>) -> bool {
        PartialEq::eq(&**self, &**rhs)
    }
}

impl<A> Eq for String2<A>
where
    A: Unsize<[u8]>,
{
}
