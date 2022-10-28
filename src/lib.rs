use std::fmt;

struct Iter<'a, T>(Option<&'a Node<T>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.map(|node| {
            self.0 = node.next.as_deref();
            &node.val
        })
    }
}

#[derive(Debug)]
struct List<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn len(&self) -> usize {
        match self.head.as_ref() {
            None => 0,
            Some(node) => node.len(1),
        }
    }

    fn push(&mut self, val: T) {
        let mut new_node = Node::new(val);
        if let Some(head) = self.head.take() {
            new_node.next = Some(head);
        }
        self.head = Some(Box::new(new_node));
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|mut node| {
            if let Some(nxt) = node.next.take() {
                self.head = Some(nxt);
            }
            node.val
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }

    fn insert_at(&mut self, val: T, at_idx: usize) -> Result<(), usize> {
        if at_idx == 0 {
            self.push(val);
            return Ok(());
        }
        if self.head.is_none() {
            return Err(0);
        }
        let mut curr_node = self.head.as_mut();
        for i in 1..at_idx {
            if curr_node.is_none() {
                return Err(i);
            }
            curr_node = curr_node.unwrap().next.as_mut();
        }
        if curr_node.is_none() {
            return Err(at_idx);
        }
        curr_node.as_mut().unwrap().insert_after(val);
        Ok(())
    }

    fn iter(&self) -> Iter<T> {
        Iter(self.head.as_deref())
    }



}

impl<T: fmt::Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HEAD -> ")?;
        for v in  self.iter() {
            write!(f, "{} -> ", v)?;
        }
        write!(f, "END")?;
        Ok(())
    }
}

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self { val, next: None }
    }

    fn len(&self, prev_len: usize) -> usize {
        match self.next.as_ref() {
            None => prev_len,
            Some(node) => node.len(prev_len + 1),
        }
    }

    fn insert_after(&mut self, val: T) {
        let mut new_node = Self::new(val);
        if let Some(nxt) = self.next.take() {
            new_node.next = Some(nxt);
        }
        self.next = Some(Box::new(new_node));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_1() {
        let mut list = List::new();
        assert!(list.head.is_none());
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
        list.push(3);
        assert!(list.head.is_some());
        assert_eq!(list.len(), 1);
        list.push(4);
        list.push(5);
        list.push(6);
        assert_eq!(list.peek(), Some(&6));
        assert_eq!(list.len(), 4);
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.peek(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn test_insert_at() {
        let mut list = List::new();
        list.push(3);
        assert!(list.insert_at(10, 1).is_ok());
        assert!(list.insert_at(20, 1).is_ok());
        assert!(list.insert_at(30, 1).is_ok());
        assert!(list.insert_at(50, 2).is_ok());
        assert_eq!(list.len(), 5);
        assert!(list.insert_at(50, 6).is_err());
        assert!(list.insert_at(50, 5).is_ok());
    }

    #[test]
    fn test_iter(){
        let mut list = List::new();
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_display(){
        let mut list: List<i32> = List::new();
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);
        assert_eq!(list.to_string(), "HEAD -> 5 -> 4 -> 3 -> 2 -> END".to_string());
    }




}
