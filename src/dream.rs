use crate::{matrix::SqMat, perm::Perm};

use std::{iter, ops::Index};

/// An ordinary pipe dream.
#[derive(PartialEq, Debug)]
pub struct Dream {
    tiles: SqMat<Tile>,
}

/// A tile in a `Dream`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Elbow,
    Cross,
}

impl Default for Tile {
    fn default() -> Self {
        Self::Elbow
    }
}

impl Dream {
    pub fn from_vec(dim: usize, v: &[Tile]) -> Self {
        Self {
            tiles: SqMat::from_vec(dim, v),
        }
    }

    pub const fn dim(&self) -> usize {
        self.tiles.dim()
    }

    pub fn tiles(&self) -> SqMat<Tile> {
        self.tiles.clone()
    }

    /// Returns the minimum column of an elbow in row `i`.
    pub fn start(&self, i: usize) -> usize {
        (0..self.dim())
            .filter(|&j| self[(i, j)] == Tile::Elbow)
            .chain(iter::once(self.dim()))
            .min()
            .unwrap()
    }

    /// Computes the `i`-th mitosis operator.
    pub fn mitosis(&self, i: usize) -> Vec<Self> {
        let mut offspring = Vec::new();
        let start = self.start(i);
        for p in (0..self.dim()).filter(|&j| j < start && self[(i + 1, j)] != Tile::Cross) {
            let mut tiles = self.tiles.clone();
            tiles[(i, p)] = Tile::Elbow;
            for j in 0..p {
                if tiles[(i, j)] == Tile::Cross && tiles[(i + 1, j)] == Tile::Elbow {
                    tiles[(i, j)] = Tile::Elbow;
                    tiles[(i + 1, j)] = Tile::Cross;
                }
            }
            offspring.push(Self { tiles });
        }
        offspring
    }

    pub fn long(n: usize) -> Self {
        let mut tiles = SqMat::new(n);
        for i in 0..n {
            for j in 0..n {
                if i + j < n - 1 {
                    tiles[(i, j)] = Tile::Cross;
                }
            }
        }
        Self { tiles }
    }
}

impl Index<(usize, usize)> for Dream {
    type Output = Tile;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.tiles[index]
    }
}

/// The set of reduced pipe dreams on a permutation.
pub struct ReducedDreams {
    perm: Perm,
    dreams: Vec<Dream>,
}

impl ReducedDreams {
    pub fn for_perm(perm: &Perm) -> Self {
        let dreams = reduced_dreams(perm);
        Self {
            perm: perm.clone(),
            dreams,
        }
    }

    pub fn perm(&self) -> &Perm {
        &self.perm
    }

    pub fn iter(&self) -> impl Iterator<Item = &Dream> {
        self.dreams.iter()
    }
}

fn mitosis(dreams: &[Dream], i: usize) -> Vec<Dream> {
    let mut res = Vec::new();
    for dream in dreams {
        res.extend(dream.mitosis(i));
    }
    res
}

fn lex_first_reduced_word(p: &Perm) -> Vec<usize> {
    let c = p.lehmer();
    let mut word = Vec::new();
    for (i, ci) in c.iter().rev().enumerate() {
        word.extend((i - ci..i).rev());
    }
    word
}

fn reduced_dreams(p: &Perm) -> Vec<Dream> {
    let p0p = p * &Perm::long(p.len());
    let mut reduced = lex_first_reduced_word(&p0p);

    if reduced.is_empty() {
        return Vec::new();
    }

    reduced.reverse();

    let mut res = Dream::long(p.len()).mitosis(reduced[0]);
    for &i in &reduced[1..] {
        res = mitosis(&res, i);
    }
    res
}
