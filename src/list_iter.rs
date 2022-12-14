use core::marker::PhantomData;
use std::fmt::Display;

use super::{ListNode, StrongPointer}; // for cursors

/// An immutable iterator over the elements of a `LinkedList`.
pub struct ListIter<T: Clone + Display> {
    pub current: Option<StrongPointer<ListNode<T>>>,
    pub marker: PhantomData<ListNode<T>>,
}

/// Returns an iterator over the elements of the list.
impl<T: Clone + Display> Iterator for ListIter<T> {
    type Item = T;

    /// Returns the next element of the list.
    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|old: StrongPointer<ListNode<T>>| {
            self.current = old.borrow().next.clone();
            old.borrow().data.clone()
        })
    }
}

/// A mutable iterator over the elements of a `LinkedList`.
pub struct ListIterMut<'a, T: 'a>
where
    T: Clone + Display,
{
    pub current: Option<StrongPointer<ListNode<T>>>,
    pub marker: PhantomData<&'a mut ListNode<T>>,
}

/// Returns an iterator over the elements of the list.
impl<'a, T: Clone + Display> Iterator for ListIterMut<'a, T> {
    type Item = &'a mut T;

    /// Returns the next element of the list.
    fn next(&mut self) -> Option<&'a mut T> {
        self.current
            .take()
            .map(|old: StrongPointer<ListNode<T>>| unsafe {
                // need this to get 'a
                let node = &mut *old.as_ptr();
				self.current = node.next.clone();
                &mut node.data
            })
    }
}
