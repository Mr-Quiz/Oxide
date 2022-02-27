#![allow(dead_code)]
#![deny(unused_must_use)]

use std::{
  fs::File,
  io::BufRead,
};
use regex::Regex;
use oxide_widgets::ox_widget::OxWidget;

pub fn parse_ox_structure() {
  let ox_file = File::open("../examples/ox/main.ox").unwrap();
  let mut ox_structure: Vec<OxWidget> = vec![];
  let mut ox_structure_stringified: Vec<String> = vec![];
  let mut actual_tag_index: i32 = 0;

  for (_index, line) in std::io::BufReader::new(ox_file).lines().enumerate() {
    let line = line.unwrap().to_string().trim_start().to_string();
    match line.as_ref() {
      line if Regex::new(oxide_widgets::ox_tokens::OxTokens::BaseTagStart.parsing_value()).unwrap().is_match(line) => {
        if Regex::new(oxide_widgets::ox_tokens::OxTokens::TagEnd.parsing_value()).unwrap().is_match(line) {
          let mut fixed_line = "".to_owned();
          for (_index, inner_entry) in line.replace("<", "").replace(">", "").split_whitespace().enumerate() {
            // TODO: Change
            match inner_entry.as_ref() {
              inner_entry if Regex::new(oxide_widgets::ox_tokens::OxTokens::Property.parsing_value()).unwrap().is_match(inner_entry) => {
                fixed_line += &inner_entry.to_string();
                if line.replace("<", "").replace(">", "").split_whitespace().enumerate().count() - 1 == _index {
                  fixed_line += "]";
                } else {
                  fixed_line += ",";
                }
              }
              inner_entry if Regex::new(oxide_widgets::ox_tokens::OxTokens::BaseTag.parsing_value()).unwrap().is_match(inner_entry) => {
                // TODO: Change
                fixed_line += &format!("{}|{}", &inner_entry, &actual_tag_index.to_string());
                if line.replace("<", "").replace(">", "").split_whitespace().enumerate().count() > 1 {
                  fixed_line += "[";
                }
              }
              _ => {
                println!("ParseError at [row,col]:[{},{}]: {:?}", actual_tag_index, _index, inner_entry.to_string());
              }
            }
          }
          ox_structure_stringified.push(fixed_line.to_string());
          actual_tag_index += 1;
        } else {}
      }
      line if Regex::new(oxide_widgets::ox_tokens::OxTokens::DirectiveEnd.parsing_value()).unwrap().is_match(line) => {
        let fixed_line = &line.replace('<', "").replace('>', "").replace('/', "").split_whitespace().next().unwrap().to_string();
        let de_duplicated_structure = filter_structure_without_duplicates(ox_structure_stringified.clone().into_iter().filter(|line| line.contains(fixed_line)).collect::<Vec<String>>());
        let de_duplicated_structure_size = de_duplicated_structure.len();
        ox_structure_stringified.push(de_duplicated_structure[de_duplicated_structure_size - 1].to_string().split('[').next().unwrap().to_string());
      }
      _ => {
//                println!("{:?}", line);
      }
    }
  }

  for node in ox_structure_stringified.clone() {
    let mut node_split = node.split("[");
    node_split.next();
    // println!("Node: {:?}", node_split.next());
  }

  // println!("{:?}", ox_structure);
  println!("{:?}", ox_structure_stringified);
}

pub fn filter_structure_without_duplicates(ox_structure: Vec<String>) -> Vec<String> {
  let mut result: Vec<String> = ox_structure;
  let mut previous_node: String = "".to_string();
  let mut nodes_to_remove: Vec<usize> = vec![];

  for i in 0..result.len() {
    if (result[i] == previous_node || previous_node.contains(&result[i])) && previous_node != "" {
      println!("{:?} == {:?}", result[i], previous_node);
      nodes_to_remove.push(i);
      nodes_to_remove.push(i - 1);
    }
    previous_node = result[i].to_string();
  }
  for index in nodes_to_remove {
    result.remove(index);
  }
  return result;
}

pub fn parse_ox_line(line: &str) {}

pub fn match_ox_line(line: &str) {
  let mut result: String;
}