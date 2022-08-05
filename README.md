# Doubly Linked List

An implementation of a doubly-linked list from hell.

## Example

```rust
extern crate dll;

use dll::LinkedList;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();

    list.push_back(0);
    list.push_back(1);
    list.push_back(2);

    for element in list.iter_mut() {
        *element += 10;
    }

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(10));
    assert_eq!(iter.next(), Some(11));
    assert_eq!(iter.next(), Some(12));
}
```