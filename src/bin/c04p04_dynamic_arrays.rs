use std::cmp;

const MIN_SIZE: usize = 10;

type Node = Option<u64>;

pub struct TimestampSaver {
    buf: Box<[Node]>,
    cap: usize,
    pub length: usize,
}

impl TimestampSaver {
    pub fn new_empty() -> Self {
        Self {
            buf: Box::new([None; MIN_SIZE]),
            length: 0,
            cap: MIN_SIZE,
        }
    }
    fn grow(&mut self, min_cap: usize) {
        let old_cap = self.buf.len();
        let mut new_cap = old_cap + (old_cap >> 1);

        new_cap = cmp::max(new_cap, min_cap);
        new_cap = cmp::min(new_cap, usize::max_value());
        let current = self.buf.clone();
        self.cap = new_cap;

        self.buf = vec![None; new_cap].into_boxed_slice();
        self.buf[..current.len()].clone_from_slice(&current);
    }

    pub fn append(&mut self, value: u64) {
        if self.length == self.cap {
            self.grow(self.length + 1);
        }
        self.buf[self.length] = Some(value);
        self.length += 1;
    }

    pub fn at(&mut self, index: usize) -> Option<u64> {
        if self.length > index {
            self.buf[index]
        } else {
            None
        }
    }
}

impl IntoIterator for TimestampSaver {
    type Item = u64;
    type IntoIter = ListIterator;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(0, self.buf)
    }
}

pub struct ListIterator {
    current: usize,
    data: Box<[Node]>,
}

impl ListIterator {
    pub fn new(index: usize, buf: Box<[Node]>) -> Self {
        Self {
            current: index,
            data: buf,
        }
    }
}

impl Iterator for ListIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.data.len() {
            let item = self.data[self.current];
            self.current += 1;
            item
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for ListIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current < self.data.len() {
            let item = self.data[self.current];
            if self.current == 0 {
                self.current = self.data.len() - 1;
            } else {
                self.current -= 1;
            }
            item
        } else {
            None
        }
    }
}

fn main() {
    println!("Aum Namah Sivaya!!! Vector -> Dynamic Array");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dynamic_array_append() {
        let mut list = TimestampSaver::new_empty();
        let max: usize = 1000;
        for i in 0..max {
            list.append(i as u64);
        }
        assert_eq!(list.length, max);
    }

    #[test]
    fn dynamic_array_at() {
        let mut list = TimestampSaver::new_empty();
        let max: usize = 1000;
        for i in 0..max {
            list.append(i as u64);
        }
        assert_eq!(list.length, max);
        for i in 0..max {
            assert_eq!(list.at(i), Some(i as u64));
        }

        assert_eq!(list.at(max + 1), None);
    }

    #[test]
    fn dynamic_array_iterate() {
        let mut list = TimestampSaver::new_empty();
        list.append(1);
        list.append(2);
        list.append(3);
        list.append(4);
        assert_eq!(list.length, 4);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), None);
    }
}
