use std::fs::File;
use std::io::{ Write, Read };
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let display = path.display();

    // Create file
    let mut file = match File::create(&path) {
        Err(reason) => panic!("couldn't create {}: {}", display, reason),
        Ok(file) => file,
    };

    // Write text to file
    let mut _input = String::new();

    println!("Enter your name: ");
    std::io::stdin().read_line(&mut _input).expect("input failed");
    
    match file.write_all(&mut _input.as_bytes()) {
        Ok(file) => file,
        Err(reason) => panic!("write file to {} failed: {}", display, reason)
    }

    // Read text from file
    let mut file_reader = match File::open(&path) {
        Err(reason) => panic!("Couldn't open file {}: {}", display, reason),
        Ok(file) => file
    };

    let mut str = String::new();
    file_reader.read_to_string(&mut str).unwrap();
    println!("Read from file: {}", str);

}
