use cute::c;
use num_traits::Unsigned;
use std::{
    mem::MaybeUninit,
    ops::{Deref, Index, IndexMut},
};
#[allow(unused_imports)]
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub trait Cube: Eq + Ord {
    fn rotated(self, face: Face, n: u32) -> Self;
    fn solved() -> Self;
    fn heuristic(&self) -> u8 {
        return 0;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct Facelets {
    faces: [u32; 6],
}

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum Face {
    U,
    D,
    L,
    R,
    F,
    B,
}

impl Deref for Facelets {
    type Target = [u32; 6];

    fn deref(&self) -> &Self::Target {
        return &self.faces;
    }
}
impl Index<Face> for Facelets {
    type Output = u32;

    #[inline]
    fn index(&self, index: Face) -> &Self::Output {
        return &self.faces[index as usize];
    }
}
impl IndexMut<Face> for Facelets {
    #[inline]
    fn index_mut(&mut self, index: Face) -> &mut Self::Output {
        return &mut self.faces[index as usize];
    }
}
impl Index<u8> for Facelets {
    type Output = u32;

    #[inline]
    fn index(&self, index: u8) -> &Self::Output {
        return &self.faces[index as usize];
    }
}
impl IndexMut<u8> for Facelets {
    #[inline]
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        return &mut self.faces[index as usize];
    }
}
impl Default for Facelets {
    fn default() -> Self {
        Self::new()
    }
}

impl Cube for Facelets {
    fn solved() -> Self {
        Facelets::new()
    }

    fn rotated(mut self, face: Face, n: u32) -> Self {
        self[face] = self[face].rotate_left(8 * n);
        let faces = Self::connections(face);
        let mut triples: [MaybeUninit<u32>; 4] = [const { MaybeUninit::uninit() }; 4];
        for i in 0..4 {
            let (face, offset) = faces[i];
            triples[i].write(self[face].rotate_right(8 * offset) & 0xfff);
        }
        let triples: [u32; 4] = unsafe { std::mem::transmute(triples) };

        for i in 0..4 {
            let (face, offset) = faces[i];
            let mask = 0xfffu32.rotate_left(8 * offset);
            self[face] =
                self[face] & !mask | triples[(i + 4 - n as usize) % 4].rotate_left(8 * offset)
        }

        self
    }
}

impl Facelets {
    pub const fn new() -> Self {
        let faces = [
            Self::uniform_face(0),
            Self::uniform_face(1),
            Self::uniform_face(2),
            Self::uniform_face(3),
            Self::uniform_face(4),
            Self::uniform_face(5),
        ];
        return Self { faces };
    }

    #[inline]
    const fn uniform_face(colour: u32) -> u32 {
        colour * 0x11111111
    }

    pub fn to_facelet_array(self) -> [i64; 48] {
        c![Self::face_index(f,i) as i64, for i in 0..8, for f in self.faces]
            .try_into()
            .unwrap()
    }

    fn face_index(f: u32, n: u32) -> u32 {
        f >> (n * 4) & 0xf
    }

    fn get_facelet(&self, face: Face, n: u32) -> u32 {
        Self::face_index(self[face], n)
    }

    #[inline]
    pub const fn connections(face: Face) -> [(Face, u32); 4] {
        match face {
            Face::U => [(Face::B, 0), (Face::R, 0), (Face::F, 0), (Face::L, 0)],
            Face::D => [(Face::B, 2), (Face::L, 2), (Face::F, 2), (Face::R, 2)],
            Face::L => [(Face::U, 3), (Face::F, 3), (Face::D, 3), (Face::B, 1)],
            Face::R => [(Face::U, 1), (Face::B, 3), (Face::D, 1), (Face::F, 1)],
            Face::F => [(Face::U, 2), (Face::R, 3), (Face::D, 0), (Face::L, 1)],
            Face::B => [(Face::U, 0), (Face::L, 3), (Face::D, 2), (Face::R, 1)],
        }
    }

    pub fn rotate(&mut self, face: Face, n: u32) {
        self[face] = self[face].rotate_left(8 * n);
        let faces = Self::connections(face);
        let mut triples: [MaybeUninit<u32>; 4] = [const { MaybeUninit::uninit() }; 4];
        for i in 0..4 {
            let (face, offset) = faces[i];
            triples[i].write(self[face].rotate_right(8 * offset) & 0xfff);
        }
        let triples: [u32; 4] = unsafe { std::mem::transmute(triples) };

        for i in 0..4 {
            let (face, offset) = faces[i];
            let mask = 0xfffu32.rotate_left(8 * offset);
            self[face] =
                self[face] & !mask | triples[(i + 4 - n as usize) % 4].rotate_left(8 * offset)
        }
    }
}
