use crate::LinkedList;
use crate::Node;
use crate::RemoveUnderCursorError;
use std::marker::PhantomData;
use std::ptr;

impl<T> Node<T> {
    // creates a new instance of Node
    // prev and next pointers are initialized with null values
    pub(crate) fn new(val: T) -> Self {
        Self {
            val,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }
}

impl<T> LinkedList<T> {
    /// Creates a new instance of the LinkedList.
    /// The `head` and `tail` pointers are initialized with `null`.
    /// ```
    /// use linked_list::LinkedList;
    /// let list: LinkedList<i32> = LinkedList::new();
    /// ```
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            _phantom: PhantomData,
        }
    }

    /// Returns the length of the liked list.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.len(), 0);
    /// list.push_front(1); list.push_front(2); list.push_front(3);
    /// assert_eq!(list.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut curr = self.head;
        // walk over each node in the list and increment the counter
        // This operation takes O(n) time.
        // Another approach is to store len in the `LinkedList` struct
        // and to update len with each push and pop operation.
        while !curr.is_null() {
            count += 1;
            unsafe {
                curr = (*curr).next;
            }
        }

        count
    }

    /// Returns true if the list is empty.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.is_empty(), true);
    /// list.push_front('a');
    /// assert_eq!(list.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    /// Removes all nodes from the list.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = [1, 2, 3].into_iter().collect::<LinkedList<i32>>();
    /// assert_eq!(list.len(), 3);
    /// assert_eq!(list.is_empty(), false);
    /// list.clear();
    /// assert_eq!(list.len(), 0);
    /// assert_eq!(list.is_empty(), true);
    /// ```
    pub fn clear(&mut self) {
        // pop off all nodes from the list until list is empty
        while self.pop_front().is_some() {}
    }

    /// Returns true if the list contains the given value otherwise false.
    /// ```
    /// use linked_list::LinkedList;
    /// let list = [1, 2, 3].into_iter().collect::<LinkedList<i32>>();
    /// assert_eq!(list.contains(&3), true);
    /// assert_eq!(list.contains(&4), false);
    /// ```
    pub fn contains(&self, item: &T) -> bool
    where
        T: PartialEq,
    {
        self.iter().any(|x| x == item)
    }

    /// Adds a new node onto the front of the list.
    /// `head` pointer will point to the newly created node after this operation.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_front(2);
    /// ```
    pub fn push_front(&mut self, elem: T) {
        // create a new node with elem
        // create a raw pointer with the Node
        // Box::new method will allocate the memory in the heap
        // Box::into_raw method will provide the raw pointer of the allocated memory
        let new_node = Box::into_raw(Box::new(Node::new(elem)));
        unsafe {
            // set current head as the next of new_node
            (*new_node).next = self.head;
        }
        // if head is null that means list is empty
        // tail will also point to new_node.
        if self.head.is_null() {
            self.tail = new_node;
        } else {
            unsafe {
                // otherwise `prev` of current head will point to new_node
                (*self.head).prev = new_node;
            }
        }
        // and head will be set to new_node
        self.head = new_node;
    }

    /// Adds a new node onto the back of the list.
    /// `tail` pointer will point to the newly created node after this operation.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(2);
    /// ```
    pub fn push_back(&mut self, elem: T) {
        // create a new node with elem
        // create a raw pointer with the Node
        // Box::new method will allocate the memory in the heap
        // Box::into_raw method will provide the raw pointer of the allocated memory
        let new_node = Box::into_raw(Box::new(Node::new(elem)));
        unsafe {
            // set current tail as the prev of new_node
            (*new_node).prev = self.tail;
        }
        // if tail is null that means list is empty
        // head will also point to new_node.
        if self.tail.is_null() {
            self.head = new_node;
        } else {
            // otherwise `next` of current tail will point to new_node
            unsafe {
                (*self.tail).next = new_node;
            }
        }
        // and tail will be set to new_node
        self.tail = new_node;
    }

    /// Removes a node from the front of the list and returns the contained value.
    /// Returns `None` if the list is empty.
    /// `head` pointer will move backward one step after this operation.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_front(1); list.push_front(2); list.push_front(3);
    /// assert_eq!(list.pop_front(), Some(3));
    /// assert_eq!(list.pop_front(), Some(2));
    /// assert_eq!(list.pop_front(), Some(1));
    /// assert_eq!(list.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        // if head is null that means list is empty return None
        if self.head.is_null() {
            return None;
        }

        unsafe {
            // take out the node head currently pointing to.
            // turn into a Box so that it can be dropped
            let node = Box::from_raw(self.head);
            // set head as the next of the current head
            self.head = node.next;
            // if head is becoming null that means list is empty
            // reset tail to null as well
            if self.head.is_null() {
                self.tail = ptr::null_mut();
            }
            // return the value inside node
            Some(node.val)
        }
    }

    /// Removes a node from the front of the list and returns the contained value.
    /// Returns `None` if the list is empty.
    /// `tail` pointer will move forward one step after this operation.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_front(1); list.push_front(2); list.push_front(3);
    /// assert_eq!(list.pop_back(), Some(1));
    /// assert_eq!(list.pop_back(), Some(2));
    /// assert_eq!(list.pop_back(), Some(3));
    /// assert_eq!(list.pop_back(), None);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        // if tail is null that means list is empty return None
        if self.tail.is_null() {
            return None;
        }

        unsafe {
            // take out the node tail currently pointing to.
            // turn into a Box so that it can be dropped
            let node = Box::from_raw(self.tail);
            // set tail as the prev of the current tail
            self.tail = node.prev;
            // if tail is becoming null that means list is empty
            // reset head to null as well
            if self.tail.is_null() {
                self.head = ptr::null_mut();
            }
            // return the value inside node
            Some(node.val)
        }
    }

    /// Returns the reference to the first element from the front
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.peek_front(), None);
    /// list.push_front(1); list.push_front(2); list.push_front(3);
    /// assert_eq!(list.peek_front(), Some(&3));
    /// ```
    pub fn peek_front(&self) -> Option<&T> {
        // if head is null then list is empty, return None
        if self.head.is_null() {
            return None;
        }
        unsafe {
            // return the reference to the value contains in the node
            // the head is pointing to
            Some(&(*self.head).val)
        }
    }

    /// Returns mutable reference to the first element from the front
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.peek_front_mut(), None);
    /// list.push_front(1); list.push_front(2); list.push_front(3);
    /// assert_eq!(list.peek_front_mut(), Some(&mut 3));
    /// ```
    pub fn peek_front_mut(&self) -> Option<&mut T> {
        // if head is null then list is empty, return None
        if self.head.is_null() {
            return None;
        }
        unsafe {
            // return the reference to the value contains in the node
            // the head is pointing to
            Some(&mut (*self.head).val)
        }
    }

    /// Returns the reference to the last element from the back
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.peek_back(), None);
    /// list.push_front(1); list.push_front(2); list.push_front(3);
    /// assert_eq!(list.peek_back(), Some(&1));
    /// ```
    pub fn peek_back(&self) -> Option<&T> {
        // if tail is null then list is empty, return None
        if self.tail.is_null() {
            return None;
        }
        unsafe {
            // return the reference to the value contains in the node
            // the head is pointing to
            Some(&(*self.tail).val)
        }
    }

    /// Returns the reference to the last element from the back
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.peek_back_mut(), None);
    /// list.push_front(1); list.push_front(2); list.push_front(3);
    /// assert_eq!(list.peek_back_mut(), Some(&mut 1));
    /// ```
    pub fn peek_back_mut(&self) -> Option<&mut T> {
        // if tail is null then list is empty, return None
        if self.tail.is_null() {
            return None;
        }
        unsafe {
            // return the reference to the value contains in the node
            // the head is pointing to
            Some(&mut (*self.tail).val)
        }
    }

    /// Moves all elements from `other` to the end of the list.
    /// This reuses all the nodes from other and moves them into self.
    /// After this operation, other becomes empty.
    /// This operation should compute in O(1) time and O(1) memory.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list1 = LinkedList::new();
    /// list1.push_back('a'); list1.push_back('b'); list1.push_back('c');
    /// let mut list2 = LinkedList::new();
    /// list2.push_back('d');
    /// assert_eq!(list1.len(), 3);
    /// assert_eq!(list2.len(), 1);
    /// list1.append(&mut list2);
    /// assert_eq!(list1.len(), 4);
    /// assert_eq!(list2.len(), 0);
    /// assert_eq!(list2.is_empty(), true);
    /// assert_eq!(list1.peek_back(), Some(&'d'));
    /// assert_eq!(list1.peek_front(), Some(&'a'));
    /// ```
    pub fn append(&mut self, other: &mut Self) {
        // if others is empty nothing to be done
        if other.is_empty() {
            return;
        }
        unsafe {
            // if self is not empty then next of current tail
            // will point to other head
            if !self.tail.is_null() {
                (*self.tail).next = other.head;
            }
        }
        // set tail as the other tail
        self.tail = other.tail;
        // if head is null then set head as the other head
        if self.head.is_null() {
            self.head = other.head;
        }
        // clear head and tail in other list
        // so that it becomes empty
        other.head = ptr::null_mut();
        other.tail = ptr::null_mut();
    }

    /// Insert a node at a given index.
    /// Note: Final index at the list will wrap around when length of the list is lesser.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::from([1, 2, 3, 4]);
    /// list.insert_at(5, 4);
    /// assert_eq!(list.len(), 5);
    /// assert_eq!(list.peek_back(), Some(&5));
    /// ```
    pub fn insert_at(&mut self, elem: T, index: usize) {
        // if list is empty then just push the element to the list
        if self.is_empty() || index == 0 {
            return self.push_front(elem);
        }
        let mut cursor = self.cursor_front_mut().unwrap();
        cursor.step_by(index - 1);
        cursor.insert(elem);
    }

    /// Remove a node at a given index.
    /// Note: Final index at the list will wrap around when length of the list is lesser.
    /// If the list is empty then it throws error
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::from([1, 2, 3, 4]);
    /// list.remove_at(2);
    /// assert_eq!(list.len(), 3);
    /// assert_eq!(list.peek_back(), Some(&4));
    /// ```
    pub fn remove_at(&mut self, index: usize) -> Result<T, RemoveUnderCursorError> {
        // if list is empty then throw error
        if self.is_empty() {
            return Err(RemoveUnderCursorError);
        }
        let len = self.len();
        let index = index % len;
        // if first element to be removed
        if index == 0 {
            return self.pop_front().ok_or(RemoveUnderCursorError);
        }
        // if last element to be removed
        if index == len - 1 {
            return self.pop_back().ok_or(RemoveUnderCursorError);
        }
        let mut cursor = self.cursor_front_mut().unwrap();
        cursor.step_by(index - 1);
        cursor.remove()
    }

    /// Splits the list at a given index. Returns a new list.
    /// Note: Final index at the list will wrap around when length of the list is lesser.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::from([1, 2, 3, 4]);
    /// let new_list = list.split_at(2);
    /// assert_eq!(list.len(), 3);
    /// assert_eq!(list.peek_back(), Some(&3));
    /// assert_eq!(new_list.len(), 1);
    /// assert_eq!(new_list.peek_back(), Some(&4));
    /// ```
    pub fn split_at(&mut self, index: usize) -> Self {
        if self.is_empty() {
            return Self::new();
        }
        let mut cursor = self.cursor_front_mut().unwrap();
        cursor.step_by(index);
        cursor.split()
    }

    /// Splice the list at a given index
    /// Note: Final index at the list will wrap around when length of the list is lesser.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::from([1, 2, 3, 4]);
    /// list.splice_at(LinkedList::from([10, 11]), 2);
    /// assert_eq!(list.len(), 6);
    /// assert_eq!(list.peek_back(), Some(&4));
    /// ```
    pub fn splice_at(&mut self, mut other: Self, index: usize) {
        if self.is_empty() {
            self.head = other.head;
            self.tail = other.tail;
            other.head = ptr::null_mut();
            other.tail = ptr::null_mut();
            return;
        }
        let mut cursor = self.cursor_front_mut().unwrap();
        cursor.step_by(index);
        cursor.splice(other);
    }
}
