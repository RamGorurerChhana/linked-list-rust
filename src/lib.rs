//! Implementation of Doubly Linked List using raw pointers.
//! Every node in the list will consist of two pointers.
//! One points to its previous node and one points to its next node.
//! `LinkedList` will consists of two pointers. One to point to the head node.
//! Another to point to the tail node.
//! Since raw pointers are used in the implementation, the test cases will be tested with miri as well
//! along with normal `cargo test`.
//! `MIRIFLAGS="-Zmiri-tag-raw-pointers" cargo +nightly-2022-01-21 miri test`
//! Things to be tested specially with miri -
//! - Undefined behaviour
//! - Memory leaks
//! - Dangling pointers
//! ## Methods
//! - append
//! - peek_back
//! - peek_back_mut
//! - clear
//! - contains
//! - cursor_back
//! - cursor_back_mut
//! - cursor_front
//! - cursor_front_mut
//! - peek_front
//! - peek_front_mut
//! - [] is_empty
//! - iter
//! - iter_mut
//! - [] len
//! - [] new
//! - [] push_front
//! - [] push_back
//! - [] pop_front
//! - [] pop_back
//! - remove_at
//! - insert_at
//! - split
//! - splice
//! ## Traits
//! - [] Clone
//! - [] Debug
//! - [] Default
//! - [] Drop
//! - [] Eq
//! - [] PartialEq
//! - [] Ord
//! - [] PartialOrd
//! - [] Extend<&'a T>
//! - [] Extend<T>
//! - [] From<[T;N]>
//! - [] FromIterator<T>
//! - [] Hash
//! - [] IntoIterator for &'a LinkedList<T>
//! - [] IntoIterator for &'a mut LinkedList<T>
//! - [] IntoIterator for LinkedList<T>
//! - [] Send
//! - [] Sync
//!
//!
//! Thats lot of work already 😢
//!

use std::marker::PhantomData;

mod combinatorics;
mod methods;
mod traits;

/// Doubly linked list.
///
/// Example:
/// ```
/// use linked_list::LinkedList;
/// let list: LinkedList<u32> = LinkedList::new();
/// ```
///
pub struct LinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    _phantom: PhantomData<T>,
}

// Node struct represents each node in the list
// contains value owned by the node and two pointers
// to point to previous and next node in the list
#[derive(Debug)]
struct Node<T> {
    val: T,
    prev: *mut Node<T>,
    next: *mut Node<T>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // fn test_test() {
    //     let v = vec![1, 2, 3];
    //     let mut iter = v.into_iter();
    //     println!("{:?}", iter.next());
    // }

    #[test]
    fn test_list_1() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
}
