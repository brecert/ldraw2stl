use vecmat::vec::Vec3;

#[derive(Clone, Copy)]
pub struct Vertex(pub Vec3<f32>);

impl Vertex {
    pub fn new(a: f32, b: f32, c: f32) -> Vertex {
        Vertex(Vec3::from(a, b, c))
    }
}

#[derive(Clone, Copy)]
pub struct Triangle(pub [Vertex; 3]);

impl Triangle {
    pub fn surface_normal(&self) -> Vertex {
        let u = self.0[1].0 - self.0[2].0;
        let v = self.0[2].0 - self.0[0].0;
        Vertex(u.cross(v))
    }
}

impl From<Triangle> for stl_io::Triangle {
    fn from(tri: Triangle) -> stl_io::Triangle {
        stl_io::Triangle {
            normal: tri.surface_normal().0.data,
            vertices: [tri.0[0].0.data, tri.0[1].0.data, tri.0[2].0.data],
        }
    }
}

pub trait Determinant {
    fn determinant(self: &Self) -> f32;
}

impl Determinant for [f32; 16] {
    fn determinant(&self) -> f32 {
        let [a00, a01, a02, a03, a10, a11, a12, a13, a20, a21, a22, a23, a30, a31, a32, a33] = self;
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

pub fn mul_mat_vert(mat: [f32; 16], vert: Vertex) -> Vertex {
    let [a1, a2, a3, a4, b1, b2, b3, b4, c1, c2, c3, c4, ..] = mat;

    let [old_x, old_y, old_z] = vert.0.data;

    Vertex::new(
        a1 * old_x + a2 * old_y + a3 * old_z + a4,
        b1 * old_x + b2 * old_y + b3 * old_z + b4,
        c1 * old_x + c2 * old_y + c3 * old_z + c4,
    )
}
