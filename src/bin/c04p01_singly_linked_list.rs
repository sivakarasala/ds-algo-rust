use std::{cell::RefCell, rc::Rc};

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    value: String,
    next: SingleLink,
}

impl Node {
    // create a new node
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}
struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
    pub length: u64,
}

impl TransactionLog {
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
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        };
        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
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
}

fn main() {
    println!("Hara Hara Mahadev!!! Singly Linked List");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transaction_log_append() {
        let mut transaction_log = TransactionLog::new_empty();
        assert_eq!(transaction_log.length, 0);
        transaction_log.append("Siva".to_owned());
        transaction_log.append("Krishna".to_owned());
        transaction_log.append("Shambho".to_owned());
        assert_eq!(transaction_log.length, 3);
        assert_eq!(transaction_log.pop(), Some("Siva".to_owned()));
        assert_eq!(transaction_log.pop(), Some("Krishna".to_owned()));
        assert_eq!(transaction_log.pop(), Some("Shambho".to_owned()));
        assert_eq!(transaction_log.pop(), None);
    }

    #[test]
    fn transaction_log_pop() {
        let mut list = TransactionLog::new_empty();
        assert_eq!(list.pop(), None);
        list.append("Siva".to_owned());
        list.append("Krishna".to_owned());
        list.append("Shambho".to_owned());
        assert_eq!(list.pop(), Some("Siva".to_owned()));
        assert_eq!(list.pop(), Some("Krishna".to_owned()));
        assert_eq!(list.pop(), Some("Shambho".to_owned()));
        assert_eq!(list.pop(), None);
    }
}
