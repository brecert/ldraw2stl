mod parser;
mod primatives;
use argopt::cmd;
use parser::*;
use result::Result;
use std::fs::{self, File};
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

pub mod result {
    pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
}

#[cmd]
fn main(files: Vec<PathBuf>) -> Result<()> {
    let cache = Cache::default();
    let ldraw_path = Path::new("ldraw");
    if ldraw_path.is_dir() {
        files.iter().try_for_each(|path| -> Result<_> {
            let parser = Parser::new(&ldraw_path, &cache, false);
            let triangles = parser.parse(path)?;
            let out_file_name = path
                .file_stem()
                .map(|s| Path::new("stls").join(s).with_extension("stl"))
                .ok_or("Invalid output filename")?;
            fs::create_dir_all(&out_file_name.parent().ok_or(".")?)?;
            let mut file = File::create(out_file_name)?;
            let triangles: Vec<_> = triangles
                .into_iter()
                .map(Into::<stl_io::Triangle>::into)
                .collect();
            stl_io::write_stl(&mut file, triangles.iter())?;
            Ok(())
        })
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            "ldraw folder not found in current directory",
        ))?
    }
}
