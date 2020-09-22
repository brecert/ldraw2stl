#![feature(try_find)]

#[macro_use]
mod result;
mod ldraw;
mod stl;
mod traits;

use result::*;

use std::fs::File;
use stl_io::write_stl;

fn main() -> Result<()> {
    let parser = stl::Parser::new("C:/Users/SpamB/Code/perl/ldraw2stl/ldraw");
    let triangles = parser.parse("C:/Users/SpamB/Code/perl/ldraw2stl/ldraw/parts/1.dat")?;
    // println!("{:#?}", &triangles);
    let mut file = File::create("test.rs.stl")?;
    write_stl(&mut file, triangles.iter())?;
    Ok(())
}
