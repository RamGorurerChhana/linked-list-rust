use crate::LinkedList;
use crate::Node;
use std::marker::PhantomData;
use std::ptr;

impl<T> Node<T> {
    // creates a new instance of Node
    // prev and next pointers are initialized with null values
    fn new(val: T) -> Self {
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
}
