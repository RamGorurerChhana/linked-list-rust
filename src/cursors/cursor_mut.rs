use crate::LinkedList;
use crate::Node;

pub struct CursorMut<'a, T> {
    pub(super) curr: *mut Node<T>,
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
        unsafe { (&mut (*self.curr).val, self.index) }
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
                (&mut (*self.list.tail).val, self.length - 1)
            } else {
                // otherwise return the value from prev of curr
                (&mut (*(*self.curr).prev).val, self.index - 1)
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
                (&mut (*self.list.head).val, 0)
            } else {
                // otherwise return the value from next of curr
                (&mut (*(*self.curr).next).val, self.index + 1)
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
            (*self.curr).next = new_node;
        }
        // if at last element then adjust tail pointer of the list
        if self.index == self.length - 1 {
            self.list.tail = new_node;
        }
        // increase length of the cursor
        self.length += 1;
        // move the cursor to next node
        self.move_next();
    }
}
