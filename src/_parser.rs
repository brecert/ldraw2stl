use itertools::Itertools;
use vecmat::mat::Mat3x3;
use vecmat::vec::{Vec3, Vec4};

pub type Vertex = Vec3<f32>;
pub type Triangle = Vec3<Vertex>;
pub type Quadrilateral = Vec4<Vertex>;
pub type Matrix = Mat3x3<f32>;

#[derive(Debug, PartialEq)]
pub enum LineType<'a> {
  CommentMeta(Meta),
  SubFileReference(SubFileReference<'a>),
  Line,
  Triangle(Triangle),
  Quadrilateral(Quadrilateral),
  OptionalLine,
}

#[derive(Debug, PartialEq)]
pub struct SubFileReference<'a> {
  pub coordinates: Vertex,
  pub matrix: Matrix,
  pub file_name: &'a str,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct CommandArgs<'a> {
  pub color: &'a str,
  pub args: Vec<&'a str>,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Meta {
  Invert,
}

pub fn parse_command_meta(cmd: &str) -> Option<Meta> {
  if cmd.starts_with("BFC INVERTNEXT") {
    Some(Meta::Invert)
  } else {
    None
  }
}

pub fn parse_command_args(cmd: &str) -> CommandArgs {
  let mut args = cmd.split_ascii_whitespace();

  CommandArgs {
    color: args.next().unwrap(),
    args: args.skip(1).collect(),
  }
}

pub fn parse_command_vertexes(args: Vec<&str>) -> Vec<Vertex> {
  args
    .iter()
    .map(|v| v.parse::<f32>().unwrap())
    .tuples::<(_, _, _)>()
    .map(|t| Vertex::from(t.0, t.1, t.2))
    .collect()
}

pub fn parse_triangle_command(cmd: &str) -> Triangle {
  let args = parse_command_args(&cmd).args;
  let verts = parse_command_vertexes(args);

  Triangle::from(verts[0], verts[1], verts[2])
}

pub fn parse_sub_file_reference(cmd: &str) -> SubFileReference {
  let args = parse_command_args(&cmd).args;
  let verts: Vec<f32> = args
    .iter()
    .take(12)
    .map(|v| v.parse::<f32>().unwrap())
    .collect();
  println!("{:?}{:?}", verts, args.clone().last());
  SubFileReference {
    coordinates: Vertex::from(verts[0], verts[1], verts[2]),
    matrix: Matrix::from(
      verts[3], verts[4], verts[5], 
      verts[6], verts[7], verts[8], 
      verts[9], verts[10], verts[11],
    ),
    file_name: args.last().unwrap(),
  }
}

pub fn parse_quadrilateral_command(cmd: &str) -> Quadrilateral {
  let args = parse_command_args(&cmd).args;
  let verts = parse_command_vertexes(args);

  Quadrilateral::from(verts[0], verts[1], verts[2], verts[3])
}

pub fn parse_line(mut line: &str) -> Option<LineType> {
  line = line.trim();
  let line_type = line.chars().nth(0);
  match line_type {
    Some('0') => parse_command_meta(&line).map(LineType::CommentMeta),
    Some('1') => Some(LineType::SubFileReference(parse_sub_file_reference(&line))),
    Some('2') => Some(LineType::Line),
    Some('3') => Some(LineType::Triangle(parse_triangle_command(&line))),
    Some('4') => Some(LineType::Quadrilateral(parse_quadrilateral_command(&line))),
    Some('5') => Some(LineType::OptionalLine),
    _ => None,
  }
}

pub fn parse_input(input: &str) -> Vec<LineType> {
  input.lines().filter_map(parse_line).collect()
}
