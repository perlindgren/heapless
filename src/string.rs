use core::marker::Unsize;
use core::{fmt, ops, ptr, slice, str};
use core::str::Utf8Error;

use {BufferFullError, Vec};
use core::ops::Deref;

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
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let mut s: String<[u8; 4]> = String::new();
    /// ```
    #[inline]
    pub const fn new() -> Self {
        String { vec: Vec::new() }
    }

    /// Constructs a new, empty `String` backed by a Vec<u8,[u8;N]> from an &str.
    /// Cannot be called from a `static context (not `const fn`).
    ///
    /// Current implementation silently truncates the result to the capacity of the String.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let s: String<[u8; 4]> = String::from("abc");
    /// assert!(s.len() == 3);
    ///
    /// let s: String<[_; 4]> = String::from("abcde");
    /// assert!(s.len() == 4);
    /// ```
    // Todo, Trait implementation?
    // Return a Vec::Result?
    //
    pub fn from(s: &str) -> Self {
        let mut new = String::new();
        new.push_str(s);
        new
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
    /// ```
    /// let mut v: Vec<u8, [u8; 8]> = Vec::new();
    /// v.push('a' as u8).unwrap();
    /// v.push('b' as u8).unwrap();
    /// ```
    ///
    /// let s = String::from_utf8(v).unwrap();
    /// assert!(s.len() == 2);
    ///
    /// Incorrect bytes:
    ///
    /// ```
    /// // some invalid bytes, in a vector
    /// let mut v: Vec<u8, [u8; 8]> = Vec::new();
    /// v.push(0).unwrap();
    /// v.push(159).unwrap();
    /// v.push(146).unwrap();
    /// v.push(150).unwrap();
    /// assert!(String::from_utf8(v).is_err());
    /// ```
    #[inline]
    pub fn from_utf8(vec: Vec<u8, A>) -> Result<String<A>, Utf8Error> {
        {
            let buffer: &[u8] = unsafe { vec.buffer.as_ref() };
            str::from_utf8(&buffer[0..vec.len])?;
        }
        Ok(String { vec: vec })
    }

    /// Converts a vector of bytes to a `String` without checking that the
    /// string contains valid UTF-8.
    ///
    /// See the safe version, [`from_utf8`], for more details.
    #[inline]
    pub unsafe fn from_utf8_unchecked(vec: Vec<u8, A>) -> String<A> {
        String { vec: vec }
    }

    /// Converts a `String` into a byte vector.
    ///
    /// This consumes the `String`, so we do not need to copy its contents.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let s: String<[_; 4]> = String::from("ab");
    /// let b = s.into_bytes();
    /// assert!(b.len() == 2);
    ///
    /// assert_eq!(&['a' as u8, 'b' as u8], &b[..]);
    /// ```
    #[inline]
    pub fn into_bytes(self) -> Vec<u8, A> {
        self.vec
    }

    /// Extracts a string slice containing the entire string.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let mut s: String<[_; 4]> = String::from("ab");
    /// assert!(s.as_str() == "ab");
    ///
    /// let s1 = s.as_str();
    /// s.push('c'); // <- cannot borrow `s` as mutable because it is also borrowed as immutable
    /// ```
    #[inline]
    pub fn as_str(&self) -> &str {
        let buffer: &[u8] = unsafe { self.vec.buffer.as_ref() };
        unsafe { str::from_utf8_unchecked(&buffer[..self.vec.len]) }
    }

    /// Converts a `String` into a mutable string slice.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let mut s: String<[_; 4]> = String::from("ab");
    /// let s = s.as_mut_str();
    /// s.make_ascii_uppercase();
    /// ```
    #[inline]
    pub fn as_mut_str(&mut self) -> &mut str {
        let buffer: &mut [u8] = unsafe { self.vec.buffer.as_mut() };
        unsafe { str::from_utf8_unchecked_mut(&mut buffer[..self.vec.len]) }
    }

    /// Appends a given string slice onto the end of this `String`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let mut s = String::from("foo");
    ///
    /// s.push_str("bar");
    ///
    /// assert_eq!("foobar", s);
    /// ```
    //
    // TODO, should be implemented using `extend_from_slice` on Vec
    // (Hower, this is not yet implemented in Vec, so we do a hack.)
    // In the future we will return a Vec::Result
    #[inline]
    pub fn push_str(&mut self, s: &str) {
        let buffer: &mut [u8] = unsafe { self.vec.buffer.as_mut() };
        let start = self.vec.len;
        let end = start.saturating_add(s.len());
        let new_len = end.min(buffer.len());
        self.vec.len = new_len;
        buffer[start..self.vec.len]
            .copy_from_slice(&s.as_bytes()[0..self.vec.len.saturating_sub(start)]);
    }

    /// Returns the maximum number of elements the String can hold
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let mut s: String<[u8; 4]> = String::new();
    /// assert!(s.capacity() == 4);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        let buffer: &[u8] = unsafe { self.vec.buffer.as_ref() };
        buffer.len()
    }

    /// Appends the given [`char`] to the end of this `String`.
    /// Assumes ch.len_utf8() == 1
    ///
    /// [`char`]: ../../std/primitive.char.html
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let mut s: String<[u8; 8]> = String::from("abc");
    ///
    /// s.push('1').unwrap();
    /// s.push('2').unwrap();
    /// s.push('3').unwrap();
    ///
    /// assert!("abc123" == s.as_str());
    ///
    /// assert_eq!("abc123", s);
    /// ```
    #[inline]
    pub fn push(&mut self, c: char) -> Result<(), BufferFullError> {
        self.vec.push(c as u8)
    }

    /// Returns a byte slice of this `String`'s contents.
    ///
    /// The inverse of this method is [`from_utf8`].
    ///
    /// [`from_utf8`]: #method.from_utf8
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let s: String<[u8; 8]> = String::from("hello");
    ///
    /// assert_eq!(&[104, 101, 108, 108, 111], s.as_bytes());
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.vec
    }

    /// Shortens this `String` to the specified length.
    ///
    /// If `new_len` is greater than the string's current length, this has no
    /// effect.
    ///
    /// Note that this method has no effect on the allocated capacity
    /// of the string
    ///
    /// # Panics
    ///
    /// Panics if `new_len` does not lie on a [`char`] boundary.
    ///
    /// [`char`]: ../../std/primitive.char.html
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let mut s = String::from("hello");
    ///
    /// s.truncate(2);
    ///
    /// assert_eq!("he", s);
    /// ```
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
    pub fn remove(&mut self, _idx: usize) -> char {
        unimplemented!();
    }

    ///
    pub fn insert(&mut self, _idx: usize, _ch: char) {
        unimplemented!();
    }

    ///
    pub fn insert_str(&mut self, _idx: usize, _string: &str) {
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
    // pub fn from<'a>(&mut self, s: &'a str) -> Result<(), BufferFullError> {
    //     match self.vec.len <= s.len() {
    //         true => {
    //             let buffer: &mut [u8] = unsafe { self.vec.buffer.as_mut() };
    //             let len = s.len().min(buffer.len());
    //             self.vec.len = len;
    //             buffer[0..len].copy_from_slice(&s.as_bytes()[0..len]);
    //             Ok(())
    //         }
    //         _ => Err(BufferFullError),
    //     }
    // }

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

    fn ne(&self, rhs: &String<B>) -> bool {
        PartialEq::ne(&**self, &**rhs)
    }
}

macro_rules! impl_eq {
    ($lhs:ty, $rhs: ty) => {

        impl<'a, 'b, A> PartialEq<$rhs> for $lhs
        where
            A: Unsize<[u8]>,
        {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool { PartialEq::eq(&self[..], &other[..]) }
            #[inline]
            fn ne(&self, other: &$rhs) -> bool { PartialEq::ne(&self[..], &other[..]) }
        }

        impl<'a, 'b, A> PartialEq<$lhs> for $rhs
        where
            A: Unsize<[u8]>,
        {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool { PartialEq::eq(&self[..], &other[..]) }
            #[inline]
            fn ne(&self, other: &$lhs) -> bool { PartialEq::ne(&self[..], &other[..]) }
        }

    }
}

impl_eq! { String<A>, str }
impl_eq! { String<A>, &'a str }

impl<A> Eq for String<A>
where
    A: Unsize<[u8]>,
{
}
