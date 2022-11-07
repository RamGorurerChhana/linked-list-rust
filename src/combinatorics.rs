use crate::Link;
use crate::LinkMut;
use crate::LinkedList;
use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::ptr;

pub struct Iter<'a, T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
    _phantom: &'a PhantomData<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    /// Implement `Iterator` trait for Iter.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_front(1); list.push_front(2); list.push_front(3);
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        // if head is null then no more items left in the list
        // return None
        if self.head.is_null() {
            return None;
        }

        unsafe {
            // copy the current head
            let curr = self.head;
            // set head as the `next` of the current head
            self.head = (*self.head).next;
            // if head is becoming null then reset tail as null too
            if self.head.is_null() {
                self.tail = ptr::null_mut();
            }
            // return the reference of `val` from the current head
            Some(&(*curr).val)
        }
    }

    // Returns a tuple where the first element is the lower bound,
    // and the second element is the upper bound.
    // It provides an estimate for the length of the iterator
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    /// Implement `DoubleEndedIterator` trait for Iter.
    /// Which allow iterating over the list from the back.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_front(1); list.push_front(2); list.push_front(3);
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next_back(), Some(&1));
    /// assert_eq!(iter.next_back(), Some(&2));
    /// assert_eq!(iter.next_back(), Some(&3));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn next_back(&mut self) -> Option<Self::Item> {
        // if tail is null then no more items left in the list
        // return None
        if self.tail.is_null() {
            return None;
        }

        unsafe {
            // copy the current tail
            let curr = self.tail;
            // set tail as the `prev` of the current tail
            self.tail = (*self.tail).prev;
            // if tail is becoming null then reset head as null too
            if self.tail.is_null() {
                self.head = ptr::null_mut();
            }
            // return the reference of `val` from the current tail
            Some(&(*curr).val)
        }
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {}
impl<'a, T> FusedIterator for Iter<'a, T> {}

pub struct IterMut<'a, T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
    _phantom: &'a PhantomData<T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    /// Implement `Iterator` trait for IterMut.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_front(1); list.push_front(2); list.push_front(3);
    /// let mut iter = list.iter_mut();
    /// assert_eq!(iter.next(), Some(&mut 3));
    /// assert_eq!(iter.next(), Some(&mut 2));
    /// assert_eq!(iter.next(), Some(&mut 1));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        // if head is null then no more items left in the list
        // return None
        if self.head.is_null() {
            return None;
        }

        unsafe {
            // copy the current head
            let curr = self.head as LinkMut<T>;
            // set head as the `next` of the current head
            self.head = (*self.head).next;
            // if head is becoming null then reset tail as null too
            if self.head.is_null() {
                self.tail = ptr::null_mut();
            }
            // return the reference of `val` from the current head
            Some(&mut (*curr).val)
        }
    }

    // Returns a tuple where the first element is the lower bound,
    // and the second element is the upper bound.
    // It provides an estimate for the length of the iterator
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    /// Implement `DoubleEndedIterator` trait for IterMut.
    /// Which allow iterating over the list from the back.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_front(1); list.push_front(2); list.push_front(3);
    /// let mut iter = list.iter_mut();
    /// assert_eq!(iter.next_back(), Some(&mut 1));
    /// assert_eq!(iter.next_back(), Some(&mut 2));
    /// assert_eq!(iter.next_back(), Some(&mut 3));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn next_back(&mut self) -> Option<Self::Item> {
        // if tail is null then no more items left in the list
        // return None
        if self.tail.is_null() {
            return None;
        }

        unsafe {
            // copy the current tail
            let curr = self.tail as LinkMut<T>;
            // set tail as the `prev` of the current tail
            self.tail = (*self.tail).prev;
            // if tail is becoming null then reset head as null too
            if self.tail.is_null() {
                self.head = ptr::null_mut();
            }
            // return the reference of `val` from the current tail
            Some(&mut (*curr).val)
        }
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {}
impl<'a, T> FusedIterator for IterMut<'a, T> {}

/// An iterator that owns the LinkedList. Returns the owned value T when `next` is called.
/// This struct can be instantiated by calling `into_iter` method in the LinkedList.
pub struct IntoIter<T>(LinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    /// Implement `Iterator` trait for IntoIter.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_front(1); list.push_front(2); list.push_front(3);
    /// let mut iter = list.into_iter();
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        // `next` method will just pop nodes from the front
        // since IntoIter owns the list `pop_front` should be fine
        self.0.pop_front()
    }

    // Returns a tuple where the first element is the lower bound,
    // and the second element is the upper bound.
    // It provides an estimate for the length of the iterator
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.0.len();
        (size, Some(size))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    /// Implement `DoubleEndedIterator` trait for IntoIter.
    /// Which allow iterating over the list from the back.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_front(1); list.push_front(2); list.push_front(3);
    /// let mut iter = list.into_iter();
    /// assert_eq!(iter.next_back(), Some(1));
    /// assert_eq!(iter.next_back(), Some(2));
    /// assert_eq!(iter.next_back(), Some(3));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}
impl<T> FusedIterator for IntoIter<T> {}

impl<T> LinkedList<T> {
    /// Returns a new instance of `Iter` struct.
    /// Returns &T when `next` method is called on the iterator.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_front(1); list.push_front(2); list.push_front(3);
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            tail: self.tail,
            size: self.len(),
            _phantom: &PhantomData,
        }
    }

    /// Returns a new instance of `IterMut` struct.
    /// Returns &mut T when `next` method is called on the iterator.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_front(1); list.push_front(2); list.push_front(3);
    /// let mut iter = list.iter_mut();
    /// assert_eq!(iter.next(), Some(&mut 3));
    /// assert_eq!(iter.next(), Some(&mut 2));
    /// assert_eq!(iter.next(), Some(&mut 1));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_mut(&self) -> IterMut<'_, T> {
        IterMut {
            head: self.head,
            tail: self.tail,
            size: self.len(),
            _phantom: &PhantomData,
        }
    }

    /// Returns a new instance of `IntoIter`.
    /// This method takes the list by value.
    /// Returns owned value T when `next` method is called on the iterator.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_front(1); list.push_front(2); list.push_front(3);
    /// let mut iter = list.into_iter();
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}
