use crate::parser::{self, parse_input, LineType, Meta, Quadrilateral, SubFileReference, Vertex};
use std::collections::HashMap;
use std::{
  fs,
  path::{Path, PathBuf},
};
use stl_io;

fn surface_normal(triangle: &parser::Triangle) -> Vertex {
  let u = triangle[1] - triangle[0];
  let v = triangle[2] - triangle[0];

  u.cross(v)
}

fn tris_from_quad(quad: Quadrilateral) -> [parser::Triangle; 2] {
  [
    parser::Triangle::from(quad[0], quad[1], quad[2]),
    parser::Triangle::from(quad[2], quad[3], quad[0]),
  ]
}

fn stl_tri_from_tri(tri: &parser::Triangle) -> stl_io::Triangle {
  stl_io::Triangle {
    normal: surface_normal(&tri).data,
    vertices: [tri[0].data, tri[1].data, tri[2].data],
  }
}

fn align_vertex(sub: &SubFileReference, vert: Vertex) -> Vertex {
  let mat = sub.matrix;
  let pos = (
    (mat[(0, 0)] * vert[0])
      + (mat[(0, 1)] * vert[1])
      + (mat[(0, 2)] * vert[2])
      + sub.coordinates[0] + 100.,
    (mat[(1, 0)] * vert[0])
      + (mat[(1, 1)] * vert[1])
      + (mat[(1, 2)] * vert[2])
      + sub.coordinates[1],
    (mat[(2, 0)] * vert[0])
      + (mat[(2, 1)] * vert[1])
      + (mat[(2, 2)] * vert[2])
      + sub.coordinates[2],
  );
  Vertex::from(pos.0, pos.1, pos.2)
}

fn align_tri(sub: &SubFileReference, tri: &stl_io::Triangle, invert: bool) -> stl_io::Triangle {
  if invert {
    stl_io::Triangle {
      normal: tri.normal,
      vertices: [
        align_vertex(sub, Vertex::from_array(tri.vertices[0])).data,
        align_vertex(sub, Vertex::from_array(tri.vertices[2])).data,
        align_vertex(sub, Vertex::from_array(tri.vertices[1])).data,
      ],
    }
  } else {
    stl_io::Triangle {
      normal: tri.normal,
      vertices: [
        align_vertex(sub, Vertex::from_array(tri.vertices[0])).data,
        align_vertex(sub, Vertex::from_array(tri.vertices[1])).data,
        align_vertex(sub, Vertex::from_array(tri.vertices[2])).data,
      ],
    }
  }
}

#[derive(Clone)]
pub struct Transform {
  cache: HashMap<PathBuf, Vec<stl_io::Triangle>>,
  pub ldraw_path: PathBuf,
  pub next_inverted: bool,
}

impl Transform {
  pub fn new(ldraw_path: impl Into<PathBuf>) -> Self {
    Transform {
      cache: HashMap::new(),
      ldraw_path: ldraw_path.into(),
      next_inverted: false,
    }
  }

  pub fn transform(&mut self, lines: Vec<LineType>, inverted: bool) -> Vec<stl_io::Triangle> {
    lines
      .iter()
      .filter_map(|line| match line {
        LineType::CommentMeta(meta) => {
          if *meta == Meta::Invert {
            self.next_inverted = !self.next_inverted
          };
          None
        }
        LineType::Triangle(tri) => Some(vec![stl_tri_from_tri(&tri)]),
        LineType::Quadrilateral(quad) => {
          Some(tris_from_quad(*quad).iter().map(stl_tri_from_tri).collect())
        }
        LineType::SubFileReference(sub) => {
          let path = self.ldraw_path.join(&sub.file_name);
          let tris = self.get_sub_file(path).unwrap();
          Some(
            tris
              .iter()
              .map(|tri| align_tri(sub, tri, inverted))
              .collect(),
          )
        }
        _ => None,
      })
      .flatten()
      .collect()
  }

  fn get_sub_file<'b>(&mut self, path: impl AsRef<Path>) -> Option<Vec<stl_io::Triangle>> {
    let path = self.ldraw_path.join(path);
    let data = fs::read(&path).unwrap();
    let input = String::from_utf8_lossy(&data);
    let tris = self.transform(parse_input(&input), self.next_inverted);
    if self.next_inverted {
      self.next_inverted = false
    }
    Some(tris)
    // self
    //   .cache
    //   .get(&path)
    //   .map(|v| v.clone())
    //   .or_else(|| {
    //     let data = fs::read(&path).unwrap();
    //     let input = String::from_utf8_lossy(&data);
    //     let result = self.transform(parse_input(&input));
    //     self
    //       .cache
    //       .insert(path, result)
    //   })
  }
}
