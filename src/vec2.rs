// vec2.rs
//
// Created on 2021/12/04 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use std::fmt;

use super::ops::{Dot, Length, Normalize};
use std::ops::{Div, Index, IndexMut, Mul, MulAssign}; //, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

pub type Vec2 = Vector2<f32>;
pub type Vec2I = Vector2<i32>;

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 { x, y }
    }

    pub fn as_ptr(&self) -> *const T {
        &self.x
    }
}

macro_rules! vector2_common {
    ($vec2: ident, $data_type: ident) => {
        impl fmt::Display for $vec2 {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({:.6}, {:.6}", self.x, self.y,)
            }
        }

        impl Dot for $vec2 {
            type Output = $data_type;
            #[inline(always)]
            fn dot(&self, v2: &$vec2) -> Self::Output {
                self.x * v2.x + self.y * v2.y
            }
        }

        impl Length for $vec2 {
            type Output = $data_type;
            #[inline(always)]
            fn length(&self) -> Self::Output {
                // i32 doesn't support sqrt. Hack convert to f32 then back to i32
                // @FIXME there will be precision loss if $data_type is greater than f32
                ((self.x * self.x + self.y * self.y) as f32).sqrt() as $data_type
            }
        }

        impl<'a> Div<$data_type> for &'a $vec2 {
            type Output = $vec2;
            #[inline(always)]
            fn div(self, scalar: $data_type) -> Self::Output {
                Self::Output {
                    x: self.x / scalar,
                    y: self.y / scalar,
                }
            }
        }

        impl Div<$data_type> for $vec2 {
            type Output = $vec2;
            #[inline(always)]
            fn div(self, scalar: $data_type) -> Self::Output {
                Self::Output {
                    x: self.x / scalar,
                    y: self.y / scalar,
                }
            }
        }

        impl Normalize for $vec2 {
            #[inline(always)]
            fn normalize(&self) -> $vec2 {
                self / self.length()
            }
        }

        impl Index<usize> for $vec2 {
            type Output = $data_type;
            #[inline(always)]
            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    0 => &self.x,
                    1 => &self.y,
                    _ => panic!("Vec2 index out of bound: {}", index),
                }
            }
        }

        impl IndexMut<usize> for $vec2 {
            fn index_mut(&mut self, index: usize) -> &mut $data_type {
                match index {
                    0 => &mut self.x,
                    1 => &mut self.y,
                    _ => panic!("Vec2 index out of bound: {}", index),
                }
            }
        }

        impl Mul<$data_type> for $vec2 {
            type Output = Self;
            #[inline(always)]
            fn mul(self, scalar: $data_type) -> Self {
                Self {
                    x: scalar * &self.x,
                    y: scalar * &self.y,
                }
            }
        }

        impl MulAssign<$data_type> for $vec2 {
            #[inline(always)]
            fn mul_assign(&mut self, scalar: $data_type) {
                self.x *= scalar;
                self.y *= scalar;
            }
        }

        impl Mul for $vec2 {
            type Output = Self;
            #[inline(always)]
            fn mul(self, v: Self) -> Self {
                Self {
                    x: self.x * &v.x,
                    y: self.y * &v.y,
                }
            }
        }

        impl Mul<$vec2> for $data_type {
            type Output = $vec2;
            #[inline(always)]
            fn mul(self, v: $vec2) -> $vec2 {
                v * self
            }
        }
    };
}

vector2_common!(Vec2, f32);
vector2_common!(Vec2I, i32);
