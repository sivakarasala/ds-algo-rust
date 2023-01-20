use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct Node {
    value: String,
    next: Link,
    prev: Link,
}

type Link = Option<Rc<RefCell<Node>>>;

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }))
    }
}

pub struct ListIterator {
    current: Link,
}

impl ListIterator {
    fn new(start_at: Link) -> Self {
        Self { current: start_at }
    }
}

impl Iterator for ListIterator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.next.clone()
            }
            None => None,
        };
        result
    }
}

impl DoubleEndedIterator for ListIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.prev.clone()
            }
            None => None,
        };
        result
    }
}

#[derive(Debug, Clone)]
pub struct BetterTransactionLog {
    head: Link,
    tail: Link,
    pub length: u64,
}

impl BetterTransactionLog {
    pub fn new_empty() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: String) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => {
                old.borrow_mut().next = Some(new.clone());
                new.borrow_mut().prev = Some(old);
            }
            None => self.head = Some(new.clone()),
        };
        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                next.borrow_mut().prev = None;
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .value
        })
    }

    pub fn back_iter(self) -> ListIterator {
        ListIterator::new(self.tail)
    }

    pub fn iter(&self) -> ListIterator {
        ListIterator::new(self.head.clone())
    }
}

impl IntoIterator for BetterTransactionLog {
    type Item = String;
    type IntoIter = ListIterator;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(self.head)
    }
}

fn main() {
    println!("Hara Hara Mahadev!!! Doubly Linked List");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn better_transaction_log_append() {
        let mut transaction_log = BetterTransactionLog::new_empty();
        assert_eq!(transaction_log.length, 0);
        transaction_log.append("Siva".to_owned());
        transaction_log.append("Shambho".to_owned());
        transaction_log.append("Shankara".to_owned());
        assert_eq!(transaction_log.length, 3);
        assert_eq!(transaction_log.pop(), Some("Siva".to_owned()));
        assert_eq!(transaction_log.pop(), Some("Shambho".to_owned()));
        assert_eq!(transaction_log.pop(), Some("Shankara".to_owned()));
        assert_eq!(transaction_log.pop(), None);
    }

    #[test]
    fn better_transaction_log_pop() {
        let mut list = BetterTransactionLog::new_empty();
        assert_eq!(list.pop(), None);
        list.append("Siva".to_owned());
        list.append("Shambho".to_owned());
        list.append("Shankara".to_owned());
        assert_eq!(list.length, 3);
        assert_eq!(list.pop(), Some("Siva".to_owned()));
        assert_eq!(list.pop(), Some("Shambho".to_owned()));
        assert_eq!(list.pop(), Some("Shankara".to_owned()));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn better_transaction_log_iterator() {
        let mut list = BetterTransactionLog::new_empty();
        assert_eq!(list.pop(), None);
        list.append("Siva".to_owned());
        list.append("Shambho".to_owned());
        list.append("Shankara".to_owned());
        let mut iter = list.clone().into_iter();
        assert_eq!(iter.next(), Some("Siva".to_owned()));
        assert_eq!(iter.next(), Some("Shambho".to_owned()));
        assert_eq!(iter.next(), Some("Shankara".to_owned()));

        let mut iter = list.clone().back_iter();
        assert_eq!(iter.next_back(), Some("Shankara".to_owned()));
        assert_eq!(iter.next_back(), Some("Shambho".to_owned()));
        assert_eq!(iter.next_back(), Some("Siva".to_owned()));
    }
}
