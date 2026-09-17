mod gap {
    use std::ops::Range;

    pub struct GapBuffer<T> {
        /// 要素を格納する場所。必要とする容量を持つが、長さは常にゼロになる
        /// GapBuffer は要素とギャップを `Vec` が「使用していない領域」に置く
        storage: Vec<T>,

        /// `storage` 内で初期化されていない要素の範囲を指す
        /// この範囲の前後の領域は常に初期化されている
        gap: Range<usize>,
    }

    impl<T> GapBuffer<T> {
        pub fn new() -> GapBuffer<T> {
            GapBuffer {
                storage: Vec::new(),
                gap: 0..0,
            }
        }

        pub fn capacity(&self) -> usize {
            self.storage.capacity()
        }

        pub fn len(&self) -> usize {
            self.capacity() - self.gap.len()
        }

        pub fn position(&self) -> usize {
            self.gap.start
        }

        unsafe fn space(&self, index: usize) -> *const T {
            unsafe { self.storage.as_ptr().add(index) }
        }

        unsafe fn space_mut(&mut self, index: usize) -> *mut T {
            unsafe { self.storage.as_mut_ptr().add(index) }
        }

        fn index_to_raw(&self, index: usize) -> usize {
            if index < self.gap.start {
                index
            } else {
                index + self.gap.len()
            }
        }

        pub fn get(&self, index: usize) -> Option<&T> {
            let raw = self.index_to_raw(index);
            if raw < self.capacity() {
                unsafe { Some(&*self.space(raw)) }
            } else {
                None
            }
        }

        pub fn set_position(&mut self, pos: usize) {
            if pos > self.len() {
                panic!("index {} out of range for GapBuffer", pos);
            }

            unsafe {
                let gap = self.gap.clone();
                if pos > gap.start {
                    let distance = pos - gap.start;
                    std::ptr::copy(self.space(gap.end), self.space_mut(gap.start), distance);
                } else if pos < gap.start {
                    let distance = gap.start - pos;
                    std::ptr::copy(
                        self.space(pos),
                        self.space_mut(gap.end - distance),
                        distance,
                    );
                }

                self.gap = pos..pos + gap.len();
            }
        }

        pub fn insert(&mut self, elt: T) {
            if self.gap.len() == 0 {
                self.enlarge_gap();
            }

            unsafe {
                let index = self.gap.start;
                std::ptr::write(self.space_mut(index), elt);
            }

            self.gap.start += 1;
        }

        pub fn insert_iter<I>(&mut self, iterable: I)
        where
            I: IntoIterator<Item = T>,
        {
            for item in iterable {
                self.insert(item);
            }
        }

        pub fn remove(&mut self) -> Option<T> {
            if self.gap.end == self.capacity() {
                return None;
            }

            let element = unsafe { std::ptr::read(self.space(self.gap.end)) };
            self.gap.end += 1;
            Some(element)
        }

        fn enlarge_gap(&mut self) {
            let mut new_capacity = self.capacity() * 2;
            if new_capacity == 0 {
                new_capacity = 4;
            }

            let mut new = Vec::with_capacity(new_capacity);
            let after_gap = self.capacity() - self.gap.end;
            let new_gap = self.gap.start..new.capacity() - after_gap;

            unsafe {
                std::ptr::copy_nonoverlapping(self.space(0), new.as_mut_ptr(), self.gap.start);
                let new_gap_end = new.as_mut_ptr().offset(new_gap.end as isize);
                std::ptr::copy_nonoverlapping(self.space(self.gap.end), new_gap_end, after_gap);
            }

            self.storage = new;
            self.gap = new_gap;
        }
    }

    impl<T> Drop for GapBuffer<T> {
        fn drop(&mut self) {
            unsafe {
                for i in 0..self.gap.start {
                    std::ptr::drop_in_place(self.space_mut(i));
                }
                for i in self.gap.end..self.capacity() {
                    std::ptr::drop_in_place(self.space_mut(i));
                }
            }
        }
    }

    pub struct Iter<'a, T> {
        buffer: &'a GapBuffer<T>,
        pos: usize,
    }

    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            if self.pos >= self.buffer.len() {
                None
            } else {
                self.pos += 1;
                self.buffer.get(self.pos - 1)
            }
        }
    }

    impl<'a, T: 'a> IntoIterator for &'a GapBuffer<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;
        fn into_iter(self) -> Self::IntoIter {
            Iter {
                buffer: self,
                pos: 0,
            }
        }
    }

    impl GapBuffer<char> {
        pub fn get_string(&self) -> String {
            let mut text = String::new();
            text.extend(self);
            text
        }
    }

    use std::fmt;
    impl<T: fmt::Debug> fmt::Debug for GapBuffer<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let indices = (0..self.gap.start).chain(self.gap.end..self.capacity());
            let elements = indices.map(|i| unsafe { &*self.space(i) });
            f.debug_list().entries(elements).finish()
        }
    }
}
