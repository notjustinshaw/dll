pub mod list_iter;
pub mod list_node;
pub mod sort;

pub use list_iter::ListIter;
pub use list_node::ListNode;
pub use list_node::StrongPointer;
pub use sort::bubble_sort; // TODO: upgrade to reasonable sorting algorithm

use core::fmt;
use core::marker::PhantomData; // for cursors
use std::cell::RefCell;
use std::clone::Clone;
use std::cmp::Ordering;
use std::fmt::Display;
use std::fmt::Formatter;
use std::rc::Rc;

use self::list_iter::ListIterMut;

/// A doubly-linked list from hell >:)
///
/// This `LinkedList` allows pushing and popping elements at either end.
pub struct LinkedList<T: Clone + Display> {
    pub head: Option<StrongPointer<ListNode<T>>>,
    pub tail: Option<StrongPointer<ListNode<T>>>,
    num_elements: usize,
}

impl<T: Clone + Display> LinkedList<T> {
    // Creates an empty `LinkedList`.
    ///
    /// # Example
    ///
    /// ```
    /// # use dll::LinkedList;
    /// let list: LinkedList<u32> = LinkedList::new();
    /// assert_eq!(list.len(), 0);
    /// ```
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            num_elements: 0,
        }
    }

    /// Returns the length of the list.
    ///
    /// # Example
    ///
    /// ```
    /// # use dll::LinkedList;
    /// let mut list: LinkedList<u32> = LinkedList::new();
    /// assert_eq!(list.len(), 0);
    /// list.push_front(1);
    /// list.push_front(2);
    /// list.push_front(3);
    /// assert_eq!(list.len(), 3);
    /// list.pop_front();
    /// list.pop_front();
    /// list.pop_front();
    /// assert_eq!(list.len(), 0);
    /// assert_eq!(list.pop_front(), None);
    /// assert_eq!(list.pop_back(), None);
    /// ```
    pub fn len(&self) -> usize {
        self.num_elements
    }

    /// Adds an element to the head of the list.
    ///
    /// # Example
    ///
    /// ```
    /// # use dll::LinkedList;
    /// let mut list: LinkedList<u32> = LinkedList::new();
    /// list.push_front(1);
    /// assert_eq!(list.len(), 1);
    /// list.push_front(2);
    /// assert_eq!(list.len(), 2);
    /// ```
    pub fn push_front(&mut self, value: T) {
        let new_node: ListNode<T> = ListNode::from(value);
        let new_ref: StrongPointer<ListNode<T>> = Rc::new(RefCell::new(new_node));
        match self.head.take() {
            Some(old_head) => {
                // old head's prev now points to the new node (as a weak ptr)
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_ref));
                new_ref.borrow_mut().next = Some(old_head);
                self.head = Some(new_ref);
            }
            None => {
                // list is empty, so new node is the head and the tail
                self.head = Some(new_ref.clone());
                self.tail = Some(new_ref);
            }
        }
        self.num_elements += 1;
    }

    /// Removes an element from the head of the list and returns it.
    ///
    /// # Example
    ///
    /// ```
    /// # use dll::LinkedList;
    /// let mut list: LinkedList<u32> = LinkedList::new();
    /// list.push_front(1);
    /// list.push_front(2);
    /// assert_eq!(list.len(), 2);
    /// assert_eq!(list.pop_front(), Some(2));
    /// assert_eq!(list.pop_front(), Some(1));
    /// assert_eq!(list.pop_front(), None);
    /// assert_eq!(list.pop_back(), None);
    /// assert_eq!(list.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        self.head
            .take()
            .map(|old_head: StrongPointer<ListNode<T>>| {
                self.num_elements -= 1;
                if self.num_elements == 0 {
                    self.head = None;
                    self.tail = None;
                } else {
                    let mut new_head = old_head.borrow_mut().next.take();
                    new_head.as_mut().unwrap().borrow_mut().prev = None;
                    self.head = new_head;
                }
                old_head.borrow().data.clone()
            })
    }

    /// Adds an element to the tail of the list.
    ///
    /// # Example
    ///
    /// ```
    /// use dll::LinkedList;
    /// let mut list: LinkedList<u32> = LinkedList::new();
    /// assert_eq!(list.len(), 0);
    /// list.push_back(1);
    /// assert_eq!(list.len(), 1);
    /// list.push_back(2);
    /// assert_eq!(list.len(), 2);
    /// ```
    pub fn push_back(&mut self, value: T) {
        let new_node: ListNode<T> = ListNode::from(value);
        let new_ref: StrongPointer<ListNode<T>> = Rc::new(RefCell::new(new_node));
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_ref.clone());
                new_ref.borrow_mut().prev = Some(Rc::downgrade(&old_tail.clone()));
                self.tail = Some(new_ref);
            }
            None => {
                self.head = Some(new_ref.clone());
                self.tail = Some(new_ref);
            }
        }
        self.num_elements += 1;
    }

    /// Removes an element from the tail of the list and returns it.
    ///
    /// # Example
    ///
    /// ```
    /// # use dll::LinkedList;
    /// let mut list: LinkedList<u32> = LinkedList::new();
    /// list.push_front(1);
    /// list.push_front(2);
    /// assert_eq!(list.len(), 2);
    /// assert_eq!(list.pop_back(), Some(1));
    /// assert_eq!(list.pop_back(), Some(2));
    /// assert_eq!(list.pop_back(), None);
    /// assert_eq!(list.pop_front(), None);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail
            .take()
            .map(|old_tail: StrongPointer<ListNode<T>>| {
                self.num_elements -= 1;
                if self.num_elements == 0 {
                    self.head = None;
                    self.tail = None;
                } else {
                    let mut new_tail = old_tail
                        .borrow_mut()
                        .prev
                        .take()
                        .map(|prev| prev.upgrade().unwrap());
                    new_tail.as_mut().unwrap().borrow_mut().next = None;
                    self.tail = new_tail;
                }
                old_tail.borrow().data.clone()
            })
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// # Example
    ///
    /// ```
    /// # use dll::LinkedList;
    ///
    /// let mut list: LinkedList<u32> = LinkedList::new();
    ///
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_back(3);
    ///
    /// assert_eq!(list.len(), 3); // list -> 1 -> 2 -> 3
    ///
    /// list.retain(|x| x % 2 == 0); // list -> 2
    ///
    /// assert_eq!(list.len(), 1);
    /// assert_eq!(list.pop_front(), Some(2));
    /// assert_eq!(list.pop_front(), None);
    /// assert_eq!(list.pop_back(), None);
    /// ```
    pub fn retain(&mut self, mut f: impl FnMut(&T) -> bool) {
        let mut current = self.head.take();
        while let Some(curr) = current {
            let node = curr.borrow();
            if !f(&node.data) {
                // We're removing the front: update head pointer
                if node.prev.is_none() {
                    self.head = node.next.clone();
                }

                // We're removing the tail: update tail pointer
                if node.next.is_none() {
                    self.tail = curr
                        .borrow()
                        .prev
                        .clone()
                        .map(|weak_ptr| weak_ptr.upgrade().unwrap());
                }

                // Remove current node: change prev's next and next's prev
                node.prev.as_ref().map(|prev| {
                    prev.upgrade().map(|prev| {
                        prev.borrow_mut().next = node.next.clone().take();
                    });
                });
                node.next.as_ref().map(|next| {
                    next.borrow_mut().prev = node.prev.clone().take();
                });
                self.num_elements -= 1;
            }
            current = node.next.clone();
        }
    }

    /// Clears the linked list, removing all values.
    ///
    /// # Example
    /// ```
    /// # use dll::LinkedList;
    /// let mut list: LinkedList<u32> = LinkedList::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_back(3);
    /// assert_eq!(list.len(), 3);
    /// list.clear();
    /// assert_eq!(list.len(), 0);
    /// assert_eq!(list.pop_front(), None);
    /// assert_eq!(list.pop_back(), None);
    /// ```
    pub fn clear(&mut self) {
        self.head = None;
        self.tail = None;
        self.num_elements = 0;
    }

    /// Returns `true` if the list contains no elements.
    ///
    /// # Example
    /// ```
    /// # use dll::LinkedList;
    /// let mut list: LinkedList<u32> = LinkedList::new();
    /// assert!(list.is_empty());
    /// list.push_back(1);
    /// assert!(!list.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.num_elements == 0
    }

    /// Returns a reference to an element at the given index or `None` if the
    /// index is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// # use dll::LinkedList;
    /// let mut list: LinkedList<u32> = LinkedList::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_back(3);
    ///
    /// assert_eq!(list.get(0), Some(1));
    /// assert_eq!(list.get(1), Some(2));
    /// assert_eq!(list.get(2), Some(3));
    /// assert_eq!(list.get(3), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<T> {
        self.get_ptr(index).map(|ptr| ptr.borrow().data.clone())
    }

    pub(crate) fn get_ptr(&self, index: usize) -> Option<StrongPointer<ListNode<T>>> {
        if index >= self.num_elements {
            return None;
        }

        let mut current = self.head.clone();
        for _ in 0..index {
            current = current.unwrap().borrow().next.clone();
        }
        Some(current.unwrap())
    }

    /// Returns an iterator over the list.
    ///
    /// # Example
    ///
    /// ```
    /// use dll::LinkedList;
    ///
    /// let mut list: LinkedList<u32> = LinkedList::new();
    ///
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_back(3);
    ///
    /// assert_eq!(list.len(), 3);
    ///
    /// let mut iter = list.iter();
    ///
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(3));
    ///
    /// assert_eq!(iter.next(), None);
    /// assert_eq!(list.len(), 3);
    /// ```
    pub fn iter(&self) -> ListIter<T> {
        ListIter {
            current: self.head.as_ref().map(|node| node.clone()),
            marker: PhantomData,
        }
    }

    /// Returns a mutable iterator over the list.
    ///
    /// # Example
    ///
    /// ```
    /// use dll::LinkedList;
    ///
    /// let mut list: LinkedList<u32> = LinkedList::new();
    ///
    /// list.push_back(0);
    /// list.push_back(1);
    /// list.push_back(2);
    ///
    /// for element in list.iter_mut() {
    ///     *element += 10;
    /// }
    ///
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(10));
    /// assert_eq!(iter.next(), Some(11));
    /// assert_eq!(iter.next(), Some(12));
    /// ```
    pub fn iter_mut(&self) -> ListIterMut<'_, T> {
        ListIterMut {
            current: self.head.as_ref().map(|node| node.clone()),
            marker: PhantomData,
        }
    }

    /// Sorts the list with a comparator function.
    ///
    /// The comparator function must define a total ordering for the elements in
    /// the list. If the ordering is not total, the order of the elements is
    /// unspecified. An order is a total order if (for all `a`, `b` and `c`) it
    /// is:
    ///
    /// * total and antisymmetric: either `a < b`, `a == b` or `a > b` is true
    /// * transitive, `a < b` and `b < c` implies `a < c` and so on.
    ///
    /// For example, while [`f64`] doesn't implement [`Ord`] because
    /// `NaN != NaN`, we can use `partial_cmp` as our sort function when we know
    /// the slice doesn't contain a `NaN`.
    ///
    /// ```
    /// use dll::LinkedList;
    /// let mut list: LinkedList<f64> = LinkedList::new();
    /// list.push_back(1.0);
    /// list.push_back(3.0);
    /// list.push_back(2.0);
    /// list.sort(|a, b| a.partial_cmp(b).unwrap());
    /// assert_eq!(list.to_string(), "1 -> 2 -> 3");
    /// ```
    ///
    /// # Examples
    /// ```
    /// use dll::LinkedList;
    ///
    /// let mut list: LinkedList<u32> = LinkedList::new();
    /// list.push_back(1);
    /// list.push_back(3);
    /// list.push_back(2);
    /// list.sort(|a, b| a.cmp(b));
    /// assert_eq!(list.to_string(), "1 -> 2 -> 3");
    ///
    /// // Sorts the list in decreasing order
    /// list.sort(|a, b| b.cmp(a));
    /// assert_eq!(list.to_string(), "3 -> 2 -> 1");
    /// ```
    pub fn sort(&mut self, compare: impl FnMut(&T, &T) -> Ordering) {
        sort::bubble_sort(self, compare);
    }
}

impl<T: Clone + Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut iter = self.iter();
        if let Some(first) = iter.next() {
            write!(f, "{}", first)?;
            while let Some(next) = iter.next() {
                write!(f, " -> {}", next)?;
            }
        }
        Ok(())
    }
}
