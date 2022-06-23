use std::{
    fmt,
    ops::{Index, Mul},
    str::FromStr,
};

use crate::matrix::SqMat;

/// A zero-indexed permutation on the natural numbers.
#[derive(Clone, Debug)]
pub struct Perm {
    // Invariant: `inner` sorted is the range `0..inner.len()`.
    inner: Vec<usize>,
}

impl Perm {
    /// Create a permutation from a vector.
    ///
    /// Returns `None` if input is invalid.
    #[inline]
    pub fn new(v: &[usize]) -> Option<Self> {
        let mut sort = vec![0; v.len()];
        sort.copy_from_slice(v);
        sort.sort_unstable();
        if !sort.iter().cloned().eq(0..v.len()) {
            return None;
        }

        // Safety: validated input above.
        Some(unsafe { Self::new_unchecked(v) })
    }

    /// Create a permutation from a vector.
    ///
    /// # Safety
    ///
    /// Assumes the sorted `v` is the range `0..v.len()`.
    #[inline]
    pub unsafe fn new_unchecked(v: &[usize]) -> Self {
        Self { inner: v.into() }
    }

    /// Returns the length of the permutation.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Compute the Lehmer code of the permutation.
    ///
    /// <https://en.wikipedia.org/wiki/Lehmer_code>.
    pub fn lehmer(&self) -> Vec<usize> {
        let mut res = vec![0; self.len()];
        for i in 0..self.len() {
            for j in i + 1..self.len() {
                if self[j] < self[i] {
                    res[i] += 1;
                }
            }
        }
        res
    }

    /// Create the long permutation of length `n`, i.e.
    /// `n n-1 ... 0`.
    #[inline]
    pub fn long(n: usize) -> Self {
        Self {
            inner: (0..n).rev().collect(),
        }
    }

    /// Compose the permuation with another permutation,
    /// i.e. `self(other(_))`.
    ///
    /// # Panics
    ///
    /// Panics if `self.len() != other.len()`.
    pub fn compose_with(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        let mut inner = vec![0; self.len()];
        for i in 0..self.len() {
            inner[i] = self[other[i]];
        }
        Self { inner }
    }

    /// Iterator over the permutation indices.
    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.inner.iter().copied()
    }
}

impl Perm {
    pub fn matrix(&self) -> SqMat<u8> {
        let mut m = SqMat::new(self.len());
        for i in 0..self.len() {
            m[(i, self[i])] = 1;
        }
        m
    }

    pub fn rothe(&self) -> SqMat<u8> {
        let mut rothe = SqMat::new(self.len());
        for i in 0..self.len() {
            for j in 0..self.len() {
                if i < j && self[i] > self[j] {
                    rothe[(i, self[j])] = 1;
                }
            }
        }
        rothe
    }
}

impl Index<usize> for Perm {
    type Output = usize;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        &self.inner[i]
    }
}

impl<'a> Mul for &'a Perm {
    type Output = Perm;

    fn mul(self, rhs: &'a Perm) -> Self::Output {
        self.compose_with(rhs)
    }
}

impl fmt::Display for Perm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[derive(Debug)]
pub struct PermParseError;

/// Parse a zero-indexed permuation from a string of digits `input`
/// separated by a delimiter `delim`.
///
/// Returns `PermParseError` if string could not be parsed correctly.
pub fn parse_perm(input: &str, delim: &str) -> Result<Perm, PermParseError> {
    fn read<T: FromStr>(s: &str, delim: &str) -> Result<Vec<T>, T::Err> {
        s.trim()
            .split(delim)
            .map(|x| x.trim())
            .map(|x| x.parse())
            .collect()
    }

    match read::<usize>(input, delim) {
        Ok(v) => {
            if let Some(perm) = Perm::new(&v) {
                Ok(perm)
            } else {
                Err(PermParseError)
            }
        }
        Err(_) => Err(PermParseError),
    }
}

impl FromStr for Perm {
    type Err = PermParseError;

    /// Parse using `parse_perm` with delimiter `,`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_perm(s, ",")
    }
}
