use self::cursor::Cursor;
use self::cursor_mut::CursorMut;
use crate::LinkedList;

mod cursor;
mod cursor_mut;

impl<T> LinkedList<T> {
    /// Returns a new Cursor initialized at the front of the list
    /// ```
    /// use linked_list::LinkedList;
    /// let list = [1, 2, 3].into_iter().collect::<LinkedList<i32>>();
    /// let cursor = list.cursor_front();
    /// assert_eq!(cursor.is_some(), true);
    /// ```
    pub fn cursor_front(&self) -> Option<Cursor<T>> {
        // if head is null then list is empty, return None
        if self.head.is_null() {
            return None;
        }
        Some(Cursor {
            curr: self.head,
            list: self,
            index: 0,
            length: self.len(),
        })
    }

    /// Returns a new Cursor initialized at the back of the list
    /// ```
    /// use linked_list::LinkedList;
    /// let list = [1, 2, 3].into_iter().collect::<LinkedList<i32>>();
    /// let cursor = list.cursor_back();
    /// assert_eq!(cursor.is_some(), true);
    /// let list: LinkedList<i32> = LinkedList::new();
    /// let cursor = list.cursor_back();
    /// assert_eq!(cursor.is_some(), false);
    /// ```
    pub fn cursor_back(&self) -> Option<Cursor<T>> {
        // if tail is null then list is empty, return None
        if self.tail.is_null() {
            return None;
        }
        Some(Cursor {
            curr: self.tail,
            list: self,
            index: self.len() - 1,
            length: self.len(),
        })
    }

    /// Returns a new Mutable Cursor initialized at the front of the list
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = [1, 2, 3].into_iter().collect::<LinkedList<i32>>();
    /// let cursor = list.cursor_front_mut();
    /// assert_eq!(cursor.is_some(), true);
    /// ```
    pub fn cursor_front_mut(&mut self) -> Option<CursorMut<T>> {
        // if head is null then list is empty, return None
        if self.head.is_null() {
            return None;
        }
        let length = self.len();
        Some(CursorMut {
            curr: self.head,
            list: self,
            index: 0,
            length,
        })
    }

    /// Returns a new Mutable Cursor initialized at the back of the list
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = [1, 2, 3].into_iter().collect::<LinkedList<i32>>();
    /// let cursor = list.cursor_back_mut();
    /// assert_eq!(cursor.is_some(), true);
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// let cursor = list.cursor_back_mut();
    /// assert_eq!(cursor.is_some(), false);
    /// ```
    pub fn cursor_back_mut(&mut self) -> Option<CursorMut<T>> {
        // if tail is null then list is empty, return None
        if self.tail.is_null() {
            return None;
        }
        let length = self.len();
        Some(CursorMut {
            curr: self.tail,
            list: self,
            index: length - 1,
            length,
        })
    }
}
