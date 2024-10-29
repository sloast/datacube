#![allow(dead_code)]
use cute::c;
use std::{
    mem::MaybeUninit,
    ops::{Deref, Index, IndexMut},
};
#[allow(unused_imports)]
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct CubeState {
    faces: [CubeFaceState; 6],
}
type CubeFaceState = u32;

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum Face {
    U,
    D,
    L,
    R,
    F,
    B,
}

impl Deref for CubeState {
    type Target = [CubeFaceState; 6];

    fn deref(&self) -> &Self::Target {
        return &self.faces;
    }
}
impl Index<Face> for CubeState {
    type Output = CubeFaceState;

    #[inline]
    fn index(&self, index: Face) -> &Self::Output {
        return &self.faces[index as usize];
    }
}
impl IndexMut<Face> for CubeState {
    #[inline]
    fn index_mut(&mut self, index: Face) -> &mut Self::Output {
        return &mut self.faces[index as usize];
    }
}

impl CubeState {
    pub fn new() -> Self {
        let mut faces = [0u32; 6];
        for i in 0..6 {
            faces[i] = Self::uniform_face(i as u32)
        }
        return Self { faces };
    }

    fn uniform_face(colour: u32) -> u32 {
        return colour * 0x11111111;
    }

    pub fn to_facelets(&self) -> [i64; 48] {
        c![Self::face_index(f,i) as i64, for i in 0..8, for f in self.faces]
            .try_into()
            .unwrap()
    }

    fn face_index(f: CubeFaceState, n: u32) -> u32 {
        f >> n*4 & 0xf
    }

    fn get_facelet(&self, face: Face, n: u32) -> u32 {
        Self::face_index(self[face], n)
    }

    pub fn rotate(&mut self, face: Face, n: u32) {
        self[face] = self[face].rotate_left(8 * n);
        let faces = match face {
            Face::U => [(Face::B, 0), (Face::R, 0), (Face::F, 0), (Face::L, 0)],
            Face::D => [(Face::B, 2), (Face::L, 2), (Face::F, 2), (Face::R, 2)],
            Face::L => [(Face::U, 3), (Face::F, 3), (Face::D, 3), (Face::B, 1)],
            Face::R => [(Face::U, 1), (Face::B, 3), (Face::D, 1), (Face::F, 1)],
            Face::F => [(Face::U, 2), (Face::R, 3), (Face::D, 0), (Face::L, 1)],
            Face::B => [(Face::U, 0), (Face::L, 3), (Face::D, 2), (Face::R, 1)],
        };
        let mut triples: [MaybeUninit<u32>; 4] = [const { MaybeUninit::uninit() }; 4];
        for i in 0..4 {
            let (face, offset) = faces[i];
            triples[i].write(self[face].rotate_right(8 * offset) & 0xfff);
        }
        let triples = unsafe { std::mem::transmute::<_, [u32; 4]>(triples) };

        for i in 0..4 {
            let (face, offset) = faces[i];
            let mask = 0xfffu32.rotate_left(8 * offset);
            self[face] = self[face] & !mask | triples[(i + 4 - n as usize) % 4].rotate_left(8 * offset)
        }
    }
}
