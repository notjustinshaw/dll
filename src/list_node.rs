/// A doubly-linked generic node.
///
/// In a doubly-linked list, each node stores a reference to the next and prev
/// nodes in the list. A client should be able to traverse the list in either
/// direction and mutate any element in the list.
///
/// To prevent cycles, the `next` field will store a reference-counted pointer
/// to the next node, but the `prev` will only store a weak pointer to the prev
/// node (ie. a non-reference-counted pointer). This means that traversing the
/// list backwards will require promoting each weak pointer to a reference-
/// counted pointer before traversing the list.
use std::cell::RefCell;
use std::clone::Clone;
use std::fmt::Display;
use std::rc::{Rc, Weak};

/// The node type used to store data.
///
/// A `ListNode` is a wrapper around the clonable type `T` that holds a
/// reference-counted pointer to the previous/next nodes in the list. The next
/// pointer is a strong reference and the prev pointer is a weak reference.
///
/// StrongPointer - a single-threaded reference-counted pointer to a node.
/// WeakPointer - a version of rc that holds a non-owning reference to the node.
pub struct ListNode<T: Clone + Display> {
    pub data: T,
    pub next: Option<StrongPointer<ListNode<T>>>,
    pub prev: Option<WeakPointer<ListNode<T>>>,
}

pub type StrongPointer<T> = Rc<RefCell<T>>;
pub type WeakPointer<T> = Weak<RefCell<T>>;

/////////////////////////////////////////////////////////////////////////
// List Node - Core Implementation
/////////////////////////////////////////////////////////////////////////

impl<T: Clone + Display> ListNode<T> {
    /// Creates a new node with the given value, next, and previous nodes.
    ///
    /// # Example
    ///
    /// ```
    /// # use dll::ListNode;
    /// # use std::cell::RefCell;
    /// # use std::rc::{Rc, Weak};
    ///
    /// let three = Rc::new(RefCell::new(ListNode::from(3)));
    /// let two: ListNode<u32> = ListNode::new(2, Some(three), None);
    ///
    /// assert_eq!(two.data, 2);
    /// assert!(two.next.is_some());
    /// assert!(two.prev.is_none());
    /// ```
    pub fn new(
        data: T,
        next: Option<StrongPointer<ListNode<T>>>,
        prev: Option<WeakPointer<ListNode<T>>>,
    ) -> Self {
        Self { data, next, prev }
    }
}

/////////////////////////////////////////////////////////////////////////
// List Node - From Implementation
/////////////////////////////////////////////////////////////////////////

/// Constructs a new ListNode with the given value.
///
/// The next and previous nodes are set to `None` and the data is cloned into
/// the node.
impl<T: Clone + Display> From<T> for ListNode<T> {
    /// Creates a new node with the given value and no next or previous nodes.
    ///
    /// Takes ownership of the passed in value of type T.
    ///
    /// # Example
    /// ```
    /// use dll::ListNode;
    ///
    /// let node: ListNode<u32> = ListNode::from(42);
    /// assert_eq!(node.data, 42);
    /// assert!(node.next.is_none());
    /// assert!(node.prev.is_none());
    /// ```
    fn from(data: T) -> Self {
        Self::new(data, None, None)
    }
}

/////////////////////////////////////////////////////////////////////////
// List Node - PartialEq Implementation
/////////////////////////////////////////////////////////////////////////

impl<T> PartialEq<ListNode<T>> for ListNode<T>
where
    T: Clone + Display + PartialEq,
{
    fn eq(&self, other: &ListNode<T>) -> bool {
        self.data == other.data
    }
}
