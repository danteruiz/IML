// mod.rs
//
// Created on 2021/10/12 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

mod vec2;
pub use vec2::Vec2;
mod vec3;
pub use vec3::Vec3;
mod vec4;
pub use vec4::Vec4;
mod mat3;
pub use mat3::Mat3;
mod mat4;
pub use mat4::Mat4;
pub mod ops;
mod point3;
pub use point3::Point3;
mod transform;
pub use transform::Transform;

mod quat;
pub use quat::Quat;
pub mod shared;

// common function

pub fn dot<T: ops::Dot>(v1: &T, v2: &T) -> <T as ops::Dot>::Output {
    T::dot(&v1, &v2)
}

pub fn length<T: ops::Length>(v: &T) -> <T as ops::Length>::Output {
    v.length()
}

pub fn cross<T: ops::Cross>(v1: &T, v2: &T) -> <T as ops::Cross>::Output {
    T::cross(&v1, &v2)
}
