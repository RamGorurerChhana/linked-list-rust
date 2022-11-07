use crate::combinatorics::{IntoIter, Iter, IterMut};
use crate::LinkedList;
use std::cmp::Ordering;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};
use std::marker::{Send, Sync};

// Implement Send trait for the LinkedList
// This marker trait indicates that the type
// is safe to send to another thread.
unsafe impl<T: Send> Send for LinkedList<T> {}

// Implement Sync trait for the LinkedList
// This marker trait indicates that the type
// is safe to share accross threads.
unsafe impl<T: Sync> Sync for LinkedList<T> {}

// Implement Clone trait for LinkedList
// This will provide the ability to create a duplicate list from a given list.
impl<T: Clone> Clone for LinkedList<T> {
    /// Returns a new duplicate list with all nodes cloned into the new list.
    /// The original list is left as is.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_front(1); list.push_front(2); list.push_front(3);
    /// let new_list = list.clone();
    /// assert_eq!(new_list.len(), list.len());
    /// assert!(new_list.iter().eq(list.iter()));
    /// ```
    fn clone(&self) -> Self {
        let mut new_list = Self::new();
        for elem in self.iter() {
            new_list.push_back(elem.clone());
        }
        new_list
    }
}

// Implement Debug trait for LinkedList
// This will provide the ability to print the list with Debug marker
impl<T: Debug> Debug for LinkedList<T> {
    /// Allows the list to be printed with debug marker
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1); list.push_back(2); list.push_back(3);
    /// println!("{:?}", list);
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut debug_list = f.debug_list();
        let mut head = self.head;
        // walk over the entire list and debug print each node.
        while !head.is_null() {
            unsafe {
                debug_list.entry(&head);
                debug_list.entry(&*head);
                head = (*head).next;
            }
        }
        debug_list.finish()
    }
}

// Implement Display trait for LinkedList.
// This will provide the ability to convert list into String.
impl<T: Display> Display for LinkedList<T> {
    /// Allows the list to be converted to String and printed
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1); list.push_back(2); list.push_back(3);
    /// assert_eq!(list.to_string(), "HEAD -> 1 <-> 2 <-> 3 -> END".to_string());
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "HEAD")?;
        for (i, elem) in self.iter().enumerate() {
            let arrow = if i == 0 { "->" } else { "<->" };
            write!(f, " {} {}", arrow, elem)?;
        }
        write!(f, " -> END")?;
        Ok(())
    }
}

// Implement Default trait for LinkedList

impl<T> Default for LinkedList<T> {
    /// Creates a default empty list
    /// ```
    /// use linked_list::LinkedList;
    /// let list: LinkedList<u32> = LinkedList::default();
    /// assert_eq!(list.len(), 0);
    /// assert_eq!(list.is_empty(), true);
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

// Implement Drop trait for the list
// so that all allocated memory for all nodes will be cleaned up
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // pop off all nodes from the list until list is empty
        while self.pop_front().is_some() {}
    }
}

// Implement `IntoIterator` for type `LinkedList<T>`.
// It will yields owned value T when `next` is called on the iterator.
impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    /// Returns IntoIter.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1); list.push_back(2); list.push_back(3);
    /// let mut iter = list.into_iter();
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

// Implement `IntoIterator` for type `&LinkedList<T>`.
// It will yields &T when `next` is called on the iterator.
impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    /// Returns Iter.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1); list.push_back(2); list.push_back(3);
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// Implement `IntoIterator` for type `&mut LinkedList<T>`.
// It will yields &mut T when `next` is called on the iterator.
impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    /// Returns IterMut.
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1); list.push_back(2); list.push_back(3);
    /// let mut iter = list.iter_mut();
    /// assert_eq!(iter.next(), Some(&mut 1));
    /// assert_eq!(iter.next(), Some(&mut 2));
    /// assert_eq!(iter.next(), Some(&mut 3));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

// Implement PartialEq for LinkedList
impl<T: PartialEq> PartialEq for LinkedList<T> {
    /// Allow to compare equality of two lists
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1); list.push_back(2); list.push_back(3);
    /// let list_dup = list.clone();
    /// assert_eq!(list, list_dup);
    /// let mut other_list = LinkedList::new();
    /// other_list.push_back(1);
    /// assert_ne!(list, other_list);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other.iter())
    }
}

