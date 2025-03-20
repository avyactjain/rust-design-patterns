struct FileGuard {
    filename: String,
}

impl FileGuard {
    fn new(filename: &str) -> Self {
        println!("Opening file: {}", filename);
        Self {
            filename: filename.to_string(),
        }
    }
}

impl Drop for FileGuard {
    fn drop(&mut self) {
        println!("Closing file: {}", self.filename);
    }
}

fn main() {
    {
        let _file = FileGuard::new("data.txt");
        println!("Using the file...");
    } // FileGuard goes out of scope, "Closing file" is printed

    println!("File is safely closed!");
}
