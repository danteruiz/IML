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
mod ops;
mod point3;
pub use point3::Point3;
mod transform;
pub use transform::Transform;

mod quat;
pub use quat::Quat;
pub mod shared;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
