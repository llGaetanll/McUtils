use fastnbt::Value;
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;
use std::io::Write;

pub fn disp(file: File) {
    let mut decoder = GzDecoder::new(file);
    let mut buf = vec![];
    decoder.read_to_end(&mut buf).unwrap();

    let val: Value = fastnbt::de::from_bytes(buf.as_slice()).unwrap();

    let string = format!("{:#?}", val);

    // write data to file
    let mut output = File::create("output.txt").unwrap();
    write!(output, "{}", &string).expect("Could not write output file");
}
