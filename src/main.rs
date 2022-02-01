mod parser;
mod primatives;
use fncmd::fncmd;
use parser::*;
use result::Result;
use std::fs::{self, File};
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

pub mod result {
    pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
}

#[fncmd]
fn main(
    /// the path to the ldraw folder
    #[opt(short, long)]
    ldraw_path: PathBuf,

    /// the output directory for the converted files
    #[opt(short, long = "out-dir", default_value = "./")]
    output_dir: PathBuf,

    /// the ldraw files to convert
    files: Vec<PathBuf>,
) -> Result<()> {
    let cache = Cache::default();
    if ldraw_path.is_dir() {
        files.iter().try_for_each(|path| -> Result<_> {
            let parser = Parser::new(&ldraw_path, &cache, false);
            let triangles = parser.parse(path)?;
            let out_file_name = path
                .file_stem()
                .map(|s| output_dir.join(s).with_extension("stl"))
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
        Err(Error::new(ErrorKind::NotFound, "ldraw folder not found"))?
    }
}
