use crate::LinkedList;
use crate::Node;

pub struct Cursor<'a, T> {
    pub(super) curr: *mut Node<T>,
    pub(super) list: &'a LinkedList<T>,
    pub(super) index: usize,
    pub(super) length: usize,
}

impl<'a, T> Cursor<'a, T> {
    /// Returns the reference to the value under the cursor and its index
    /// ```
    /// use linked_list::LinkedList;
    /// let list = [1, 2, 3].into_iter().collect::<LinkedList<i32>>();
    /// let cursor = list.cursor_front().unwrap();
    /// assert_eq!(cursor.current(), (&1, 0));
    /// let cursor = list.cursor_back().unwrap();
    /// assert_eq!(cursor.current(), (&3, 2));
    /// ```
    ///
    /// List must not be allowed to be mutated while the cursor is active.
    /// Below must fail to compile
    /// ```compile_fail
    /// use linked_list::LinkedList;
    /// let mut list = [1, 2, 3].into_iter().collect::<LinkedList<i32>>();
    /// let cursor = list.cursor_front().unwrap();
    /// assert_eq!(cursor.current(), (&1, 0));
    /// list.push_front(0);
    /// assert_eq!(cursor.current(), (&0, 0));
    /// ```
    pub fn current(&self) -> (&T, usize) {
        // if `curr` contains null then panic
        if self.curr.is_null() {
            unreachable!("Cursor cannot contain null pointer");
        }

        // return the reference to the value under curr pointer
        unsafe { (&(*self.curr).val, self.index) }
    }

    /// Returns the reference to the value previous to the node under the cursor and its index
    /// Note: index will wrap around 0 to (length - 1) of the list.
    /// If the cursor is on the index 0 then this method will return
    /// the index of the last node in the list.
    /// For list with one node, previous node is same as the current node.
    /// ```
    /// use linked_list::LinkedList;
    /// let list = [1].into_iter().collect::<LinkedList<i32>>();
    /// let cursor = list.cursor_front().unwrap();
    /// assert_eq!(cursor.prev(), (&1, 0));
    /// let cursor = list.cursor_back().unwrap();
    /// assert_eq!(cursor.prev(), (&1, 0));
    /// let list = [1, 2, 3].into_iter().collect::<LinkedList<i32>>();
    /// let cursor = list.cursor_front().unwrap();
    /// assert_eq!(cursor.prev(), (&3, 2));
    /// ```
    pub fn prev(&self) -> (&T, usize) {
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
                (&(*self.list.tail).val, self.length - 1)
            } else {
                // otherwise return the value from prev of curr
                (&(*(*self.curr).prev).val, self.index - 1)
            }
        }
    }

    /// Returns the reference to the value next to the node under the cursor and its index
    /// Note: index will wrap around 0 to (length - 1) of the list.
    /// If the cursor is on the last node then this method will return
    /// value from first node and index as 0.
    /// ```
    /// use linked_list::LinkedList;
    /// let list = [1].into_iter().collect::<LinkedList<i32>>();
    /// let cursor = list.cursor_front().unwrap();
    /// assert_eq!(cursor.next(), (&1, 0));
    /// let cursor = list.cursor_back().unwrap();
    /// assert_eq!(cursor.next(), (&1, 0));
    /// let list = [1, 2, 3].into_iter().collect::<LinkedList<i32>>();
    /// let cursor = list.cursor_back().unwrap();
    /// assert_eq!(cursor.next(), (&1, 0));
    /// ```
    pub fn next(&self) -> (&T, usize) {
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
                (&(*self.list.head).val, 0)
            } else {
                // otherwise return the value from next of curr
                (&(*(*self.curr).next).val, self.index + 1)
            }
        }
    }

    /// Move the cursor one node towards back.
    /// When the cursor is on the last node then this method moves the cursor to the first node.
    /// ```
    /// use linked_list::LinkedList;
    /// let list = [1, 2, 3].into_iter().collect::<LinkedList<i32>>();
    /// let mut cursor = list.cursor_back().unwrap();
    /// cursor.move_next();
    /// assert_eq!(cursor.current(), (&1, 0));
    /// assert_eq!(cursor.prev(), (&3, 2));
    /// assert_eq!(cursor.next(), (&2, 1));
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

    /// Move the cursor one node towards front.
    /// When the cursor is on the first node then this method moves the cursor to the last node.
    /// ```
    /// use linked_list::LinkedList;
    /// let list = [1, 2, 3].into_iter().collect::<LinkedList<i32>>();
    /// let mut cursor = list.cursor_front().unwrap();
    /// cursor.move_prev();
    /// assert_eq!(cursor.current(), (&3, 2));
    /// assert_eq!(cursor.prev(), (&2, 1));
    /// assert_eq!(cursor.next(), (&1, 0));
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

    /// Move the cursor no of steps at once.
    /// index will wrap around according to the no of steps given.
    /// ```
    /// use linked_list::LinkedList;
    /// let list = LinkedList::from([1, 2, 3, 4, 5]);
    /// let mut cursor = list.cursor_front().unwrap();
    /// cursor.step_by(2);
    /// assert_eq!(cursor.current(), (&3, 2));
    /// assert_eq!(cursor.prev(), (&2, 1));
    /// assert_eq!(cursor.next(), (&4, 3));
    /// cursor.step_by(10);
    /// assert_eq!(cursor.current(), (&3, 2));
    /// assert_eq!(cursor.prev(), (&2, 1));
    /// assert_eq!(cursor.next(), (&4, 3));
    /// ```
    pub fn step_by(&mut self, steps: usize) {
        // calculate the final_index the cursor to move to
        let final_index = (self.index + (steps % self.length)) % self.length;
        // if final_index is same as current index then no move required
        if final_index == self.index {
            return;
        }
        // decide which is closer? forward move or backward move
        let (direction, steps) = if final_index > self.index {
            let dist = final_index - self.index;
            let alt_dist = self.length - dist;
            if alt_dist < dist {
                ("backward", alt_dist)
            } else {
                ("forward", dist)
            }
        } else {
            let dist = self.index - final_index;
            let alt_dist = self.length - dist;
            if dist < alt_dist {
                ("backward", dist)
            } else {
                ("forward", alt_dist)
            }
        };

        if direction == "backward" {
            (0..steps).for_each(|_| self.move_prev());
        } else {
            (0..steps).for_each(|_| self.move_next());
        }
    }

    /// Move the cursor backward no of steps at once.
    /// index will wrap around according to the no of steps given.
    /// ```
    /// use linked_list::LinkedList;
    /// let list = LinkedList::from([1, 2, 3, 4, 5]);
    /// let mut cursor = list.cursor_front().unwrap();
    /// cursor.step_by_backward(2);
    /// assert_eq!(cursor.current(), (&4, 3));
    /// cursor.step_by_backward(10);
    /// assert_eq!(cursor.current(), (&4, 3));
    /// ```
    pub fn step_by_backward(&mut self, steps: usize) {
        self.step_by(self.length - (steps % self.length));
    }
}
