use std::{
    fmt,
    ops::{Index, IndexMut},
};

/// A square matrix with a flat vector representation.
#[derive(Clone, PartialEq, Debug)]
pub struct SqMat<T> {
    inner: Vec<T>,
    dim: usize,
}

impl<T: Copy + Default> SqMat<T> {
    /// Create a new square matrix of dimension `dim` filled
    /// by `T::default()`.
    #[inline]
    pub fn new(dim: usize) -> Self {
        Self {
            inner: vec![T::default(); dim * dim],
            dim,
        }
    }

    /// Create a square matrix of dimension `dim` from a vector.
    ///
    /// # Panics
    ///
    /// If `v.len() != dim * dim`.
    #[inline]
    pub fn from_vec(dim: usize, v: &[T]) -> Self {
        assert_eq!(v.len(), dim * dim);
        Self {
            inner: v.into(),
            dim,
        }
    }

    /// Returns the square matrix dimension.
    #[inline]
    pub const fn dim(&self) -> usize {
        self.dim
    }

    /// Fill the matrix entries with `f`.
    pub fn fill_with(&mut self, mut f: impl FnMut() -> T) {
        for e in self.iter_mut() {
            *e = f();
        }
    }

    /// Iterator over the matrix entries.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }

    /// Mutable iterator over the matrix entries.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.inner.iter_mut()
    }

    /// Iterator over the entries in row `r`.
    pub fn row(&self, r: usize) -> impl Iterator<Item = &T> {
        (0..self.dim).map(move |c| &self[(r, c)])
    }

    /// Iterator over the entries in column `c`.
    pub fn col(&self, c: usize) -> impl Iterator<Item = &T> {
        (0..self.dim).map(move |r| &self[(r, c)])
    }

    /// Iterator over the rows.
    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.inner.chunks(self.dim)
    }

    /// Mutable iterator over the rows.
    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.inner.chunks_mut(self.dim)
    }

    /// Convert a `SqMat<T>` to a `SqMat<S>` by applying `f` to the entries.
    pub fn map<S: Default + Copy>(&self, f: impl Fn(T) -> S) -> SqMat<S> {
        let mut res = SqMat::new(self.dim());
        for i in 0..self.dim() {
            for j in 0..self.dim() {
                res[(i, j)] = f(self[(i, j)]);
            }
        }
        res
    }
}

impl<T> Index<usize> for SqMat<T> {
    type Output = [T];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.dim;
        &self.inner[start..start + self.dim]
    }
}

impl<T> IndexMut<usize> for SqMat<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * self.dim;
        &mut self.inner[start..start + self.dim]
    }
}

impl<T> Index<(usize, usize)> for SqMat<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.inner[index.1 + index.0 * self.dim]
    }
}

impl<T> IndexMut<(usize, usize)> for SqMat<T> {
    #[inline]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.inner[index.1 + index.0 * self.dim]
    }
}

impl<T: Copy + Default + fmt::Display> fmt::Display for SqMat<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.rows() {
            for e in row {
                write!(f, "{} ", e)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
