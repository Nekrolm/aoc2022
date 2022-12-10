pub fn get_input_file() -> std::fs::File {
    let file = std::env::args().nth(1).expect("expected one argument");
    std::fs::File::open(file).expect("Cannot open file with aoc input")
}

#[derive(Debug, Clone)]
pub struct Array2D<T> {
    data: Vec<T>,
    cols: usize,
    rows: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct ShapeMismatchError;

impl std::fmt::Display for ShapeMismatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mismathched array shape")
    }
}

impl std::error::Error for ShapeMismatchError {}

impl<T> Array2D<T> {
    pub fn from_iter(
        iter: impl IntoIterator<Item = T>,
        (rows, cols): (usize, usize),
    ) -> Result<Self, ShapeMismatchError> {
        let data = Vec::from_iter(iter);
        if data.len() != rows * cols {
            Err(ShapeMismatchError)
        } else {
            Ok(Array2D { data, rows, cols })
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn row(&self, x: usize) -> &[T] {
        assert!(x <= self.rows);
        let begin = x * self.cols;
        let end = begin + self.cols;
        &self.data[begin..end]
    }

    pub fn col(&self, y: usize) -> impl DoubleEndedIterator<Item = &T> + ExactSizeIterator + Clone {
        assert!(y < self.cols);
        self.data.iter().skip(y).step_by(self.cols)
    }

    pub fn cols(
        &self,
    ) -> impl DoubleEndedIterator<
        Item = impl DoubleEndedIterator<Item = &T> + ExactSizeIterator + Clone,
    > + ExactSizeIterator
           + Clone {
        (0..self.cols).map(move |idx| self.col(idx))
    }

    pub fn rows(&self) -> impl DoubleEndedIterator<Item = &[T]> + '_ {
        self.data.chunks(self.cols)
    }
}

impl<T> std::ops::Index<(usize, usize)> for Array2D<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        assert!(x < self.rows);
        assert!(y < self.cols);
        &self.data[x * self.cols + y]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Array2D<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        assert!(x < self.rows);
        assert!(y < self.cols);
        &mut self.data[x * self.cols + y]
    }
}
