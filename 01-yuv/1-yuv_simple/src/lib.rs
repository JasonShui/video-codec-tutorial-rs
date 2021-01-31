// reference: https://blog.csdn.net/leixiaohua1020/article/details/50534150

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
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

// split frame of *yuv420file* to *y*, *u*, and *v* files
pub fn yuv420_split(
    yuv_path: &str,
    width: i32,
    height: i32,
    frame_num: i32,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    let path = Path::new(yuv_path);

    let mut output_path = PathBuf::from(output_path);

    let mut yuv420_file = File::open(path)?;
    let mut yuv_content = Vec::new();
    yuv420_file
        .take((frame_num * width * height * 3 / 2) as u64)
        .read_to_end(&mut yuv_content)?;

    let base_file_name = path.file_name().unwrap();
    output_path.push(base_file_name);
    let mut y_file = File::create(output_path.with_extension("y"))?;
    let mut u_file = File::create(output_path.with_extension("u"))?;
    let mut v_file = File::create(output_path.with_extension("v"))?;

    for frame in 0..frame_num {
        let base_pos = frame * width * height * 3 / 2;

        let y_content = &yuv_content[base_pos as usize..(base_pos + width * height) as usize];
        y_file.write_all(y_content);

        let u_content = &yuv_content
            [(base_pos + width * height) as usize..(base_pos + width * height * 5 / 4) as usize];
        u_file.write_all(u_content);

        let v_content = &yuv_content[(base_pos + width * height * 5 / 4) as usize
            ..(base_pos + width * height * 3 / 2) as usize];
        v_file.write_all(v_content);
    }

    Ok(())
}
