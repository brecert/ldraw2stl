// use std::fs::File;
// use std::io::{BufRead, BufReader};
// use std::path::Path;
// use vecmat::mat::{Mat3x3, Mat4x4};
// use vecmat::vec::{Vec3, Vec4};

// type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// type Mat = Mat3x3<f32>;
// type Matrix = Mat4x4<f32>;
// type Vertex = Vec3<f32>;
// type Triangle = Vec3<Vertex>;
// type Quadrilateral = Vec4<Vertex>;

// trait SurfaceNormal {
//     fn surface_normal(self: &Self) -> Vertex;
// }

// impl SurfaceNormal for Triangle {
//     fn surface_normal(&self) -> Vertex {
//         let u = self[1] - self[0];
//         let v = self[2] - self[0];
//         u.cross(v)
//     }
// }

// trait Determinant {
//     fn determinant(self: &Self) -> f32;
// }

// impl Determinant for Matrix {
//     fn determinant(&self) -> f32 {
//         let [a00, a01, a02, a03, a10, a11, a12, a13, a20, a21, a22, a23, a30, a31, a32, a33] =
//             self.data;
//         let b00 = a00 * a11 - a01 * a10;
//         let b01 = a00 * a12 - a02 * a10;
//         let b02 = a00 * a13 - a03 * a10;
//         let b03 = a01 * a12 - a02 * a11;
//         let b04 = a01 * a13 - a03 * a11;
//         let b05 = a02 * a13 - a03 * a12;
//         let b06 = a20 * a31 - a21 * a30;
//         let b07 = a20 * a32 - a22 * a30;
//         let b08 = a20 * a33 - a23 * a30;
//         let b09 = a21 * a32 - a22 * a31;
//         let b10 = a21 * a33 - a23 * a31;
//         let b11 = a22 * a33 - a23 * a32;

//         b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06
//     }
// }

// trait Subdivide {
//     fn subdivide(&self, invert: bool) -> Vec<Triangle>;
// }

// impl Subdivide for Quadrilateral {
//     fn subdivide(&self, invert: bool) -> Vec<Triangle> {
//         match invert {
//             true => vec![
//                 Triangle::from(self[0], self[2], self[1]),
//                 Triangle::from(self[2], self[3], self[0]),
//             ],
//             false => vec![
//                 Triangle::from(self[0], self[1], self[2]),
//                 Triangle::from(self[2], self[0], self[3]),
//             ],
//         }
//     }
// }

// fn mat_x_vert(mat: Matrix, vert: Vertex) -> Vertex {
//     // mat + vert
//     Vertex::from(
//         (mat[(0, 0)] * vert[0]) + (mat[(0, 1)] * vert[1]) + (mat[(0, 2)] * vert[2]) + mat[(0, 3)],
//         (mat[(1, 0)] * vert[0]) + (mat[(1, 1)] * vert[1]) + (mat[(1, 2)] * vert[2]) + mat[(1, 3)],
//         (mat[(2, 0)] * vert[0]) + (mat[(2, 1)] * vert[1]) + (mat[(2, 2)] * vert[2]) + mat[(2, 3)],
//     )
// }

// struct Parser;

// impl Parser {
//     pub fn new() -> Self {
//         Parser {}
//     }

//     pub fn parse<B: BufRead>(self, buf: &B) -> Vec<Line> {
//         buf.lines()
//             .filter_map(|line| self.parse_line(line.unwrap()))
//             .collect()
//     }

//     fn parse_line(&mut self, line: String) -> Option<Line> {
//         let mut line = line.trim().split_ascii_whitespace();
//         let line_type = line.next();
//         let items = line.collect();

//         match line_type {
//             Some("0") => self.parse_comment_or_meta(&items).map(Line::Meta),
//             Some("1") => Some(self.parse_sub_file_reference(&items)).map(Line::SubFileReference),
//             Some("3") => Some(self.parse_triangle_command(&items)).map(Line::Triangle),
//             Some("4") => Some(self.parse_quadrilateral_command(&items)).map(Line::Quadrilateral),
//             _ => None,
//         }
//     }

//     fn parse_comment_or_meta(&mut self, items: &Vec<&str>) -> Option<BFC> {
//         let mut items = items.iter();
//         match items.next() {
//             Some(&"BFC") => match items.next() {
//                 Some(&"InvertNext") => {
//                     // self.invert_next = true;
//                     Some(BFC::InvertNext)
//                 }
//                 _ => None,
//             },
//             _ => None,
//         }
//     }

//     fn parse_sub_file_reference(&self, items: &Vec<&str>) -> SubFileReference {
//         let file_name = items.last().unwrap();
//         let verts: Vec<f32> = items
//             .iter()
//             .skip(1)
//             .take(12)
//             .map(|v| v.parse::<f32>().unwrap())
//             .collect();

//         let verts = verts.clone();

//         SubFileReference {
//             coords: Vertex::from(verts[0], verts[1], verts[2]),
//             matrix: Mat::from(
//                 verts[3], verts[4], verts[5], verts[3], verts[4], verts[5], verts[3], verts[4],
//                 verts[5],
//             ),
//             file_name: file_name.to_string(),
//         }
//     }

