use core::cell::RefCell;
use dll::LinkedList;
use dll::ListNode;
use std::rc::Rc;

#[test]
fn create_empty_list() {
    let list: LinkedList<u32> = LinkedList::new();
    assert_eq!(list.len(), 0);
    assert!(list.head.is_none());
    assert!(list.tail.is_none());
}

#[test]
fn simple_push_pop() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert_eq!(list.len(), 0);
    assert!(list.head.is_none());
    assert!(list.tail.is_none());

    // Insert an element
    list.push_front(1);
    assert_eq!(list.len(), 1);
    assert!(list.head.is_some());
    assert!(list.tail.is_some());
    let head: &Rc<RefCell<ListNode<u32>>> = list.head.as_ref().unwrap();
    let tail: &Rc<RefCell<ListNode<u32>>> = list.tail.as_ref().unwrap();
    assert!(head == tail);
    assert!(head.borrow().prev.is_none());
    assert!(tail.borrow().next.is_none());
    assert_eq!(head.borrow().data, 1);

    // Remove the element
    let one = list.pop_front();
    assert_eq!(one, Some(1));
    assert_eq!(list.len(), 0);
    assert!(list.head.is_none());

    // Fail to remove an element from an empty list
    let pop_front = list.pop_front();
    let pop_back = list.pop_back();
    assert_eq!(pop_front, None);
    assert_eq!(pop_back, None);
}

#[test]
fn double_push_pop() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert_eq!(list.len(), 0);
    assert!(list.head.is_none());
    assert!(list.tail.is_none());

    // Insert two elements
    list.push_front(1);
    assert_eq!(list.len(), 1);
    assert!(list.head.is_some());
    assert!(list.tail.is_some());
    let head: &Rc<RefCell<ListNode<u32>>> = list.head.as_ref().unwrap();
    let tail: &Rc<RefCell<ListNode<u32>>> = list.tail.as_ref().unwrap();
    assert!(head == tail);
    assert!(head.borrow().prev.is_none());
    assert!(tail.borrow().next.is_none());
    assert_eq!(head.borrow().data, 1);

    list.push_front(2);
    assert_eq!(list.len(), 2);
    let head: &Rc<RefCell<ListNode<u32>>> = list.head.as_ref().unwrap();
    let tail: &Rc<RefCell<ListNode<u32>>> = list.tail.as_ref().unwrap();
    assert!(head != tail);
    assert!(head.borrow().prev.is_none());
    assert!(tail.borrow().next.is_none());
    let tail_prev = tail.borrow().prev.as_ref().unwrap().upgrade();
    assert!(tail_prev.as_ref().unwrap() == head);
    assert!(head.borrow().next.as_ref().unwrap() == tail);
    assert_eq!(head.borrow().data, 2);
    assert_eq!(tail.borrow().data, 1);

    // Pop one element
    let one = list.pop_front();
    assert_eq!(one, Some(2));
    assert_eq!(list.len(), 1);
    let head: &Rc<RefCell<ListNode<u32>>> = list.head.as_ref().unwrap();
    let tail: &Rc<RefCell<ListNode<u32>>> = list.tail.as_ref().unwrap();
    assert!(head == tail);
    assert!(head.borrow().prev.is_none());
    assert!(tail.borrow().next.is_none());
    assert_eq!(head.borrow().data, 1);
}

