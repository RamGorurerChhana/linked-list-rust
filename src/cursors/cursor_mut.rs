use std::ptr;

use crate::Link;
use crate::LinkMut;
use crate::LinkedList;
use crate::Node;
use crate::RemoveUnderCursorError;

pub struct CursorMut<'a, T> {
    pub(super) curr: Link<T>,
    pub(super) list: &'a mut LinkedList<T>,
    pub(super) index: usize,
    pub(super) length: usize,
}

impl<'a, T> CursorMut<'a, T> {
    /// Returns the mutable reference to the value under the cursor and its index
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = [1, 2, 3].into_iter().collect::<LinkedList<i32>>();
    /// let cursor = list.cursor_front_mut().unwrap();
    /// assert_eq!(cursor.current_mut(), (&mut 1, 0));
    /// let cursor = list.cursor_back_mut().unwrap();
    /// assert_eq!(cursor.current_mut(), (&mut 3, 2));
    /// ```
    pub fn current_mut(&self) -> (&mut T, usize) {
        // if `curr` contains null then panic
        if self.curr.is_null() {
            unreachable!("Cursor cannot contain null pointer");
        }

        // return the reference to the value under curr pointer
        unsafe {
            let curr = self.curr as LinkMut<T>;
            (&mut (*curr).val, self.index)
        }
    }

    /// Returns the mutable reference to the value previous to the node under the cursor and its index
    /// Note: index will wrap around 0 to (length - 1) of the list.
    /// If the cursor is on the index 0 then this method will return
    /// the index of the last node in the list.
    /// For list with one node, previous node is same as the current node.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = [1].into_iter().collect::<LinkedList<i32>>();
    /// let cursor = list.cursor_front_mut().unwrap();
    /// assert_eq!(cursor.prev_mut(), (&mut 1, 0));
    /// let cursor = list.cursor_back_mut().unwrap();
    /// assert_eq!(cursor.prev_mut(), (&mut 1, 0));
    /// let mut list = [1, 2, 3].into_iter().collect::<LinkedList<i32>>();
    /// let cursor = list.cursor_front_mut().unwrap();
    /// assert_eq!(cursor.prev_mut(), (&mut 3, 2));
    /// let (x, _) = cursor.prev_mut();
    /// *x += 1;
    /// assert_eq!(cursor.prev_mut(), (&mut 4, 2));
    /// ```
    pub fn prev_mut(&self) -> (&mut T, usize) {
        // if `curr` contains null then panic
        if self.curr.is_null() {
            unreachable!("Cursor cannot contain null pointer");
        }
        // self.length must be greater than zero
        // otherwise cursor can't be created
        assert!(self.length > 0);
        unsafe {
            if self.index == 0 {
                // when on the first element return the value from tail of the list
                let tail = self.list.tail as LinkMut<T>;
                (&mut (*tail).val, self.length - 1)
            } else {
                // otherwise return the value from prev of curr
                let curr_prev = (*self.curr).prev as LinkMut<T>;
                (&mut (*curr_prev).val, self.index - 1)
            }
        }
    }

    /// Returns the mutable reference to the value next to the node under the cursor and its index
    /// Note: index will wrap around 0 to (length - 1) of the list.
    /// If the cursor is on the last node then this method will return
    /// value from first node and index as 0.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = [1].into_iter().collect::<LinkedList<i32>>();
    /// let cursor = list.cursor_front_mut().unwrap();
    /// assert_eq!(cursor.next_mut(), (&mut 1, 0));
    /// let cursor = list.cursor_back_mut().unwrap();
    /// assert_eq!(cursor.next_mut(), (&mut 1, 0));
    /// let mut list = [1, 2, 3].into_iter().collect::<LinkedList<i32>>();
    /// let cursor = list.cursor_back_mut().unwrap();
    /// assert_eq!(cursor.next_mut(), (&mut 1, 0));
    /// ```
    pub fn next_mut(&self) -> (&mut T, usize) {
        // if `curr` contains null then panic
        if self.curr.is_null() {
            unreachable!("Cursor cannot contain null pointer");
        }
        // self.length must be greater than zero
        // otherwise cursor can't be created
        assert!(self.length > 0);
        unsafe {
            if self.index == self.length - 1 {
                // when on the last element return the value from head of the list
                let head = self.list.head as LinkMut<T>;
                (&mut (*head).val, 0)
            } else {
                // otherwise return the value from next of curr
                let curr_next = (*self.curr).next as LinkMut<T>;
                (&mut (*curr_next).val, self.index + 1)
            }
        }
    }

