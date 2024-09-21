use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T, next: Link<T>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { value, next }))
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
}

#[derive(Debug)]
struct LinkedListNodeIter<T> {
    current: Link<T>,
}

impl<T> Iterator for LinkedListNodeIter<T> {
    type Item = Link<T>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current.take() {
            None => None,
            Some(node) => {
                self.current = node.borrow().next.clone();
                Some(Option::from(node))
            }
        }
    }
}

impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    fn push_back(&mut self, value: T) {
        let new = Node::new(value, None);
        match self.tail.take() {
            None => {
                self.head = Some(new.clone());
                self.tail = Some(new);
            }
            Some(node) => {
                node.borrow_mut().next = Some(new.clone());
                self.tail = Some(new)
            }
        }
    }

    fn push_front(&mut self, value: T) {
        let new = Node::new(value, None);
        match self.head.take() {
            None => {
                self.head = Some(new.clone());
                self.tail = Some(new);
            }
            Some(node) => {
                new.borrow_mut().next = Some(node.clone());
                self.head = Some(new.clone());
            }
        }
    }

    fn push_after_n(&mut self, n: usize, value: T) -> Result<(), &str> {
        let nth_node = self.iter().nth(n).ok_or("n over list length")?.unwrap();
        let child = nth_node.borrow().next.clone();
        nth_node.borrow_mut().next = Some(Node::new(value, child));
        Ok(())
    }

    fn iter(&self) -> LinkedListNodeIter<T> {
        LinkedListNodeIter {
            current: self.head.clone(),
        }
    }

    fn get_nth(&self, nth: usize) -> Result<Link<T>, &str> {
        self.iter().nth(nth).ok_or("nth over list length")
    }

    fn update_nth(&self, nth: usize, value: T) -> Result<(), &str> {
        let node = self.iter().nth(nth).ok_or("nth over list length")?.unwrap();
        node.borrow_mut().value = value;
        Ok(())
    }

    fn split_on_nth(self, n: usize) -> Result<(LinkedList<T>, LinkedList<T>), &'static str> {
        let nth_node = self.iter().nth(n - 1).ok_or("n over list length")?.unwrap();
        let mut sec_lst = LinkedList::new();
        sec_lst.head = nth_node.borrow().next.clone();
        sec_lst.tail = self.tail.clone();
        nth_node.borrow_mut().next = None;
        Ok((self, sec_lst))
    }
}

impl<T: Debug> Display for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for node in self.iter() {
            match node {
                None => {}
                Some(n) => {
                    if n.borrow().next.is_none() {
                        write!(f, "{:?}", &n.borrow().value)?
                    } else {
                        write!(f, "{:?}, ", &n.borrow().value)?
                    }
                }
            }
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_back() {
        let mut list = LinkedList::<i32>::new();

        list.push_back(1);
        assert_eq!(
            list.head.clone().unwrap().borrow().value,
            list.tail.clone().unwrap().borrow().value
        );

        list.push_back(2);
        assert_eq!(list.tail.clone().unwrap().borrow().value, 2);

        list.push_back(3);
        assert_eq!(list.tail.clone().unwrap().borrow().value, 3);

        let targets = [1, 2, 3];
        for (node, value) in list.iter().zip(targets) {
            assert_eq!(node.clone().unwrap().borrow().value, value)
        }
    }

    #[test]
    fn test_push_front() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(3);
        assert_eq!(
            list.head.clone().unwrap().borrow().value,
            list.tail.clone().unwrap().borrow().value
        );

        list.push_front(2);
        assert_eq!(list.head.clone().unwrap().borrow().value, 2);

        list.push_front(1);
        assert_eq!(list.head.clone().unwrap().borrow().value, 1);

        let targets = [1, 2, 3];
        for (node, value) in list.iter().zip(targets) {
            assert_eq!(node.clone().unwrap().borrow().value, value)
        }
    }

    #[test]
    fn test_push_after_n() {
        let mut list = LinkedList::<i32>::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        list.push_after_n(0, 77);
        list.push_after_n(2, 78);

        assert_eq!(list.get_nth(1).unwrap().unwrap().borrow().value, 77);
        assert_eq!(list.get_nth(3).unwrap().unwrap().borrow().value, 78)
    }

    #[test]
    fn test_get_nth() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let targets = [1, 2, 3];
        for (i, (node, value)) in list.iter().zip(targets).enumerate() {
            assert_eq!(list.get_nth(i).unwrap().unwrap().borrow().value, value)
        }
    }

    #[test]
    fn test_update_nth() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        list.update_nth(0, 4);
        list.update_nth(1, 5);
        list.update_nth(2, 6);

        let targets = [4, 5, 6];
        for (node, value) in list.iter().zip(targets) {
            assert_eq!(node.clone().unwrap().borrow().value, value)
        }
    }

    #[test]
    fn test_split_by_n() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_back(5);

        let (first, sec) = list.split_on_nth(3).unwrap();

        let target_first = [1, 2, 3];
        for (node, value) in first.iter().zip(target_first) {
            assert_eq!(node.clone().unwrap().borrow().value, value)
        }

        let target_sec = [4, 5];
        for (node, value) in sec.iter().zip(target_sec) {
            assert_eq!(node.clone().unwrap().borrow().value, value)
        }
    }

    #[test]
    fn iter() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let target_sec = [1, 2, 3];
        for (node, value) in list.iter().zip(target_sec) {
            assert_eq!(node.clone().unwrap().borrow().value, value)
        }
    }
}

fn main() {
    let mut list = LinkedList::<i32>::new();
    println!("List empty {list}");

    // show push back
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    println!("List after pushes back {list}");

    list.push_front(4);
    list.push_front(6);

    println!("List after pushes front {list}");

    let _ = list.push_after_n(0, 88888);

    println!("List after pushes after nth {list}");

    let _ = list.update_nth(0, 200);
    println!("List after update nth {list}");

    println!(
        "Get nth (1) element {}",
        list.get_nth(1).unwrap().unwrap().borrow().value
    );

    println!(
        "Get nth (2) element {}",
        list.get_nth(2).unwrap().unwrap().borrow().value
    );

    println!("List before split {list}");
    let (first, sec) = list.split_on_nth(4).unwrap();

    println!("First part of split list {first}");
    println!("Sec part of split list {sec}");
}
