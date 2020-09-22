use crate::ldraw::{self, LDraw, LDrawCommand, BFC};
use crate::result::Result;
use crate::traits::{Determinant, Mul, Subdivide, SurfaceNormal};
use std::fs;
use std::path::{Path, PathBuf};
use stl_io::Triangle;
use vecmat::mat::Mat4x4;

impl From<ldraw::Triangle> for Triangle {
  fn from(tri: ldraw::Triangle) -> Triangle {
    Triangle {
      normal: tri.surface_normal().0.data,
      vertices: [
        [tri.0.x(), tri.0.y(), tri.0.z()],
        [tri.1.x(), tri.1.y(), tri.1.z()],
        [tri.2.x(), tri.2.y(), tri.2.z()],
      ],
    }
  }
}

pub struct Parser<'a> {
  inverted: bool,
  ldraw_path: &'a Path,
}

impl<'a> Parser<'a> {
  pub fn new<P: AsRef<Path> + ?Sized + std::fmt::Debug>(path: &'a P) -> Parser<'a> {
    Parser {
      inverted: false,
      ldraw_path: path.as_ref(),
    }
  }

  pub fn parse<P: AsRef<Path> + ?Sized + std::fmt::Debug>(
    &self,
    path: &'a P,
  ) -> Result<Vec<Triangle>> {
    let file = fs::read_to_string(path)?;
    let ldraw = LDraw::read(&file)?;
    Ok(
      self
        .into_triangles(ldraw)?
        .iter()
        .map(|&tri| Triangle::from(tri))
        .collect(),
    )
  }

  pub fn parse_triangles<P: AsRef<Path> + ?Sized + std::fmt::Debug>(
    &self,
    path: &'a P,
  ) -> Result<Vec<ldraw::Triangle>> {
    let file = fs::read_to_string(path)?;
    let ldraw = LDraw::read(&file)?;
    self.into_triangles(ldraw)
  }

  pub fn into_triangles(&self, ldraw: LDraw) -> Result<Vec<ldraw::Triangle>> {
    let mut invert_next = false;
    ldraw.lines.iter().try_fold(
      vec![],
      |mut triangles, &cmd| -> Result<Vec<ldraw::Triangle>> {
        match cmd {
          LDrawCommand::MetaOrComment(meta) => {
            if meta.bfc() == Some(BFC::InvertNext) {
              invert_next = true;
            };
            Ok(triangles)
          }
          LDrawCommand::SubFileReference(subfile) => {
            let matrix: Mat4x4<f32> = subfile.into();
            let det = matrix.determinant();

            let path = ["p", "p/48", "parts", "models"]
              .iter()
              .map(|t| {
                let mut full_path = PathBuf::from(self.ldraw_path);
                full_path.push(t);
                full_path.push(subfile.file_name);
                full_path
              })
              .try_find(|path| {
                let meta = fs::metadata(path).ok()?;
                Some(meta.is_file())
              })
              .unwrap()
              .unwrap();

            let parser = Parser {
              inverted: det < 1.0 || invert_next ^ self.inverted,
              ldraw_path: self.ldraw_path,
            };

            let mut tris = parser
              .parse_triangles(&path)?
              .iter()
              .map(|tri| ldraw::Triangle(matrix.mul(tri.0), matrix.mul(tri.1), matrix.mul(tri.2)))
              .collect();
            invert_next = false;
            triangles.append(&mut tris);
            Ok(triangles)
          }
          LDrawCommand::Triangle(tri) => Ok(vec![tri.into()]),
          LDrawCommand::Quadrilateral(quad) => {
            let mut tris = quad
              .subdivide(self.inverted)
              .iter()
              .map(|&tri| tri.into())
              .collect();

            triangles.append(&mut tris);
            Ok(triangles)
          }
          _ => Ok(triangles),
        }
      },
    )
  }
}
