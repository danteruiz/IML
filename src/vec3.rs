// vec3.rs
//
// Created on 2021/10/12 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use super::ops::{Cross, Dot, Normalize};
use super::{Point3, Vec4};

use std::cmp::PartialEq;
use std::convert::From;
use std::fmt;
use std::ops::{Add, Div, Index, IndexMut, Mul, MulAssign, Sub};

// Vec3
#[derive(Debug, Copy, Clone)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Vec3 = Vector3<f32>;

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z }
    }

    pub fn as_ptr(&self) -> *const T {
        &self.x
    }
}

macro_rules! vector3_common {
    ($vec3: ident, $data_type: ident) => {
        impl $vec3 {
            pub fn inverse(&self) -> $vec3 {
                self.clone() * -1.0
            }

            pub fn cos(&self) -> $vec3 {
                Vec3 {
                    x: self.x.cos(),
                    y: self.y.cos(),
                    z: self.z.cos(),
                }
            }

            pub fn sin(&self) -> $vec3 {
                Vec3 {
                    x: self.x.sin(),
                    y: self.y.sin(),
                    z: self.z.sin(),
                }
            }

            pub fn to_radians(&self) -> $vec3 {
                Vec3 {
                    x: self.x.to_radians(),
                    y: self.y.to_radians(),
                    z: self.z.to_radians(),
                }
            }
        }

        impl From<Vec4> for $vec3 {
            fn from(v: Vec4) -> $vec3 {
                Vec3::new(v.x, v.y, v.z)
            }
        }

        impl From<[$data_type; 3]> for $vec3 {
            fn from(data: [$data_type; 3]) -> $vec3 {
                $vec3::new(data[0], data[1], data[2])
            }
        }

        impl Cross for $vec3 {
            type Output = Self;
            fn cross(&self, v2: &Self) -> $vec3 {
                Vec3 {
                    x: self.y * v2.z - self.z * v2.y,
                    y: self.z * v2.x - self.x * v2.z,
                    z: self.x * v2.y - self.y * v2.x,
                }
            }
        }

        impl Dot for $vec3 {
            type Output = $data_type;
            fn dot(&self, v: &Self) -> Self::Output {
                self.x * v.x + self.y * v.y + self.z * v.z
            }
        }

        impl Dot<&Point3> for Vec3 {
            type Output = f32;
            fn dot(&self, p: &&Point3) -> Self::Output {
                self.x * p.x + self.y * p.y + self.z * p.z
            }
        }

        impl Normalize for $vec3 {
            fn normalize(&self) -> $vec3 {
                let magnitude = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
                return *self / magnitude;
            }
        }

        impl Mul<$data_type> for $vec3 {
            type Output = Self;
            fn mul(self, scalar: $data_type) -> Self {
                Self {
                    x: scalar * self.x,
                    y: scalar * self.y,
                    z: scalar * self.z,
                }
            }
        }

        impl Mul<$vec3> for $data_type {
            type Output = $vec3;
            #[inline(always)]
            fn mul(self, v: $vec3) -> Self::Output {
                v * self
            }
        }

        impl MulAssign<$data_type> for $vec3 {
            fn mul_assign(&mut self, scalar: f32) {
                self.x *= scalar;
                self.y *= scalar;
                self.z *= scalar;
            }
        }

        impl Mul for $vec3 {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self {
                Self {
                    x: self.x * rhs.x,
                    y: self.y * rhs.y,
                    z: self.z * rhs.z,
                }
            }
        }

        impl Sub for $vec3 {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                Self {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                    z: self.z - rhs.z,
                }
            }
        }

        impl Div<$data_type> for $vec3 {
            type Output = Self;
            fn div(self, scalar: $data_type) -> Self {
                Self {
                    x: self.x / scalar,
                    y: self.y / scalar,
                    z: self.z / scalar,
                }
            }
        }

        impl Add for $vec3 {
            type Output = Self;
            #[inline(always)]
            fn add(self, other: Self) -> Self {
                Self {
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                }
            }
        }

        impl PartialEq for $vec3 {
            fn eq(&self, other: &Self) -> bool {
                self.x == other.x && self.y == other.y && self.z == other.z
            }

            fn ne(&self, other: &Self) -> bool {
                !(self == other)
            }
        }

        impl Index<usize> for $vec3 {
            type Output = $data_type;
            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    0 => &self.x,
                    1 => &self.y,
                    2 => &self.z,
                    _ => panic!("Vec3 index out of bound: {}", index),
                }
            }
        }

        impl IndexMut<usize> for $vec3 {
            fn index_mut(&mut self, index: usize) -> &mut $data_type {
                match index {
                    0 => &mut self.x,
                    1 => &mut self.y,
                    2 => &mut self.z,
                    _ => panic!("Vec3 index out of bound: {}", index),
                }
            }
        }

        impl fmt::Display for $vec3 {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({:.6}, {:.6}, {:.6}", self.x, self.y, self.z,)
            }
        }
    };
}

vector3_common!(Vec3, f32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot() {
        let v1 = Vec3::new(3.0, -2.0, 7.0);
        let v2 = Vec3::new(0.0, 4.0, -1.0);

        let result = -15.0;
        assert_eq!(Vec3::dot(&v1, &v2), result);
    }

    #[test]
    fn cross() {
        let v1 = Vec3::new(1.0, 3.0, 4.0);
        let v2 = Vec3::new(2.0, -5.0, 8.0);

        let result = Vec3::new(44.0, 0.0, -11.0);
        assert_eq!(Vec3::cross(&v1, &v2), result);
    }

    #[test]
    fn add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        let result = Vec3::new(5.0, 7.0, 9.0);

        assert_eq!(v1 + v2, result);
    }

    #[test]
    fn sub() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        let result = Vec3::new(-3.0, -3.0, -3.0);
        assert_eq!(v1 - v2, result);
    }

    #[test]
    fn add_sub() {
        let v1 = Vec3::new(4.0, 5.0, 6.0);
        let v2 = Vec3::new(7.0, -3.0, 0.0);
        let v3 = Vec3::new(1.0, 2.0, 3.0);

        let result = Vec3::new(10.0, 0.0, 3.0);
        assert_eq!(v1 + v2 - v3, result);
    }

    #[test]
    fn normalize() {
        let v = Vec3::new(0.0, 0.0, -10.0);
        let result = Vec3::new(0.0, 0.0, -1.0);

        assert_eq!(v.normalize(), result);
    }
}
