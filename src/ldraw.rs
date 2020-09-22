use crate::result::{ErrorType, Result};
use vecmat::mat::Mat3x3;
use vecmat::vec::Vec3;

#[derive(Clone, PartialEq, Debug)]
pub struct LDraw<'a> {
  pub lines: Vec<LDrawCommand<'a>>,
}

impl<'a> LDraw<'a> {
  pub fn read(raw: &'a str) -> Result<LDraw<'a>> {
    let lines: Vec<LDrawCommand<'a>> = raw
      .lines()
      .map(|line| LDrawCommand::read(line))
      .scan(false, |prev_was_error, elem| {
        if *prev_was_error {
          return None;
        }
        *prev_was_error = elem.is_err();
        Some(elem)
      })
      .filter_map(|line| match line {
        Ok(Some(val)) => Some(Ok(val)),
        Err(err) => Some(Err(err)),
        _ => None,
      })
      .collect::<Result<_>>()?;

    Ok(LDraw { lines })
  }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LDrawCommand<'a> {
  MetaOrComment(MetaOrComment<'a>),
  SubFileReference(SubFileReference<'a>),
  Line,
  Triangle(Triangle),
  Quadrilateral(Quadrilateral),
  Optional,
}

impl<'a> LDrawCommand<'a> {
  pub fn read(raw: &'a str) -> Result<Option<LDrawCommand<'a>>> {
    let trim = raw.trim();
    let mut args = trim.split_ascii_whitespace();

    if let Some(line_type) = args.next() {
      match line_type {
        "0" => Ok(
          trim
            .strip_prefix("0 ")
            .map(|v| LDrawCommand::MetaOrComment(MetaOrComment(v))),
        ),
        "1" => Ok(Some(LDrawCommand::SubFileReference(
          SubFileReference::read(args)?,
        ))),
        "2" => Ok(Some(LDrawCommand::Line)),
        "3" => Ok(Some(LDrawCommand::Triangle(Triangle::read(args)?))),
        "4" => Ok(Some(LDrawCommand::Quadrilateral(Quadrilateral::read(
          args,
        )?))),
        "5" => Ok(Some(LDrawCommand::Optional)),
        _ => Err(err_invalid!("unknown command type"))?,
      }
    } else {
      Ok(None)
    }
  }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BFC {
  InvertNext,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct MetaOrComment<'a>(&'a str);

impl<'a> MetaOrComment<'a> {
  pub fn bfc(&self) -> Option<BFC> {
    match self.0.strip_prefix("BFC ") {
      Some("INVERTNEXT") => Some(BFC::InvertNext),
      _ => None,
    }
  }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SubFileReference<'a> {
  pub coords: Vec3<f32>,
  pub matrix: Mat3x3<f32>,
  pub file_name: &'a str,
}

impl<'a> SubFileReference<'a> {
  pub fn read<I: IntoIterator<Item = &'a str> + Clone>(raw: I) -> Result<SubFileReference<'a>> {
    let file_name = raw
      .clone()
      .into_iter()
      .last()
      .ok_or(err_malformed!("filename missing for subfile reference"))?;

    let verts = raw
      .into_iter()
      .by_ref()
      .skip(1)
      .take(12)
      .map(|v| v.parse::<f32>().unwrap())
      .collect::<Vec<f32>>()
      .clone();

    Ok(SubFileReference {
      coords: Vec3::from(verts[0], verts[1], verts[2]),
      matrix: Mat3x3::from(
        verts[3], verts[4], verts[5], verts[6], verts[7], verts[8], verts[9], verts[10], verts[11],
      ),
      file_name: file_name,
    })
  }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vertex(pub Vec3<f32>);

impl Vertex {
  pub fn new(x: f32, y: f32, z: f32) -> Vertex {
    Vertex(Vec3::from(x, y, z))
  }

  pub fn read<'a, I: IntoIterator<Item = &'a str>>(raw: I) -> Result<Vertex> {
    let points = raw
      .into_iter()
      .take(3)
      .map(|i| i.parse().map(|v| Ok(v))?)
      .scan(false, |prev_was_error, elem| {
        if *prev_was_error {
          return None;
        }
        *prev_was_error = elem.is_err();
        Some(elem)
      })
      .collect::<Result<Vec<f32>>>()?;

    Ok(Vertex(Vec3::from(points[0], points[1], points[2])))
  }

  #[inline]
  pub fn x(&self) -> f32 {
    self.0[0]
  }

  #[inline]
  pub fn y(&self) -> f32 {
    self.0[1]
  }

  #[inline]
  pub fn z(&self) -> f32 {
    self.0[2]
  }

  #[inline]
  pub fn points(self) -> [f32; 3] {
    self.0.data
  }
}

impl From<Vec3<f32>> for Vertex {
  fn from(vec: Vec3<f32>) -> Vertex {
    Vertex(vec)
  }
}

impl From<[f32; 3]> for Vertex {
  fn from(arr: [f32; 3]) -> Vertex {
    Vertex::new(arr[0], arr[1], arr[2])
  }
}

impl From<Vertex> for [f32; 3] {
  fn from(vert: Vertex) -> [f32; 3] {
    vert.points()
  }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Triangle(pub Vertex, pub Vertex, pub Vertex);

impl Triangle {
  pub fn new(p1: Vertex, p2: Vertex, p3: Vertex) -> Triangle {
    Triangle(p1, p2, p3)
  }

  pub fn read<'a, I: IntoIterator<Item = &'a str> + Clone>(raw: I) -> Result<Triangle> {
    let p1 = Vertex::read(raw.clone().into_iter().skip(1))?;
    let p2 = Vertex::read(raw.clone().into_iter().skip(4))?;
    let p3 = Vertex::read(raw.clone().into_iter().skip(7))?;
    let tri = Triangle(p1, p2, p3);
    Ok(tri)
  }

  #[inline]
  pub fn verticies(self) -> [Vertex; 3] {
    [self.0, self.1, self.2]
  }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Quadrilateral(pub Vertex, pub Vertex, pub Vertex, pub Vertex);

impl Quadrilateral {
  pub fn new(p1: Vertex, p2: Vertex, p3: Vertex, p4: Vertex) -> Quadrilateral {
    Quadrilateral(p1, p2, p3, p4)
  }
  pub fn read<'a, I: IntoIterator<Item = &'a str> + Clone>(raw: I) -> Result<Quadrilateral> {
    let p1 = Vertex::read(raw.clone().into_iter().skip(1))?;
    let p2 = Vertex::read(raw.clone().into_iter().skip(4))?;
    let p3 = Vertex::read(raw.clone().into_iter().skip(7))?;
    let p4 = Vertex::read(raw.clone().into_iter().skip(10))?;
    let quad = Quadrilateral(p1, p2, p3, p4);
    Ok(quad)
  }

  #[inline]
  pub fn verticies(self) -> [Vertex; 4] {
    [self.0, self.1, self.2, self.3]
  }
}
