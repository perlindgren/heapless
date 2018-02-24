use core::marker::Unsize;
use core::{fmt, ops, ptr, slice, str};
use core::mem::transmute;

//FromUtf8Error};

use Vec;

use untagged_option::UntaggedOption;

/// A String, backed by a fixed size array
///
/// String: https://doc.rust-lang.org/std/string/struct.String.html

pub struct String<A>
where
    // FIXME(rust-lang/rust#44580) use "const generics" instead of `Unsize`
    A: Unsize<[u8]>,
{
    buffer: UntaggedOption<A>,
    len: usize,
}

impl<A> String<A>
where
    A: Unsize<[u8]>,
{
    /// Constructs a new, empty `String` backed by the array `A`
    pub const fn new() -> Self {
        String {
            buffer: UntaggedOption::none(),
            len: 0,
        }
    }

    /// Returns the maximum number of elements the vector can hold
    pub fn capacity(&self) -> usize {
        let buffer: &[u8] = unsafe { self.buffer.as_ref() };
        buffer.len()
    }

    /// Clears the string by setting lenght to zero
    pub fn clear(&mut self) {
        self.len = 0;
    }

    ///
    ///
    pub fn from_utf8<B>(&mut self, vec: &Vec<u8, B>)
    // -> Result<(), fmt::FromUtf8Error>
    where
        B: Unsize<[u8]>,
    {
        let buffer: &mut [u8] = unsafe { self.buffer.as_mut() };
        let len = vec.len().min(buffer.len());
        self.len = len;
        let v = &*vec;
        // a bit brute, should use some conversion.
        buffer[0..len].copy_from_slice(&vec[0..len]);
    }

    ///
    pub fn into_bytes(self) -> Vec<u8, A> {
        // unsafe { transmute::<String<A>, Vec<u8, A>>(self) }
        unimplemented!();
    }

    ///
    pub fn as_str(&self) -> &str {
        let buffer: &[u8] = unsafe { self.buffer.as_ref() };
        unsafe { str::from_utf8_unchecked(&buffer[..self.len]) }
    }

    ///
    pub fn as_mut_str(&mut self) -> &mut str {
        let buffer: &mut [u8] = unsafe { self.buffer.as_mut() };
        unsafe { str::from_utf8_unchecked_mut(&mut buffer[..self.len]) }
    }

    ///
    pub fn push_str(&mut self, s: &str) {
        let buffer: &mut [u8] = unsafe { self.buffer.as_mut() };
        let start = self.len;
        let end = start.saturating_add(s.len());
        let new_len = end.min(buffer.len());
        self.len = new_len;
        buffer[start..self.len].copy_from_slice(&s.as_bytes()[0..self.len.saturating_sub(start)]);
    }

    ///
    pub fn push(&mut self, c: char) {
        let buffer: &mut [u8] = unsafe { self.buffer.as_mut() };
        if self.len < buffer.len() {
            buffer[self.len] = c as u8;
            self.len = self.len.saturating_add(1);
        }
    }

    ///
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { self.buffer.as_ref() }
    }

    ///
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe { self.buffer.as_mut() }
    }

    ///
    pub fn truncate(&mut self, new_len: usize) {
        self.len = self.len.min(new_len);
    }

    ///
    pub fn pop(&mut self) -> Option<char> {
        let buffer: &[u8] = unsafe { self.buffer.as_ref() };
        if self.len > 0 {
            self.len = self.len.saturating_sub(1);
            Some(buffer[self.len] as char)
        } else {
            None
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
    pub unsafe fn as_mut_vec(&mut self) -> &mut Vec<u8, A> {
        unimplemented!();
    }

    ///
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    ///
    // pub fn split_off<B>(&mut self, at: usize) -> String<B>
    // where
    //     B: Unsize<[u8]>,
    // {
    //     let buffer: &mut [u8] = unsafe { self.buffer.as_mut() };
    //     if self.len <= at {
    //         let (mut b1, mut b2) = buffer.split_at_mut(at);
    //         self.buffer = b1;
    //         String {
    //             buffer: b2,
    //             len: b2.len(),
    //         }
    //     } else {
    //         String {
    //             buffer: UntaggedOption::none(),

    //         }
    //     }
    // }

    ///
    pub fn from<'a>(&mut self, s: &'a str) {
        let buffer: &mut [u8] = unsafe { self.buffer.as_mut() };
        let len = s.len().min(buffer.len());
        self.len = len;
        buffer[0..len].copy_from_slice(&s.as_bytes()[0..len]);
    }

    ///
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<A> fmt::Debug for String<A>
where
    A: Unsize<[u8]>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let slice: &str = match str::from_utf8(&**self) {
            Ok(s) => s,
            Err(_) => "could not convert to String",
        };
        slice.fmt(f)
    }
}

