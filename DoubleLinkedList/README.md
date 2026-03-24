# DoubleLinkedList — Rust Learning Project

## Project Goals

This project is a hands-on exercise to understand Rust’s ownership, borrowing, and smart pointer models, especially:

- How `Rc` (reference counting) enables shared ownership of nodes in a data structure.
- How `RefCell` provides interior mutability, allowing mutation of data even when it is shared.
- How `Weak` pointers prevent reference cycles in doubly linked lists.
- How to safely traverse and mutate a linked list using these types.
- How to work with `Option` and avoid panics by not overusing `unwrap()`.


## Accessing Elements: The `at` Function

The `at` function provides access to the value at a given index in the list:

```rust
pub fn at(&self, index: usize) -> Option<T>
```

- Returns `Some(T)` if the index is valid, or `None` if out of bounds.
- **Note:** This function returns a clone of the value at the given index. This is necessary because the list uses `Rc<RefCell<Node<T>>>` for shared, mutable nodes, and Rust's borrowing rules do not allow returning references to data inside a `RefCell`.
- If cloning is expensive, consider a different data structure (e.g., `Vec` or arena allocation).

## What’s Implemented

- A doubly linked list using `Rc<RefCell<Node<T>>>` for shared, mutable nodes.
- O(1) insertion at the end of the list using a `Weak` pointer to the tail.
- Forward and backward traversal with safe, idiomatic Rust patterns.
- Extensive comments and experiments with `as_ref()`, `map(Rc::clone)`, `and_then`, and `unwrap()` to understand how to move between references, owned pointers, and options.
- Removing nodes (especially in the middle) is a more advanced exercise, as it requires careful handling of `Rc` counts and pointer updates.

## Lessons Learned

- **Rc and RefCell**: You must use `Rc::clone` to share ownership, and `borrow()`/`borrow_mut()` to access or mutate the node inside a `RefCell`.
- **Weak**: Use `Rc::downgrade` to create a `Weak` pointer for back-links, and `upgrade()` to temporarily get an `Rc` for traversal.
- **Option and Unwrap**: Many fields are `Option` because nodes may be missing (start, end, next, prev). Using `unwrap()` can panic if you hit `None`, so prefer safe patterns like `map`, `and_then`, and `while let Some(...)`.
- **Borrow Checker**: Rust’s borrow checker enforces safe access, so you must be careful not to hold borrows across mutations or assignments.

## Next Steps

- Add more tests and edge case handling.
- Continue experimenting with Rust’s ownership and borrowing patterns.

## Why This Matters

Understanding these patterns is essential for writing safe, efficient, and idiomatic Rust code—especially for data structures and systems programming. This project is a stepping stone to mastering Rust’s unique approach to memory safety.
