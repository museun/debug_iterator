pub trait DebugIterator: Iterator {
    fn debug<'a>(self) -> DebugPrinter<'a, Self>
    where
        Self: Sized,
        Self::Item: std::fmt::Debug,
    {
        DebugPrinter::new(self, false, None)
    }

    fn debug_pretty<'a>(self) -> DebugPrinter<'a, Self>
    where
        Self: Sized,
        Self::Item: std::fmt::Debug,
    {
        DebugPrinter::new(self, true, None)
    }

    fn debug_prefix<'a, S>(self, prefix: S) -> DebugPrinter<'a, Self>
    where
        Self: Sized + 'a,
        Self::Item: std::fmt::Debug,
        S: Into<Cow<'a, str>>,
    {
        DebugPrinter::new(self, false, Some(prefix.into()))
    }

    fn debug_prefix_pretty<'a, S>(self, prefix: S) -> DebugPrinter<'a, Self>
    where
        Self: Sized + 'a,
        Self::Item: std::fmt::Debug,
        S: Into<Cow<'a, str>>,
    {
        DebugPrinter::new(self, true, Some(prefix.into()))
    }
}

use std::borrow::Cow;
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
                log::debug!("{}",format_args!($e, $($xs),*));

                #[cfg(not(feature = "logging"))]
                eprintln!("{}",format_args!($e, $($xs),*));
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
