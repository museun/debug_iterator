# debug_iterator

## debug_iterator
This is a simple iterator adapter thats is applicable to iterators where the Iterator::Item is std::fmt::Debug

It prints to `stderr` by default, but using the feature 'logging' prints out to the `log` crate facade.

```rust
use debug_iterator::DebugIterator as _;

#[derive(Debug)]
struct Person {
    name: String,
    age: i32
}

let everyone_is_named_bob = "Bob".to_string();
let iter = (1..=3)
    .map(|k| k * 4)
    .map(|age| Person {
        name: everyone_is_named_bob.clone(),
        age,
    })
    .clone();

// debug ("{:?}")
iter.debug().for_each(|_| ());
// Person { name: "Bob", age: 4 }
// Person { name: "Bob", age: 8 }
// Person { name: "Bob", age: 12 }

// debug_pretty ("{:#?}")
iter.debug_pretty().for_each(|_| ());
// Person {
//     name: "Bob",
//     age: 4,
// }
// Person {
//     name: "Bob",
//     age: 8,
// }
// Person {
//     name: "Bob",
//     age: 12,
// }

// '{:?}' with a `&str` prefix:
iter.debug_prefix("This person is").for_each(|_| ());
// This person is: Person { name: "Bob", age: 4 }
// This person is: Person { name: "Bob", age: 8 }
// This person is: Person { name: "Bob", age: 12 }

// '{:#?}' with a `&str` prefix:
iter.debug_prefix_pretty("This person is").for_each(|_| ());
// This person is: Person {
//     name: "Bob",
//     age: 4,
// }
// This person is: Person {
//     name: "Bob",
//     age: 8,
// }
// This person is: Person {
//     name: "Bob",
//     age: 12,
// }

```

License: 0BSD