//     pub fn parse_command_vertexes(args: Vec<&str>) -> Vec<Vertex> {
//         use itertools::Itertools;

//         args.iter()
//             .map(|v| v.parse::<f32>().unwrap())
//             .tuples::<(_, _, _)>()
//             .map(|t| Vertex::from(t.0, t.1, t.2))
//             .collect()
//     }

//     fn parse_triangle_command(&self, items: &Vec<&str>) -> Triangle {
//         let verts: Vec<f32> = items
//             .iter()
//             .skip(1)
//             .take(9)
//             .map(|v| v.parse::<f32>().unwrap())
//             .collect();

//         Triangle::from(
//             Vertex::from(verts[0], verts[1], verts[2]),
//             Vertex::from(verts[3], verts[4], verts[5]),
//             Vertex::from(verts[6], verts[7], verts[8]),
//         )
//     }

//     fn parse_quadrilateral_command(&self, items: &Vec<&str>) -> Quadrilateral {
//         let verts: Vec<f32> = items
//             .iter()
//             .skip(1)
//             .take(12)
//             .map(|v| v.parse::<f32>().unwrap())
//             .collect();

//         Quadrilateral::from(
//             Vertex::from(verts[0], verts[1], verts[2]),
//             Vertex::from(verts[3], verts[4], verts[5]),
//             Vertex::from(verts[6], verts[7], verts[8]),
//             Vertex::from(verts[9], verts[10], verts[11]),
//         )
//     }
// }

// #[derive(Debug, PartialEq)]
// pub struct CommandArgs<'a> {
//     pub color: &'a str,
//     pub args: Vec<&'a str>,
// }

// #[derive(Debug, PartialEq)]
// pub struct SubFileReference {
//     pub coords: Vertex,
//     pub matrix: Mat,
//     pub file_name: String,
// }

// #[derive(Debug, PartialEq)]
// enum Line {
//     Meta(BFC),
//     SubFileReference(SubFileReference),
//     Triangle(Triangle),
//     Quadrilateral(Quadrilateral),
// }

// #[derive(Debug, PartialEq)]
// enum BFC {
//     InvertNext,
// }

// use stl_io;

// struct LDrawTransformer<'a> {
//     lines: Box<dyn Iterator<Item = Line>>,
//     ldraw_path: &'a Path,
// }

// impl<'a> LDrawTransformer<'a> {
//     pub fn new<P: AsRef<Path> + Sized>(
//         lines: Box<dyn Iterator<Item = Line>>,
//         ldraw_path: &'a P,
//     ) -> Self {
//         LDrawTransformer {
//             lines: lines,
//             ldraw_path: &ldraw_path.as_ref(),
//         }
//     }

//     fn transform(self) -> Result<Vec<stl_io::Triangle>> {
//         let tris = self
//             .lines
//             .map(|line| self.line_into_stl(&line).unwrap())
//             .flat_map(|tri| tri)
//             .collect();

//         Ok(tris)
//     }

//     fn line_into_stl(self, line: &Line) -> Result<Vec<stl_io::Triangle>> {
//         match line {
//             Line::SubFileReference(subfile) => self.subfile_into_stl(&subfile),
//             Line::Triangle(tri) => Ok(vec![self.tri_into_stl(&tri)]),
//             Line::Quadrilateral(quad) => Ok(self.quad_into_stl(&quad)),
//             _ => Ok(vec![]),
//         }
//     }

//     fn tri_into_stl(&self, tri: &Triangle) -> stl_io::Triangle {
//         stl_io::Triangle {
//             normal: tri.surface_normal().data,
//             vertices: [tri[0].data, tri[1].data, tri[2].data],
//         }
//     }

//     fn quad_into_stl(&self, quad: &Quadrilateral) -> Vec<stl_io::Triangle> {
//         quad.subdivide(false)
//             .iter()
//             .map(|tri| self.tri_into_stl(tri))
//             .collect()
//     }

//     fn subfile_into_stl(self, subfile: &SubFileReference) -> Result<Vec<stl_io::Triangle>> {
//         // todo: join
//         let file_name = subfile.file_name.clone();
//         let file = File::open(file_name)?;
//         let buf = BufReader::new(file);

//         let tris = Parser::new()
//             .parse(&buf)
//             .iter()
//             .flat_map(|line| self.line_into_stl(line))
//             .flatten()
//             .collect();

//         Ok(tris)
//     }
// }

// fn main() -> Result<()> {
//     let file = File::open("C:/Users/SpamB/Code/perl/ldraw2stl/ldraw/parts/1.dat")?;
//     let buf = BufReader::new(file);
//     let parser = Parser::new();
//     let lines = parser.parse(&buf);
//     let iter = Box::new(lines.into_iter());

//     let tris =
//         LDrawTransformer::new(iter, &"C:/Users/SpamB/Code/perl/ldraw2stl/ldraw").transform()?;
//     let mut stl_file = File::create("./r_test.stl")?;
//     stl_io::write_stl(&mut stl_file, tris.iter());

//     // stl_io::write_stl(writer: &mut W, mesh: I);

//     Ok(())
// }