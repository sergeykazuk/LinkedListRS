# Single Linked List in Rust

A generic, educational implementation of a singly linked list data structure in Rust. Demonstrates core concepts like ownership, borrowing, safe pointer management with `Box` and `Option` types, and **generic programming with trait bounds** (`Clone`, `PartialEq`, `Display`).

## Why SingleLinkedList?

Learning to implement a single linked list is valuable for several reasons:

1. **Understanding Memory Management** - learn how dynamic memory allocation works and why Rust's ownership system is powerful. Unlike garbage-collected languages, there is direct control of when memory is freed.

2. **Rust's Type System in Action** - This project demonstrates:
   - `Option<T>` for representing "maybe a value" safely (no null pointers)
   - `Box<T>` for heap allocation and owning pointers
   - Pattern matching with `while let` for safe iteration
   - Rust's borrow checker in a non-trivial structure
   - **Generic type parameters `<T>` with trait bounds** (`Clone`, `PartialEq`, `Display`)

3. **Pointer Arithmetic Patterns** - see how to navigate and modify linked structures using references, solving the classical "borrow checker puzzle" that trips up many Rust newcomers.

4. **Foundation for Advanced Structures** - Linked lists are building blocks for queues, stacks, trees, and graphs.

5. **Generic Programming Pain Points** - learn why generic Rust code is more verbose than C++ templates. Discover the design trade-offs: explicitness and safety vs. brevity. Understand trait bounds and when they're necessary.

## Building and Running

### Prerequisites
- Rust 1.70+ (check with `rustc --version`)

For Rust installation instructions, see the [official Rust installation guide](https://www.rust-lang.org/tools/install).

### Build
```bash
cargo build --release
```

### Run
```bash
cargo run
``` typed list: `SingleLinkedList::<i16>::new()`
- Adding values by reference: `ll.add_value(&14)`
- Removing values by reference: `ll.remove_value(&25)`emonstrates:
- Creating an empty list
- Adding values (14, 25, 32, 59)
- Removing values by value
- Printing the list state

## API<T>::new()`
Creates an empty linked list for type `T` (must implement `Clone + PartialEq + Display`).

```rust
let mut ll: SingleLinkedList<i16> = SingleLinkedList::new();
// or
let mut ll = SingleLinkedList::<String>::new();
```

### `add_value(&mut self, val: &T)`
Adds a value to the **end** of the list (appends). Takes a reference and clones it internally.

```rust
ll.add_value(&42);
ll.add_value(&"hello".to_string());
```

### `remove_value(&mut self, val: &T)`
Removes the first occurrence of a value from the list. Prints a message if value not found. Takes a reference.

```rust
ll.remove_value(&42);
ll.remove_value(&"hello".to_string()
```rust
ll.remove_value(42);
```

### `print_all(&self)`
Prints all values in the list to stdout.

```rust
ll.print_all();
```

### `size(&self) -> usize`
Returns the number of elements in the list.

```rGeneric Type**: `SingleLinkedList<T>` where `T: Clone + PartialEq + Display`
- **Why `Clone`?**: Values are passed by reference and cloned when stored. This pattern is safer for large types like `String` and `Vec`.
- **Why `PartialEq`?**: Required for value comparisons in `remove_value()`
- **Why `Display`?**: Required for `print_all()` to print values
let count = ll.size();
```

## Current Implementation Details

**Limited Error Handling** - Uses println for "not found" errors. Could return `Result<T>` or `Option<T>`.
**Remove by Value Only** - No remove-by-index. Could add `remove_at(index: usize) -> Option<T>`.
**Trait Bounds are Restrictive** - Only works with types implementing `Clone + PartialEq + Display`. Could use `Copy` version for primitives (faster, cleaner syntax).
**No `is_empty()` method** - Could add convenience methods like `is_empty()`, `first()`, `last
