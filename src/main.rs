use std::fs;

use nfd;

struct CSVReader {
    comma: char,
    content: String,
}

impl CSVReader {
    fn read(&self) -> Vec<Vec<String>> {
        let lines = self.content.split('\n');
        let mut result: Vec<Vec<String>> = vec![];
        for line in lines {
            let mut parsed_line: Vec<String> = vec![];
            for field in line.split(self.comma) {
                parsed_line.push(String::from(field));
            }
            result.push(parsed_line)
        }
        return result
    }
}

fn main() {
    println!("Hello, world!");
    let files = show_open_file_dialog();
    for file in files {
        let reader = CSVReader{
            comma: ';',
            content: file,
        };
        println!("{:?}", reader.read());
    }
}

fn show_open_file_dialog() -> Vec<String> {
    let res = nfd::open_file_dialog(Some("csv"), None).expect("couldn't open a file dialog");
    match res {
        nfd::Response::Okay(file_path) => {
            let content = must_read_file(file_path);
            if !content.eq("") {
                return vec![content];
            }
            vec![]
        }
        nfd::Response::OkayMultiple(files) => {
            let mut v: Vec<String> = vec![];
            for file in files {
                let content = must_read_file(file);
                if !content.eq("") {
                    v.push(content);
                }
            }
            v
        }
        _ => vec![]
    }
}

fn must_read_file(path: String) -> String {
    match fs::read_to_string(path) {
        Ok(content) => content,
        _ => String::from("")
    }
}