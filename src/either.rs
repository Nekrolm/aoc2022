

#[derive(Debug, Clone, Copy)]
pub enum Either<L, R> {
    Left(L),
    Right(R)
}

impl<L, R, T> Iterator for Either<L, R> where
L: Iterator<Item = T>, R : Iterator<Item = T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Either::Left(inner) => inner.next(),
            Either::Right(inner) => inner.next()
        }
    }
}