#[test]
fn push_back_pop_back() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert_eq!(list.len(), 0);
    assert!(list.head.is_none());
    assert!(list.tail.is_none());

    // Insert an element
    list.push_back(1);
    assert_eq!(list.len(), 1);
    assert!(list.head.is_some());
    assert!(list.tail.is_some());
    let head: &Rc<RefCell<ListNode<u32>>> = list.head.as_ref().unwrap();
    let tail: &Rc<RefCell<ListNode<u32>>> = list.tail.as_ref().unwrap();
    assert!(head == tail);
    assert!(head.borrow().prev.is_none());
    assert!(tail.borrow().next.is_none());
    assert_eq!(head.borrow().data, 1);

    // Pop one element
    let one = list.pop_back();
    assert_eq!(one, Some(1));
    assert_eq!(list.len(), 0);
    assert!(list.head.is_none());
    assert!(list.tail.is_none());

    // Fail to remove an element from an empty list
    let pop_front = list.pop_front();
    let pop_back = list.pop_back();
    assert_eq!(pop_front, None);
    assert_eq!(pop_back, None);

    // Insert two elements
    list.push_back(1);
    assert_eq!(list.len(), 1);
    assert!(list.head.is_some());
    assert!(list.tail.is_some());
    let head: &Rc<RefCell<ListNode<u32>>> = list.head.as_ref().unwrap();
    let tail: &Rc<RefCell<ListNode<u32>>> = list.tail.as_ref().unwrap();
    assert!(head == tail);
    assert!(head.borrow().prev.is_none());
    assert!(tail.borrow().next.is_none());
    assert_eq!(head.borrow().data, 1);

    list.push_back(2);
    assert_eq!(list.len(), 2);
    let head: &Rc<RefCell<ListNode<u32>>> = list.head.as_ref().unwrap();
    let tail: &Rc<RefCell<ListNode<u32>>> = list.tail.as_ref().unwrap();
    assert!(head != tail);
    assert!(head.borrow().prev.is_none());
    assert!(tail.borrow().next.is_none());
    let tail_prev = tail.borrow().prev.as_ref().unwrap().upgrade();
    assert!(tail_prev.as_ref().unwrap() == head);
    assert!(head.borrow().next.as_ref().unwrap() == tail);
    assert_eq!(head.borrow().data, 1);
    assert_eq!(tail.borrow().data, 2);

    // Pop one element
    let one = list.pop_back();
    assert_eq!(one, Some(2));
    assert_eq!(list.len(), 1);
    let head: &Rc<RefCell<ListNode<u32>>> = list.head.as_ref().unwrap();
    let tail: &Rc<RefCell<ListNode<u32>>> = list.tail.as_ref().unwrap();
    assert!(head == tail);
    assert!(head.borrow().prev.is_none());
    assert!(tail.borrow().next.is_none());
    assert_eq!(head.borrow().data, 1);
}

#[test]
fn sort_and_verify() {
    let mut list: LinkedList<u32> = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    assert_eq!(list.len(), 3);
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.len(), 0);
    assert_eq!(list.pop_front(), None);
}

#[test]
fn push_and_pop_front() {
    let mut list: LinkedList<u32> = LinkedList::new();
    list.push_front(3);
    list.push_front(1);
    list.push_front(2);
    assert_eq!(list.len(), 3);
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.len(), 0);
    assert_eq!(list.pop_front(), None);
}

#[test]
fn push_and_pop_back() {
    let mut list: LinkedList<u32> = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    assert_eq!(list.len(), 3);
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.len(), 0);
    assert_eq!(list.pop_back(), None);
}

#[test]
fn push_and_pop_front_and_back() {
    let mut list: LinkedList<u32> = LinkedList::new();
    list.push_front(1);
    list.push_back(2);
    list.push_front(3);
    list.push_back(4);
    assert_eq!(list.len(), 4);
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_back(), Some(4));
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.len(), 0);
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.pop_back(), None);
}

#[test]
fn iterable() {
    let mut list: LinkedList<u32> = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    list.push_front(4);

    let mut iter = list.iter();

    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
}

#[test]
fn clear() {
    let mut list: LinkedList<u32> = LinkedList::new();
    for i in 0..1000 {
        list.push_back(i);
    }
    list.clear();
    assert_eq!(list.len(), 0);
}

#[test]
fn retains_odd_values() {
    let mut list: LinkedList<u64> = LinkedList::new();
    for i in 0..1000 {
        list.push_back(i);
    }
    list.retain(|x| x % 2 != 0);
    for _ in 0..list.len() {
        assert!((list.pop_front().unwrap() % 2) != 0);
    }
    assert_eq!(list.len(), 0);
}

#[test]
fn immutable_iterator() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert_eq!(list.len(), 0);

    list.push_back(1);
    assert_eq!(format!("{}", list), "1");
    assert_eq!(list.len(), 1);

    list.push_back(2);
    assert_eq!(format!("{}", list), "1 -> 2");
    assert_eq!(list.len(), 2);

    list.push_back(3);
    assert_eq!(format!("{}", list), "1 -> 2 -> 3");
    assert_eq!(list.len(), 3);

    list.push_back(4);
    assert_eq!(format!("{}", list), "1 -> 2 -> 3 -> 4");
    assert_eq!(list.len(), 4);

    list.push_back(5);
    assert_eq!(format!("{}", list), "1 -> 2 -> 3 -> 4 -> 5");
    assert_eq!(list.len(), 5);
}
