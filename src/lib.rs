/*!
# debug_iterator
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
*/
use std::borrow::Cow;

/// [`DebugIterator`](./trait.DebugIterator.html) is an [`std::iter::Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) adapter that simply prints out
/// the debug representation of the [`Iterator::Item`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#associatedtype.Item)
pub trait DebugIterator: Iterator {
    /// Create an adapter that prints out the [`std::fmt::Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) representation of the Item
    fn debug<'a>(self) -> DebugPrinter<'a, Self>
    where
        Self: Sized,
        Self::Item: std::fmt::Debug,
    {
        DebugPrinter::new(self, false, None)
    }

    /// Create an adapter that prints out the [`std::fmt::Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) alterntive representation of the Item
    fn debug_pretty<'a>(self) -> DebugPrinter<'a, Self>
    where
        Self: Sized,
        Self::Item: std::fmt::Debug,
    {
        DebugPrinter::new(self, true, None)
    }

    /// Create an adapter that prints out the [`std::fmt::Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) representation of the Item, with a Prefix
    fn debug_prefix<'a, S>(self, prefix: S) -> DebugPrinter<'a, Self>
    where
        Self: Sized + 'a,
        Self::Item: std::fmt::Debug,
        S: Into<Cow<'a, str>>,
    {
        DebugPrinter::new(self, false, Some(prefix.into()))
    }

    /// Create an adapter that prints out the [`std::fmt::Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) alterntive representation of the Item, with a Prefix
    fn debug_prefix_pretty<'a, S>(self, prefix: S) -> DebugPrinter<'a, Self>
    where
        Self: Sized + 'a,
        Self::Item: std::fmt::Debug,
        S: Into<Cow<'a, str>>,
    {
        DebugPrinter::new(self, true, Some(prefix.into()))
    }
}

/// [`DebugPrinter`](./struct.DebugPrinter.html) is the iterator for debug printing
pub struct DebugPrinter<'a, T>(T, bool, Option<Cow<'a, str>>);

impl<'a, T> DebugPrinter<'a, T>
where
    T: Iterator,
    T::Item: std::fmt::Debug,
{
    fn new(x: T, pretty: bool, msg: Option<Cow<'a, str>>) -> Self {
        Self(x, pretty, msg)
    }
}

impl<'a, T> Iterator for DebugPrinter<'a, T>
where
    T: Iterator,
    T::Item: std::fmt::Debug,
{
    type Item = T::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.0.next()?;

        #[inline]
        macro_rules! _log_this {
            ($e:expr, $($xs:expr),* $(,)?) => {{
                #[cfg(feature = "logging")]
                ::log::debug!("{}", format_args!($e, $($xs),*));

                #[cfg(not(feature = "logging"))]
                eprintln!("{}", format_args!($e, $($xs),*));
            }};
        }

        match (self.1, &self.2) {
            (true, Some(prefix)) => _log_this!("{}: {:#?}", prefix, next),
            (false, Some(prefix)) => _log_this!("{}: {:?}", prefix, next),
            (true, None) => _log_this!("{:#?}", next),
            (false, None) => _log_this!("{:?}", next),
        }
        Some(next)
    }
}

impl<T: ?Sized> DebugIterator for T where T: Iterator {}
