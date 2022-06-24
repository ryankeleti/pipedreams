use std::{collections::HashMap, fmt};

use crate::{
    dream::{Dream, ReducedDreams, Tile},
    perm::Perm,
};

/// Monomial in one variable with no coefficients.
#[derive(Clone, Debug)]
pub struct Monomial {
    // `(i, p)` represents x_i^p.
    powers: Vec<(usize, usize)>,
}

impl Monomial {
    pub fn powers(&self) -> &[(usize, usize)] {
        &self.powers
    }
}

impl<'a> From<&'a Dream> for Monomial {
    fn from(dream: &'a Dream) -> Self {
        let mut powers = HashMap::new();
        for i in 0..dream.dim() {
            for j in 0..dream.dim() {
                if dream[(i, j)] == Tile::Cross {
                    *powers.entry(i).or_insert(0) += 1;
                }
            }
        }
        let mut powers: Vec<_> = powers.into_iter().collect();
        powers.sort_unstable();
        Self { powers }
    }
}

impl fmt::Display for Monomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn elem(i: usize, p: usize) -> String {
            match p {
                0 => "1".into(),
                1 => format!("x_{}", i),
                _ => format!("x_{}^{}", i, p),
            }
        }

        if let Some((&(i, p), rest)) = self.powers().split_first() {
            write!(f, "{}", elem(i, p))?;
            for &(i, p) in rest {
                write!(f, "*{}", elem(i, p))?;
            }
        } else {
            write!(f, "1")?;
        }
        Ok(())
    }
}

/// Schubert polynomial.
#[derive(Clone, Debug)]
pub struct Schubert {
    perm: Perm,
    parts: Vec<Monomial>,
}

impl Schubert {
    pub fn from_dreams(dreams: &ReducedDreams) -> Self {
        let mut parts = Vec::new();
        for dream in dreams.iter() {
            let mono = Monomial::from(dream);
            if !mono.powers().is_empty() {
                parts.push(mono);
            }
        }

        Self {
            perm: dreams.perm().clone(),
            parts,
        }
    }

    pub fn perm(&self) -> &Perm {
        &self.perm
    }

    pub fn parts(&self) -> &[Monomial] {
        &self.parts
    }
}

impl fmt::Display for Schubert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "S_{} = ", self.perm())?;
        if let Some((first, rest)) = self.parts().split_first() {
            write!(f, "{}", first)?;
            for part in rest {
                write!(f, " + {}", part)?;
            }
        } else {
            write!(f, "1")?;
        }
        Ok(())
    }
}
