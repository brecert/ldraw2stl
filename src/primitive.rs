use vecmat::mat::{Mat3x3, Mat4x4};
use vecmat::vec::{Vec3, Vec4};

pub type Mat = Mat3x3<f32>;
pub type Matrix = Mat4x4<f32>;
pub type Vertex = Vec3<f32>;
pub type Triangle = Vec3<Vertex>;
pub type Quadrilateral = Vec4<Vertex>;

pub trait SurfaceNormal {
    fn surface_normal(self: &Self) -> Vertex;
}

impl SurfaceNormal for Triangle {
    fn surface_normal(&self) -> Vertex {
        let u = self[1] - self[0];
        let v = self[2] - self[0];
        u.cross(v)
    }
}

pub trait Determinant {
    fn determinant(self: &Self) -> f32;
}

impl Determinant for Matrix {
    fn determinant(&self) -> f32 {
        let [a00, a01, a02, a03, a10, a11, a12, a13, a20, a21, a22, a23, a30, a31, a32, a33] =
            self.data;
        let b00 = a00 * a11 - a01 * a10;
        let b01 = a00 * a12 - a02 * a10;
        let b02 = a00 * a13 - a03 * a10;
        let b03 = a01 * a12 - a02 * a11;
        let b04 = a01 * a13 - a03 * a11;
        let b05 = a02 * a13 - a03 * a12;
        let b06 = a20 * a31 - a21 * a30;
        let b07 = a20 * a32 - a22 * a30;
        let b08 = a20 * a33 - a23 * a30;
        let b09 = a21 * a32 - a22 * a31;
        let b10 = a21 * a33 - a23 * a31;
        let b11 = a22 * a33 - a23 * a32;

        b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06
    }
}

pub trait Subdivide {
    fn subdivide(&self, invert: bool) -> Vec<Triangle>;
}

impl Subdivide for Quadrilateral {
    fn subdivide(&self, invert: bool) -> Vec<Triangle> {
        match invert {
            true => vec![
                Triangle::from(self[0], self[2], self[1]),
                Triangle::from(self[2], self[3], self[0]),
            ],
            false => vec![
                Triangle::from(self[0], self[1], self[2]),
                Triangle::from(self[2], self[0], self[3]),
            ],
        }
    }
}

pub trait Mul<T> {
  fn mul(&self, other: T) -> T;
}

impl Mul<Vertex> for Matrix {
  fn mul(&self, vert: Vertex) -> Vertex {
      Vertex::from(
        (self[(0, 0)] * vert[0]) + (self[(0, 1)] * vert[1]) + (self[(0, 2)] * vert[2]) + self[(0, 3)],
        (self[(1, 0)] * vert[0]) + (self[(1, 1)] * vert[1]) + (self[(1, 2)] * vert[2]) + self[(1, 3)],
        (self[(2, 0)] * vert[0]) + (self[(2, 1)] * vert[1]) + (self[(2, 2)] * vert[2]) + self[(2, 3)],
    )
  }
}