impl<A> fmt::Display for String<A>
where
    A: Unsize<[u8]>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let slice: &str = match str::from_utf8(&**self) {
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

    fn write_char(&mut self, c: char) -> Result<(), fmt::Error> {
        self.push(c);
        Ok(())
    }
}

impl<A> ops::Deref for String<A>
where
    A: Unsize<[u8]>,
{
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        let buffer: &[u8] = unsafe { self.buffer.as_ref() };
        &buffer[..self.len]
    }
}

impl<A> ops::DerefMut for String<A>
where
    A: Unsize<[u8]>,
{
    fn deref_mut(&mut self) -> &mut [u8] {
        let len = self.len();
        let buffer: &mut [u8] = unsafe { self.buffer.as_mut() };
        &mut buffer[..len]
    }
}

impl<A> Drop for String<A>
where
    A: Unsize<[u8]>,
{
    fn drop(&mut self) {
        unsafe { ptr::drop_in_place(&mut self[..]) }
    }
}

impl<'a, A> IntoIterator for &'a String<A>
where
    A: Unsize<[u8]>,
{
    type Item = &'a u8;
    type IntoIter = slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, A> IntoIterator for &'a mut String<A>
where
    A: Unsize<[u8]>,
{
    type Item = &'a mut u8;
    type IntoIter = slice::IterMut<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
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

// #[cfg(test)]
// mod tests {
//     use Vec;

//     #[test]
//     fn drop() {
//         struct Droppable;
//         impl Droppable {
//             fn new() -> Self {
//                 unsafe {
//                     COUNT += 1;
//                 }
//                 Droppable
//             }
//         }
//         impl Drop for Droppable {
//             fn drop(&mut self) {
//                 unsafe {
//                     COUNT -= 1;
//                 }
//             }
//         }

//         static mut COUNT: i32 = 0;

//         {
//             let mut v: Vec<Droppable, [Droppable; 2]> = Vec::new();
//             v.push(Droppable::new()).unwrap();
//             v.push(Droppable::new()).unwrap();
//             v.pop().unwrap();
//         }

//         assert_eq!(unsafe { COUNT }, 0);

//         {
//             let mut v: Vec<Droppable, [Droppable; 2]> = Vec::new();
//             v.push(Droppable::new()).unwrap();
//             v.push(Droppable::new()).unwrap();
//         }

//         assert_eq!(unsafe { COUNT }, 0);
//     }

//     #[test]
//     fn eq() {
//         let mut xs: Vec<i32, [i32; 4]> = Vec::new();
//         let mut ys: Vec<i32, [i32; 8]> = Vec::new();

//         assert_eq!(xs, ys);

//         xs.push(1).unwrap();
//         ys.push(1).unwrap();

//         assert_eq!(xs, ys);
//     }

//     #[test]
//     fn full() {
//         let mut v: Vec<i32, [i32; 4]> = Vec::new();

//         v.push(0).unwrap();
//         v.push(1).unwrap();
//         v.push(2).unwrap();
//         v.push(3).unwrap();

//         assert!(v.push(4).is_err());
//     }

//     #[test]
//     fn iter() {
//         let mut v: Vec<i32, [i32; 4]> = Vec::new();

//         v.push(0).unwrap();
//         v.push(1).unwrap();
//         v.push(2).unwrap();
//         v.push(3).unwrap();

//         let mut items = v.iter();

//         assert_eq!(items.next(), Some(&0));
//         assert_eq!(items.next(), Some(&1));
//         assert_eq!(items.next(), Some(&2));
//         assert_eq!(items.next(), Some(&3));
//         assert_eq!(items.next(), None);
//     }

//     #[test]
//     fn iter_mut() {
//         let mut v: Vec<i32, [i32; 4]> = Vec::new();

//         v.push(0).unwrap();
//         v.push(1).unwrap();
//         v.push(2).unwrap();
//         v.push(3).unwrap();

//         let mut items = v.iter_mut();

//         assert_eq!(items.next(), Some(&mut 0));
//         assert_eq!(items.next(), Some(&mut 1));
//         assert_eq!(items.next(), Some(&mut 2));
//         assert_eq!(items.next(), Some(&mut 3));
//         assert_eq!(items.next(), None);
//     }

//     #[test]
//     fn push_and_pop() {
//         let mut v: Vec<i32, [i32; 4]> = Vec::new();
//         assert_eq!(v.len(), 0);

//         assert_eq!(v.pop(), None);
//         assert_eq!(v.len(), 0);

//         v.push(0).unwrap();
//         assert_eq!(v.len(), 1);

//         assert_eq!(v.pop(), Some(0));
//         assert_eq!(v.len(), 0);

//         assert_eq!(v.pop(), None);
//         assert_eq!(v.len(), 0);
//     }

//     #[test]
//     fn resize_size_limit() {
//         let mut v: Vec<u8, [u8; 4]> = Vec::new();

//         v.resize(0, 0).unwrap();
//         v.resize(4, 0).unwrap();
//         v.resize(5, 0).err().expect("BufferFullError");
//     }

//     #[test]
//     fn resize_length_cases() {
//         let mut v: Vec<u8, [u8; 4]> = Vec::new();

//         assert_eq!(v.len(), 0);

//         // Grow by 1
//         v.resize(1, 0).unwrap();
//         assert_eq!(v.len(), 1);

//         // Grow by 2
//         v.resize(3, 0).unwrap();
//         assert_eq!(v.len(), 3);

//         // Resize to current size
//         v.resize(3, 0).unwrap();
//         assert_eq!(v.len(), 3);

//         // Shrink by 1
//         v.resize(2, 0).unwrap();
//         assert_eq!(v.len(), 2);

//         // Shrink by 2
//         v.resize(0, 0).unwrap();
//         assert_eq!(v.len(), 0);
//     }

//     #[test]
//     fn resize_contents() {
//         let mut v: Vec<u8, [u8; 4]> = Vec::new();

//         // New entries take supplied value when growing
//         v.resize(1, 17).unwrap();
//         assert_eq!(v[0], 17);

//         // Old values aren't changed when growing
//         v.resize(2, 18).unwrap();
//         assert_eq!(v[0], 17);
//         assert_eq!(v[1], 18);

//         // Old values aren't changed when length unchanged
//         v.resize(2, 0).unwrap();
//         assert_eq!(v[0], 17);
//         assert_eq!(v[1], 18);

//         // Old values aren't changed when shrinking
//         v.resize(1, 0).unwrap();
//         assert_eq!(v[0], 17);
//     }

//     #[test]
//     fn resize_default() {
//         let mut v: Vec<u8, [u8; 4]> = Vec::new();

//         // resize_default is implemented using resize, so just check the
//         // correct value is being written.
//         v.resize_default(1).unwrap();
//         assert_eq!(v[0], 0);
//     }
// }
