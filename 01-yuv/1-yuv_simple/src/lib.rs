// reference: https://blog.csdn.net/leixiaohua1020/article/details/50534150

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct MyError {
    msg: String,
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for MyError {}

impl MyError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

// split the first frame of *yuv420file* to *y*, *u*, and *v* files
pub fn yuv420_split(
    yuv_path: &str,
    width: i32,
    height: i32,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    let path = Path::new(yuv_path);
    let mut output_path = PathBuf::from(output_path);

    let mut yuv_content = Vec::new();
    let mut yuv420_file = File::open(path)?.take((width * height * 3 / 2) as u64);
    yuv420_file.read_to_end(&mut yuv_content)?;

    let base_file_name = path.file_name().unwrap();
    output_path.push(base_file_name);

    let y_content = &yuv_content[..(width * height) as usize];
    let mut y_file = File::create(output_path.with_extension("y"))?;
    y_file.write_all(y_content);

    let u_content = &yuv_content[(width * height) as usize..(width * height * 5 / 4) as usize];
    let mut u_file = File::create(output_path.with_extension("u"))?;
    u_file.write_all(u_content);

    let v_content =
        &yuv_content[(width * height * 5 / 4) as usize..(width * height * 3 / 2) as usize];
    let mut v_file = File::create(output_path.with_extension("v"))?;
    v_file.write_all(v_content);

    Ok(())
}