    /// Move the cursor one node towards front.
    /// When the cursor is on the first node then this method moves the cursor to the last node.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = [1, 2, 3].into_iter().collect::<LinkedList<i32>>();
    /// let mut cursor = list.cursor_front_mut().unwrap();
    /// cursor.move_prev();
    /// assert_eq!(cursor.current_mut(), (&mut 3, 2));
    /// assert_eq!(cursor.prev_mut(), (&mut 2, 1));
    /// assert_eq!(cursor.next_mut(), (&mut 1, 0));
    /// ```
    pub fn move_prev(&mut self) {
        // when on the first node move the cursor to the last node
        if self.index == 0 {
            self.curr = self.list.tail;
            self.index = self.length - 1;
            return;
        }
        unsafe {
            // move the cursor to the prev node
            self.curr = (*self.curr).prev;
            self.index -= 1;
        }
    }

    /// Move the cursor one node towards back.
    /// When the cursor is on the last node then this method moves the cursor to the first node.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = [1, 2, 3].into_iter().collect::<LinkedList<i32>>();
    /// let mut cursor = list.cursor_back_mut().unwrap();
    /// cursor.move_next();
    /// assert_eq!(cursor.current_mut(), (&mut 1, 0));
    /// assert_eq!(cursor.prev_mut(), (&mut 3, 2));
    /// assert_eq!(cursor.next_mut(), (&mut 2, 1));
    /// ```
    pub fn move_next(&mut self) {
        // when on the last node move the cursor to the first node
        if self.index == self.length - 1 {
            self.curr = self.list.head;
            self.index = 0;
            return;
        }
        unsafe {
            // move the cursor to the next node
            self.curr = (*self.curr).next;
            self.index += 1;
        }
    }

    /// Move the cursor no of steps at once.
    /// index will wrap around according to the no of steps given.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::from([1, 2, 3, 4, 5]);
    /// let mut cursor = list.cursor_front_mut().unwrap();
    /// cursor.step_by(2);
    /// assert_eq!(cursor.current_mut(), (&mut 3, 2));
    /// assert_eq!(cursor.prev_mut(), (&mut 2, 1));
    /// assert_eq!(cursor.next_mut(), (&mut 4, 3));
    /// cursor.step_by(10);
    /// assert_eq!(cursor.current_mut(), (&mut 3, 2));
    /// assert_eq!(cursor.prev_mut(), (&mut 2, 1));
    /// assert_eq!(cursor.next_mut(), (&mut 4, 3));
    /// ```
    pub fn step_by(&mut self, steps: usize) {
        // calculate the final_index the cursor to move to
        let final_index = (self.index + (steps % self.length)) % self.length;
        // if final_index is less than current index then call move_prev repeatedly
        if self.index > final_index {
            (final_index..self.index).for_each(|_| self.move_prev());
        }
        // call move_next repeatedly to reach final_index
        (self.index..final_index).for_each(|_| self.move_next());
    }

    /// Move the cursor backward no of steps at once.
    /// index will wrap around according to the no of steps given.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::from([1, 2, 3, 4, 5]);
    /// let mut cursor = list.cursor_front_mut().unwrap();
    /// cursor.step_by_backward(2);
    /// assert_eq!(cursor.current_mut(), (&mut 4, 3));
    /// cursor.step_by_backward(10);
    /// assert_eq!(cursor.current_mut(), (&mut 4, 3));
    /// ```
    pub fn step_by_backward(&mut self, steps: usize) {
        self.step_by(self.length - (steps % self.length));
    }

    /// Insert a new node after the node cursor currently pointing
    /// Cursor also moves one node towards back.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::from([1, 2, 3, 4, 5]);
    /// let mut cursor = list.cursor_back_mut().unwrap();
    /// cursor.insert(6);
    /// assert_eq!(cursor.current_mut(), (&mut 6, 5));
    /// assert_eq!(cursor.prev_mut(), (&mut 5, 4));
    /// assert_eq!(cursor.next_mut(), (&mut 1, 0));
    /// assert_eq!(list.len(), 6);
    /// ```
    pub fn insert(&mut self, elem: T) {
        // create a new_node
        let mut new_node = Box::into_raw(Box::new(Node::new(elem)));
        unsafe {
            // set next of curr as the next of new_node
            (*new_node).next = (*self.curr).next;
            // set the current node as the prev of new_node
            (*new_node).prev = self.curr;
            // set next of curr as the new_node
            let curr = self.curr as LinkMut<T>;
            (*curr).next = new_node as Link<T>;
        }
        // if at last element then adjust tail pointer of the list
        if self.index == self.length - 1 {
            self.list.tail = new_node as Link<T>;
        }
        // increase length of the cursor
        self.length += 1;
        // move the cursor to next node
        self.move_next();
    }

