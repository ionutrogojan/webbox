use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use webbrowser;

fn main() {
    let content = "[LINKS]\nhttps://github.com/ionutrogojan/webbox";
    let path = Path::new("config.wb");
    if path.exists() {
        println!("File exists.");
        let config = fs::read_to_string("config.wb").unwrap();
        for link in config.lines() {
            if link == "[LINKS]" {
                continue;
            }
            if webbrowser::open(link).is_ok() {
                println!("Success opening {}", link);
            } else {
                panic!("Error opening {}", link);
            }
        }
    } else {
        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(e) => panic!("Error creating file. {}", e)
        };
        match file.write_all(content.as_bytes()) {
            Ok(_) => println!("File created."),
            Err(e) => panic!("Error writing file. {}", e)
        };
    }
}
