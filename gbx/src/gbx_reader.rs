use std::fs;
use std::io::Read;

struct GbxReader {
    filepath: String,
    content: Vec<u8>,
}

impl GbxReader {
    fn new(filepath: String) -> GbxReader {
        let raw_file = fs::read(&filepath).expect("Can not read from file");

        GbxReader {
            filepath,
            content: raw_file,
        }
    }

}