    /// Removes the node under the cursor and cursor moves to be node next
    /// Note: Returns error if the list contain only one node
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::from([1, 2, 3, 4, 5]);
    /// let mut cursor = list.cursor_back_mut().unwrap();
    /// assert!(cursor.remove().is_ok());
    /// assert_eq!(cursor.current_mut(), (&mut 1, 0));
    /// assert_eq!(cursor.prev_mut(), (&mut 4, 3));
    /// assert_eq!(cursor.next_mut(), (&mut 2, 1));
    /// assert_eq!(list.len(), 4);
    /// let mut cursor = list.cursor_front_mut().unwrap();
    /// assert!(cursor.remove().is_ok());
    /// assert_eq!(cursor.current_mut(), (&mut 2, 0));
    /// assert_eq!(cursor.prev_mut(), (&mut 4, 2));
    /// assert_eq!(cursor.next_mut(), (&mut 3, 1));
    /// assert_eq!(list.len(), 3);
    /// ```
    pub fn remove(&mut self) -> Result<T, RemoveUnderCursorError> {
        if self.length < 2 {
            return Err(RemoveUnderCursorError);
        }
        unsafe {
            // take out the node currently under the cursor
            let boxed_node = Box::from_raw(self.curr as LinkMut<T>);
            // if the `prev` of `boxed_node` is not null
            // then `next` of `prev` of `boxed_node` will point to `next` of `boxed_node`
            if !boxed_node.prev.is_null() {
                let node_prev = boxed_node.prev as LinkMut<T>;
                (*node_prev).next = boxed_node.next;
            } else {
                // boxed_node is the first node in the list
                // `head` pointer of the list now point to `next` of `boxed_node`
                self.list.head = boxed_node.next;
            }

            // if the `next` of `boxed_node` is not null
            // then `prev` of `next` of `boxed_node` will point to `prev` of `boxed_node`
            if !boxed_node.next.is_null() {
                let node_next = boxed_node.next as LinkMut<T>;
                (*node_next).prev = boxed_node.prev;
                // curr will now point to `next` of `boxed_node`
                self.curr = boxed_node.next;
            } else {
                // boxed_node is the last node in the list
                // tail pointer of the list now point to `prev` of `boxed_node`
                self.list.tail = boxed_node.prev;
                // curr will now point to head of the list
                self.curr = self.list.head;
            }
            // adjust length of the cursor and index
            self.length -= 1;
            self.index = self.index % self.length;

            Ok(boxed_node.val)
        }
    }

    /// Split the list at the node where the cursor is pointing to.
    /// After split the node under the cursor becomes the last node of the list.
    /// A new list is generated and returned with all rest of the elements
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::from([1, 2, 3, 4, 5]);
    /// let mut cursor = list.cursor_front_mut().unwrap();
    /// let new_list = cursor.split();
    /// assert_eq!(list.len(), 1);
    /// assert_eq!(new_list.len(), 4);
    /// assert_eq!(new_list.peek_front(), Some(&2));
    /// assert_eq!(list.peek_back(), Some(&1));
    /// ```
    pub fn split(&mut self) -> LinkedList<T> {
        let mut new_list = LinkedList::new();
        unsafe {
            if !(*self.curr).next.is_null() {
                new_list.tail = self.list.tail;
                new_list.head = (*self.curr).next;
                (*(self.curr as LinkMut<T>)).next = ptr::null();
                self.list.tail = self.curr;
                self.length = self.list.len();
            }
        }

        new_list
    }

    /// Insert the given list into the underlying list.
    /// Cursor advances until the last node of the other list.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::from([1, 2, 3, 4, 5]);
    /// let mut cursor = list.cursor_front_mut().unwrap();
    /// cursor.step_by(2);
    /// cursor.splice(LinkedList::from([10, 11]));
    /// assert_eq!(cursor.current_mut(), (&mut 11, 4));
    /// assert_eq!(cursor.prev_mut(), (&mut 10, 3));
    /// assert_eq!(cursor.next_mut(), (&mut 4, 5));
    /// assert_eq!(list.len(), 7);
    /// ```
    pub fn splice(&mut self, mut other: LinkedList<T>) {
        if other.is_empty() {
            return;
        }
        let other_len = other.len();
        unsafe {
            if !(*self.curr).next.is_null() {
                let curr_next = (*self.curr).next as LinkMut<T>;
                let other_tail = other.tail as LinkMut<T>;
                (*curr_next).prev = other.tail;
                (*other_tail).next = (*self.curr).next;
            } else {
                self.list.tail = other.tail;
            }
            (*(self.curr as LinkMut<T>)).next = other.head;
            self.curr = other.tail;
        }
        self.length += other_len;
        self.index += other_len;
        other.head = ptr::null();
        other.tail = ptr::null();
    }
}
