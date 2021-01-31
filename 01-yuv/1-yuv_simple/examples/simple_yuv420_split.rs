use yuv_simple::yuv420_split;

fn main() {
    yuv420_split(
        "./test-files/waterfall_cif.yuv",
        352,
        288,
        "./01-yuv/1-yuv_simple/examples",
    )
    .unwrap();
}
