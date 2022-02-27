use std::{
  fs::File,
  io::BufRead,
};

pub trait FileParser {
  fn parse_file(&mut self);
  fn parse_authors(line: String) -> Vec<String>;
  fn replace_line(line: String) -> String;
}

#[derive(Debug)]
pub struct CargoFile {
  pub name: String,
  pub version: String,
  pub authors: Vec<String>,
  pub edition: String,
}

impl FileParser for CargoFile {
  fn parse_file(&mut self) {
    let file = File::open("./Cargo.toml").unwrap();
    for (_index, line) in std::io::BufReader::new(file).lines().enumerate() {
      if line.as_ref().unwrap().to_string().contains("name") {
        self.name = CargoFile::replace_line(line.as_ref().unwrap().to_string());
      } else if line.as_ref().unwrap().to_string().contains("version") {
        self.version = CargoFile::replace_line(line.as_ref().unwrap().to_string());
      } else if line.as_ref().unwrap().to_string().contains("edition") {
        self.edition = CargoFile::replace_line(line.as_ref().unwrap().to_string());
      } else if line.as_ref().unwrap().to_string().contains("authors") {
        self.authors = CargoFile::parse_authors(line.as_ref().unwrap().to_string());
      }
    }
  }
  fn parse_authors(line: String) -> Vec<String> {
    let authors = line.split(" = ").last().unwrap()
      .replace("\"", "").replace("[", "").replace("]", "")
      .split(",").map(|s| s.to_string()).collect::<Vec<String>>();
    return authors;
  }
  fn replace_line(line: String) -> String {
    return line.split_whitespace().last().unwrap().to_string().replace("\"", "");
  }
}

impl CargoFile {
  pub fn new() -> CargoFile {
    let mut file = CargoFile {
      name: "".to_string(),
      authors: vec![],
      edition: "2018".to_string(),
      version: "0.0.1".to_string(),
    };
    file.parse_file();
    return file;
  }
}