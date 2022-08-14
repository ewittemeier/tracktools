struct GbxFile {
    header: Vec<u8>,
    references: Vec<u8>,
    body: Vec<u8>,
}