// Implement Eq for LinkedList
impl<T: Eq> Eq for LinkedList<T> {}

// Implement PartialOrd for LinkedList
impl<T: PartialOrd> PartialOrd for LinkedList<T> {
    /// Compare two lists
    /// ```
    /// use std::cmp::Ordering;
    /// use linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.push_back(1); list.push_back(2); list.push_back(3);
    /// let mut other_list = LinkedList::new();
    /// other_list.push_back(4); other_list.push_back(5); other_list.push_back(6);
    /// assert_eq!(list.partial_cmp(&other_list), Some(Ordering::Less));
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other.iter())
    }
}

// Implement Ord trait for LinkedList
impl<T: Ord> Ord for LinkedList<T> {
    /// Compare two lists
    /// ```
    /// use std::cmp::Ordering;
    /// use linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.push_back(1); list.push_back(2); list.push_back(3);
    /// let mut other_list = LinkedList::new();
    /// other_list.push_back(0); other_list.push_back(5); other_list.push_back(6);
    /// assert_eq!(list.cmp(&other_list), Ordering::Greater);
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other.iter())
    }
}

// Implement Hash trait for LinkedList
impl<T: Hash> Hash for LinkedList<T> {
    /// Generate hash for a LikedList
    /// ```
    /// use std::collections::HashSet;
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1); list.push_back(2); list.push_back(3);
    /// let other_list = list.clone();
    /// let mut set = HashSet::new();
    /// set.insert(list);
    /// assert!(set.contains(&other_list));
    /// ```
    fn hash<H: Hasher>(&self, state: &mut H) {
        // include length of the list in the hash
        self.len().hash(state);
        // include each node element in the hash
        for elem in self.iter() {
            elem.hash(state);
        }
    }
}

// Implement From<[T;N]> trait for LinkedList
impl<T, const N: usize> From<[T; N]> for LinkedList<T> {
    /// Returns a new LinkedList from the given array
    /// ```
    /// use linked_list::LinkedList;
    /// let list = LinkedList::from([1, 2, 3]);
    /// assert_eq!(list.len(), 3);
    /// assert_eq!(list.is_empty(), false);
    /// assert!(list.iter().eq([1, 2, 3].iter()));
    /// ```
    fn from(arr: [T; N]) -> Self {
        arr.into_iter().collect()
    }
}

// Implement FromIterator<T> for LinkedList<T>
impl<T> FromIterator<T> for LinkedList<T> {
    /// Returns a new LinkedList from the given array
    /// ```
    /// use linked_list::LinkedList;
    /// let list: LinkedList<i32> = [1, 2, 3].into_iter().collect();
    /// assert_eq!(list.len(), 3);
    /// assert_eq!(list.is_empty(), false);
    /// assert!(list.iter().eq([1, 2, 3].iter()));
    /// ```
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        // create a new empty list
        let mut new_list = Self::new();
        new_list.extend(iter);
        new_list
    }
}

// Implement Extend<T> for LinkedList<T>
impl<T> Extend<T> for LinkedList<T> {
    /// Returns a new LinkedList from the given array
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.extend([1, 2, 3].into_iter());
    /// assert_eq!(list.len(), 3);
    /// assert_eq!(list.is_empty(), false);
    /// assert!(list.iter().eq([1, 2, 3].iter()));
    /// ```
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        // take each element in the interator and push_back into the list
        for elem in iter.into_iter() {
            self.push_back(elem);
        }
    }
}

// Implement Extend<T> for LinkedList<T>
impl<'a, T: Clone + 'a> Extend<&'a T> for LinkedList<T> {
    /// Returns a new LinkedList from the given array
    /// ```
    /// use linked_list::LinkedList;
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.extend([1, 2, 3].iter());
    /// assert_eq!(list.len(), 3);
    /// assert_eq!(list.is_empty(), false);
    /// assert!(list.iter().eq([1, 2, 3].iter()));
    /// ```
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        // take each element in the interator and push_back into the list
        for elem in iter.into_iter() {
            self.push_back(elem.clone());
        }
    }
}
