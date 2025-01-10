mod vec3;
mod rayunit;
mod optarg;
mod aabb;
//mod bvh;
pub use vec3::*;
pub use rayunit::*;
pub use optarg::*;
pub use aabb::*;
//pub use bvh::*;
    
pub const EPS: f64 = 1e-6;
pub const INF: f64 = 1e20;
pub const FRAC_SQRT_3: f64 = 1.732050807568877293527446341505872367;
