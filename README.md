# Doubly Linked List
An implementation of a doubly-linked list in Rust.

## Description
The purpose of this library is to explore how you would make a doubly-linked list in Rust, a type-safe low-level programming language. Since a doubly-linked list has both forward and backward links, a typical implementation would use raw pointers. Using raw pointers is considered unsafe and can lead to various issues in large codebases like memory leaks, dangling pointers, use-after-free, etc.

The purpose of this library is to see how you would write this entirely in ["safe Rust"](https://doc.rust-lang.org/nomicon/meet-safe-and-unsafe.html) with ["Strong" (rc) Pointers](https://doc.rust-lang.org/std/rc/struct.Rc.html) going forwards and [Weak Pointers](https://doc.rust-lang.org/std/rc/struct.Weak.html) going backwards. The strong pointers are reference-counted and will free the memory when the ref count drops to zero. On the other hand, weak pointers do not have ownership of their referent and do not count towards the total reference count of the memory (though they can be "[promoted](https://doc.rust-lang.org/std/rc/struct.Weak.html#method.upgrade)" to a normal strong pointer if needed, like in a backwards traversal).

## Quick Start
The `dll` crate exposes a generic `LinkedList<E>` interface that mimics the singly-linked list in the [standard library](https://doc.rust-lang.org/src/alloc/collections/linked_list.rs.html). You can use the dll crate like such:

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

## Features
This was mainly an educational endevor to help me learn Rust ownership patterns. I published this code so that others could use it as a reference as they try to learn Rust. In production, you really shouldn't use this crate. Linked lists are almost never the right choice since they tend to have poor data locality and cache consistency. For this reason, it is almost always faster to use a [vec](https://doc.rust-lang.org/std/vec/struct.Vec.html) instead.

See also: https://rcoh.me/posts/rust-linked-list-basically-impossible/

## License
MIT
