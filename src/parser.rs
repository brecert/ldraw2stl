use crate::primatives::*;
use crate::result::Result;

use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub type Cache = HashMap<PathBuf, Vec<Triangle>>;

pub struct Parser<'a> {
    pub subpart_paths: Vec<PathBuf>,
    pub ldraw_path: &'a Path,
    inverted: bool,
    invert_next: bool,
    triangles: Vec<Triangle>,
    cache: &'a Cache,
}

impl<'a> Parser<'a> {
    pub fn new<P: AsRef<Path> + ?Sized>(
        path: &'a P,
        cache: &'a Cache,
        inverted: bool,
    ) -> Parser<'a> {
        Parser {
            ldraw_path: path.as_ref(),
            inverted: inverted,
            invert_next: false,
            triangles: vec![],
            subpart_paths: ["p", "p/48", "parts", "models"]
                .iter()
                .map(|t| PathBuf::from(path.as_ref()).join(t))
                .collect(),
            cache: cache,
        }
    }

    pub fn parse<P: AsRef<Path> + ?Sized>(self, path: &'a P) -> Result<Vec<Triangle>> {
        let file = File::open(path)?;
        let mut buf = BufReader::new(file);
        self.read(&mut buf)
    }

    pub fn read(mut self, buf: &mut dyn BufRead) -> Result<Vec<Triangle>> {
        buf.lines().try_for_each(|line| self.parse_line(&line?))?;
        Ok(self.triangles)
    }

    fn parse_line(&mut self, line: &str) -> Result<()> {
        let mut items = line.trim().split_ascii_whitespace();
        if let Some(line_type) = items.next() {
            match line_type {
                "0" => self.parse_comment_or_meta(items),
                "1" => self.parse_sub_file_reference(items)?,
                "3" => self.parse_triangle_command(items)?,
                "4" => self.parse_quadrilateral_command(items)?,
                _ => {}
            };
        };
        Ok(())
    }

    fn parse_comment_or_meta<'b, I: IntoIterator<Item = &'b str>>(&mut self, items: I) {
        let mut items = items.into_iter();
        if let Some("BFC") = items.next() {
            self.handle_bfc_command(items)
        }
    }

    fn handle_bfc_command<'b, I: IntoIterator<Item = &'b str>>(&mut self, items: I) {
        let mut items = items.into_iter();

        if let Some(cmd_name) = items.next() {
            match cmd_name {
                "INVERT_NEXT" => self.invert_next = true,
                _ => {}
            }
        }
    }

    fn parse_sub_file_reference<'b, I: IntoIterator<Item = &'b str> + Clone>(
        &mut self,
        items: I,
    ) -> Result<()> {
        let file_name = items
            .clone()
            .into_iter()
            .last()
            .ok_or("malformed sub_file_reference: expected a file name.")?;

        let subpart_path = self
            .subpart_paths
            .iter()
            .map(|path| path.join(file_name))
            .find(|path| path.is_file())
            .ok_or("No valid subpart path found for filename")?;

        let items: [f32; 12] = items
            .into_iter()
            .skip(1)
            .take(12)
            .map(|s| s.parse::<f32>().map(|v| Ok(v))?)
            .collect::<Result<Vec<f32>>>()?
            .as_slice()
            .try_into()?;

        let [x, y, z, a, b, c, d, e, f, g, h, i] = items;
        let mat = [a, b, c, x, d, e, f, y, g, h, i, z, 0., 0., 0., 1.];

        let triangles = if self.cache.contains_key(&subpart_path) {
            self.cache.get(&subpart_path).unwrap().clone()
        } else {
            let invert = mat.determinant() < 1.0 || self.invert_next != self.inverted;
            self.invert_next = false;
            let subparser = Parser::new(self.ldraw_path, self.cache, invert);
            let parsed = subparser.parse(&subpart_path)?;
            parsed
        };

        for triangle in triangles {
            let tri = Triangle([
                mul_mat_vert(mat, triangle.0[0]),
                mul_mat_vert(mat, triangle.0[1]),
                mul_mat_vert(mat, triangle.0[2]),
            ]);
            self.triangles.push(tri)
        }

        Ok(())
    }

    fn parse_triangle_command<'b, I: IntoIterator<Item = &'b str> + Clone>(
        &mut self,
        items: I,
    ) -> Result<()> {
        let points = items
            .into_iter()
            .skip(1)
            .take(9)
            .map(|s| s.parse::<f32>().map(|v| Ok(v))?)
            .collect::<Result<Vec<f32>>>()?;

        if self.inverted {
            self.triangles.push(Triangle([
                Vertex::new(points[0], points[1], points[2]),
                Vertex::new(points[6], points[7], points[8]),
                Vertex::new(points[3], points[4], points[5]),
            ]))
        } else {
            self.triangles.push(Triangle([
                Vertex::new(points[0], points[1], points[2]),
                Vertex::new(points[3], points[4], points[5]),
                Vertex::new(points[6], points[7], points[8]),
            ]))
        }

        Ok(())
    }

    fn parse_quadrilateral_command<'b, I: IntoIterator<Item = &'b str> + Clone>(
        &mut self,
        items: I,
    ) -> Result<()> {
        let points = items
            .into_iter()
            .skip(1)
            .take(12)
            .map(|s| s.parse::<f32>().map(|v| Ok(v))?)
            .collect::<Result<Vec<f32>>>()?;

        let row1 = Vertex::new(points[0], points[1], points[2]);
        let row2 = Vertex::new(points[3], points[4], points[5]);
        let row3 = Vertex::new(points[6], points[7], points[8]);
        let row4 = Vertex::new(points[9], points[10], points[11]);

        if self.inverted {
            self.triangles.push(Triangle([row1, row3, row2]));
            self.triangles.push(Triangle([row3, row1, row4]));
        } else {
            self.triangles.push(Triangle([row1, row2, row3]));
            self.triangles.push(Triangle([row3, row4, row1]));
        }

        Ok(())
    }
}
