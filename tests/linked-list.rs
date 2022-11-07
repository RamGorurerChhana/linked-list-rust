use linked_list::*;

#[test]
fn test_empty_list() {
    let list: LinkedList<i32> = LinkedList::new();
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

#[test]
fn test_push_pop_front() {
    let mut list = LinkedList::new();
    assert_eq!(list.peek_front(), None);
    assert_eq!(list.peek_back(), None);
    assert_eq!(list.peek_front_mut(), None);
    assert_eq!(list.peek_back_mut(), None);
    list.push_front(1);
    assert_eq!(list.len(), 1);
    assert!(!list.is_empty());
    list.push_front(2);
    list.push_front(3);
    assert_eq!(list.len(), 3);
    assert_eq!(list.peek_back(), Some(&1));
    assert_eq!(list.peek_back_mut(), Some(&mut 1));
    assert_eq!(list.peek_front(), Some(&3));
    assert_eq!(list.peek_front_mut(), Some(&mut 3));
    let e = list.peek_back_mut().unwrap();
    *e += 1;
    assert_eq!(list.peek_back(), Some(&2));
    let e = list.peek_front_mut().unwrap();
    *e += 1;
    assert_eq!(list.peek_front(), Some(&4));
    assert_eq!(list.pop_front(), Some(4));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.pop_front(), None);
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
}

#[test]
fn test_push_pop_back() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    assert!(!list.is_empty());
    assert_eq!(list.len(), 3);
    assert_eq!(list.peek_back(), Some(&3));
    let e = list.peek_back_mut().unwrap();
    *e += 1;
    assert_eq!(list.peek_back(), Some(&4));
    assert_eq!(list.peek_front(), Some(&1));
    let e = list.peek_front_mut().unwrap();
    *e += 1;
    assert_eq!(list.peek_front(), Some(&2));
    assert_eq!(list.pop_back(), Some(4));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_back(), None);
    assert_eq!(list.pop_back(), None);
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
}

#[test]
fn test_push_back_pop_front() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), None);
}

#[test]
fn test_push_front_pop_back() {
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), None);
}
#[test]
fn test_push_pop_mixup() {
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_back(2);
    list.push_back(3);
    list.push_front(4);
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_front(), Some(4));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.pop_back(), None);
}

#[test]
fn test_push_front_no_pop() {
    let mut list = LinkedList::new();
    (0..4).into_iter().for_each(|n| list.push_front(n));
    assert_eq!(list.len(), 4);
}

#[test]
fn test_push_back_no_pop() {
    let mut list = LinkedList::new();
    (0..4).into_iter().for_each(|n| list.push_back(n));
    assert_eq!(list.len(), 4);
}

#[test]
fn test_list_clear() {
    let mut list = (0..10).into_iter().collect::<LinkedList<u32>>();
    assert_eq!(list.len(), 10);
    list.clear();
    assert_eq!(list.len(), 0);
    assert_eq!(list.is_empty(), true);
    assert_eq!(list.peek_back(), None);
    assert_eq!(list.peek_front(), None);
}

#[test]
fn test_list_contains() {
    let mut list = LinkedList::new();
    assert_eq!(list.contains(&1), false);
    (0..10).for_each(|n| list.push_back(n));
    assert_eq!(list.contains(&0), true);
    assert_eq!(list.contains(&7), true);
    assert_eq!(list.contains(&9), true);
    assert_eq!(list.contains(&10), false);
    list.clear();
    assert_eq!(list.contains(&1), false);
}

#[test]
fn test_list_append() {
    let mut list = (0..10).into_iter().collect::<LinkedList<u32>>();
    let old_list = list.clone();
    list.append(&mut LinkedList::new());
    assert_eq!(list, old_list);
    let mut list = LinkedList::new();
    let mut other = LinkedList::from([1, 2, 3, 4]);
    let old_other = other.clone();
    list.append(&mut other);
    assert_eq!(list, old_other);
    assert_eq!(other.is_empty(), true);
    let mut list = LinkedList::new();
    list.push_front(1);
    list.append(&mut LinkedList::from([2, 3]));
    list.push_back(4);
    assert_eq!(list.pop_back(), Some(4));
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), None);
}

#[test]
fn test_list_insert_at() {
    let mut list = LinkedList::new();
    list.insert_at(1, 0);
    list.insert_at(2, 0);
    list.insert_at(3, 0);
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), None);
    let mut list = (0..10).into_iter().collect::<LinkedList<u32>>();
    (0..10).rev().for_each(|n| list.insert_at(n, 5));
    assert_eq!(list.peek_front(), Some(&0));
    assert_eq!(list.peek_back(), Some(&9));
    let expected = (0..5).chain(0..10).chain(5..10);
    assert!(list.iter().cloned().eq(expected));
    let mut list = LinkedList::new();
    list.extend((0..5).into_iter());
    list.insert_at(5, 10);
    assert_eq!(list.pop_back(), Some(5));
}

#[test]
fn test_memory_cleanup() {
    use std::cell::Cell;
    struct DropCounter<'a>(&'a Cell<usize>);
    impl<'a> Drop for DropCounter<'a> {
        fn drop(&mut self) {
            let num = self.0.get();
            self.0.set(num + 1);
        }
    }
    let total = 10;
    let counter = Cell::new(0);
    let list = (0..total)
        .map(|_| DropCounter(&counter))
        .collect::<LinkedList<_>>();
    assert_eq!(list.len(), total);
    drop(list);
    assert_eq!(counter.get(), total);
}

#[test]
fn test_remove_at() {
    let mut list = LinkedList::new();
    assert_eq!(list.remove_at(0).is_err(), true);
    (0..4).for_each(|n| list.insert_at(n, 0));
    (0..4).for_each(|n| assert_eq!(list.remove_at(0).unwrap(), 3 - n));
    assert_eq!(list.remove_at(0), Err(RemoveUnderCursorError));
    assert!(list.is_empty());
    let mut list = LinkedList::from([1, 2, 3, 4, 5]);
    (1..6)
        .rev()
        .for_each(|n| assert_eq!(list.remove_at(n - 1).unwrap(), n));
    let mut list = LinkedList::from([1, 2, 3, 4, 5]);
    assert_eq!(list.remove_at(2).unwrap(), 3);
    assert_eq!(list.remove_at(2).unwrap(), 4);
    assert_eq!(list.remove_at(2).unwrap(), 5);
    assert_eq!(list.remove_at(2).unwrap(), 1);
    assert_eq!(list.remove_at(2).unwrap(), 2);
}

#[test]
fn test_split_at() {
    let mut list = LinkedList::new();
    list.extend(0..5);
    let other = list.split_at(5);
    assert_eq!(list.len(), 5);
    assert_eq!(other.len(), 0);
    let other = list.split_at(3);
    assert_eq!(list.len(), 4);
    assert_eq!(other.len(), 1);
    let other = list.split_at(0);
    assert_eq!(list.len(), 1);
    assert_eq!(other.len(), 3);
}
