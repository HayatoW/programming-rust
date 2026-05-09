//! 検証用の書き捨て

use std::ops::{Deref, DerefMut};

fn main() {
    let mut s = Selector {
        elements: vec!['x', 'y', 'z'],
        current: 2,
    };

    assert_eq!(*s, 'z');
    assert!(s.is_alphabetic());

    *s = 'w';
    assert_eq!(s.elements, ['x', 'y', 'w']);
}

struct Selector<T> {
    elements: Vec<T>,
    current: usize,
}

impl<T> Deref for Selector<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements[self.current]
    }
}
