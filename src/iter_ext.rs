pub struct Batching<I: Iterator, T, F: FnMut(&mut I) -> Option<T>> {
    iter: I,
    f: F,
}

pub trait IteratorExt: Iterator {
    fn batching<F, T>(self, f: F) -> Batching<Self, T, F>
    where
        Self: Sized,
        F: FnMut(&mut Self) -> Option<T>,
    {
        Batching { iter: self, f }
    }
}

impl<I: Iterator, T, F: FnMut(&mut I) -> Option<T>> Iterator for Batching<I, T, F> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        (self.f)(&mut self.iter)
    }
}

impl<I: Iterator> IteratorExt for I {}
