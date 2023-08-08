use std::fs::File;
use std::io::Read;

#[derive(Clone)]
pub struct FileHandler {
    pub file_path: String,
    pub file_content: String,
}

impl FileHandler {
    pub fn read_file(path: &str) -> FileHandler {
        let mut file = File::open(path).expect("Failed to find file");
        let mut buffer = String::new();
        let file_ending = match path.split('.').last() {
            Some(ending) => ending,
            None => "unknown",
        };

        if file_ending != "nex" && file_ending != "nx" {
            panic!("Wrong file format. Current: {}, expected: nex", file_ending);
        }

        file.read_to_string(&mut buffer)
            .expect("Failed to read file");

        FileHandler { file_path: path.to_string(), file_content: buffer }
    }
}
