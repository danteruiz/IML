// point.rs
//
// Created on 2021/10/17 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

//use crate::math::ops::{Cross, Dot};
use super::vec3::Vec3;
use std::fmt;
//use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};
use std::{
    convert::From,
    ops::{Add, Mul, Sub},
};

#[derive(Debug, Copy, Clone)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Point3 {
        Point3 { x, y, z }
    }

    pub fn inverse(point: Point3) -> Point3 {
        point.clone() * -1.0
    }

    pub fn as_ptr(&self) -> *const f32 {
        &self.x
    }
}

impl Mul<f32> for Point3 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: scalar * self.x,
            y: scalar * self.y,
            z: scalar * self.z,
        }
    }
}

impl From<Vec3> for Point3 {
    fn from(v: Vec3) -> Point3 {
        Point3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl Mul<f32> for &Point3 {
    type Output = Point3;
    fn mul(self, scalar: f32) -> Point3 {
        Point3 {
            x: scalar * self.x,
            y: scalar * self.y,
            z: scalar * self.z,
        }
    }
}

impl Add<Vec3> for Point3 {
    type Output = Self;
    fn add(self, v: Vec3) -> Self {
        Point3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl Add<Vec3> for &Point3 {
    type Output = Point3;
    fn add(self, v: Vec3) -> Point3 {
        Point3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl Sub for Point3 {
    type Output = Vec3;
    fn sub(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub for &Point3 {
    type Output = Vec3;
    fn sub(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl fmt::Display for Point3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.6}, {:.6}, {:.6}", self.x, self.y, self.z,)
    }
}

// macro_rules! point {
//     ($name: ident, $data_type: ident, $($x:ident),*) => {
//
// 	struct $name {
// 	    $(
// 		$x: $data_type,
// 	    )*
// 	}
//     };
// }
//
// point!(Point4, f32, [x, y, z, w]);
