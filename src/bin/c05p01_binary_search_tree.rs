use std::mem;

fn main() {
    println!("Aum Namah Sivaya!!! BST")
}

#[derive(Clone, Debug, PartialEq)]
pub struct IoTDevice {
    pub numerical_id: u64,
    pub address: String,
}

type Tree = Option<Box<Node>>;

struct Node {
    pub dev: IoTDevice,
    left: Tree,
    right: Tree,
}

pub struct DeviceRegistry {
    root: Tree,
    pub length: u64,
}

impl IoTDevice {
    pub fn new(numerical_id: u64, address: String) -> Self {
        Self {
            numerical_id,
            address,
        }
    }
}

impl Node {
    pub fn new(dev: IoTDevice) -> Tree {
        Some(Box::new(Node {
            dev,
            left: None,
            right: None,
        }))
    }
}

impl DeviceRegistry {
    pub fn new_empty() -> Self {
        Self {
            root: None,
            length: 0,
        }
    }

    pub fn add(&mut self, device: IoTDevice) {
        self.length += 1;
        let root = mem::replace(&mut self.root, None);
        self.root = self.add_rec(root, device);
    }

    fn add_rec(&mut self, node: Tree, device: IoTDevice) -> Tree {
        match node {
            Some(mut n) => {
                if n.dev.numerical_id <= device.numerical_id {
                    n.left = self.add_rec(n.left, device);
                } else {
                    n.right = self.add_rec(n.right, device);
                }
                Some(n)
            }
            _ => Node::new(device),
        }
    }

    pub fn find(&self, numerical_id: u64) -> Option<IoTDevice> {
        self.find_r(&self.root, numerical_id)
    }

    fn find_r(&self, node: &Tree, numerical_id: u64) -> Option<IoTDevice> {
        match node {
            Some(n) => {
                if n.dev.numerical_id == numerical_id {
                    Some(n.dev.clone())
                } else if n.dev.numerical_id < numerical_id {
                    self.find_r(&n.left, numerical_id)
                } else {
                    self.find_r(&n.right, numerical_id)
                }
            }
            _ => None,
        }
    }

    pub fn walk(&self, callback: impl Fn(&IoTDevice) -> ()) {
        self.walk_in_order(&self.root, &callback);
    }

    fn walk_in_order(&self, node: &Tree, callback: &impl Fn(&IoTDevice) -> ()) {
        if let Some(n) = node {
            self.walk_in_order(&n.left, callback);
            callback(&n.dev);
            self.walk_in_order(&n.right, callback);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn binary_search_tree_add() {
        let mut tree = DeviceRegistry::new_empty();
        tree.add(IoTDevice {
            numerical_id: 4,
            address: "Kasi".to_owned(),
        });
        tree.add(IoTDevice {
            numerical_id: 3,
            address: "Kailash".to_owned(),
        });
        tree.add(IoTDevice {
            numerical_id: 2,
            address: "Kedarnath".to_owned(),
        });
        tree.add(IoTDevice {
            numerical_id: 1,
            address: "Kanchi".to_owned(),
        });
        tree.add(IoTDevice {
            numerical_id: 5,
            address: "Kalahasti".to_owned(),
        });
        tree.add(IoTDevice {
            numerical_id: 6,
            address: "Palani".to_owned(),
        });
        tree.add(IoTDevice {
            numerical_id: 7,
            address: "Kanipakam".to_owned(),
        });
        assert_eq!(tree.length, 7);
    }

    #[test]
    fn binary_search_tree_walk_in_order() {
        let city_list = [
            (5, "Kasi"),
            (4, "Kedarnath"),
            (3, "Srisailam"),
            (2, "Bodhgaya"),
            (1, "Rameswaram"),
            (6, "Ujjain"),
            (7, "Mahabaleswar"),
            (8, "Omkareshwar"),
            (9, "Grishneswar"),
            (10, "Somnath"),
        ];
        let mut tree = DeviceRegistry::new_empty();
        city_list.map(|item| {
            tree.add(IoTDevice {
                numerical_id: item.0 as u64,
                address: item.1.to_owned(),
            })
        });

        assert_eq!(tree.length, 10);
        let v: RefCell<Vec<IoTDevice>> = RefCell::new(vec![]);
        tree.walk(|n| v.borrow_mut().push(n.clone()));
        assert_eq!(v.into_inner().len(), tree.length as usize);
    }

    #[test]
    fn binary_search_tree_find() {
        let city_list = [
            (5, "Kasi"),
            (4, "Kedarnath"),
            (3, "Srisailam"),
            (2, "Bodhgaya"),
            (1, "Rameswaram"),
            (6, "Ujjain"),
            (7, "Mahabaleswar"),
            (8, "Omkareshwar"),
            (9, "Grishneswar"),
            (10, "Somnath"),
        ];
        let mut tree = DeviceRegistry::new_empty();
        city_list.map(|item| {
            tree.add(IoTDevice {
                numerical_id: item.0 as u64,
                address: item.1.to_owned(),
            })
        });

        assert_eq!(tree.length, 10);
        assert_eq!(tree.find(100), None);
        assert_eq!(
            tree.find(4),
            Some(IoTDevice::new(4, "Kedarnath".to_owned()))
        );
        assert_eq!(tree.find(5), Some(IoTDevice::new(5, "Kasi".to_owned())));
    }
}
