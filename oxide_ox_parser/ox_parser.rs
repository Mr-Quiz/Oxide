#![allow(dead_code)]
#![deny(unused_must_use)]

use std::{
    fs::File,
    io::BufRead
};
use regex::Regex;
use oxide_widgets::ox_widget::OxWidget;

pub fn parse_ox_structure() {

    let ox_file = File::open("./examples/ox/main.ox").unwrap();
    let mut ox_structure: Vec<OxWidget>             = vec![];
    let mut ox_structure_stringified: Vec<String>   = vec![];
    let mut actual_tag_index: i32                   = 0;

    for(_index, line) in std::io::BufReader::new(ox_file).lines().enumerate() {
        let line = line.unwrap().to_string().trim_start().to_string();
        let ox_widget: OxWidget;

        match line.as_ref() {

            line if Regex::new(oxide_widgets::ox_tokens::OxTokens::BaseTagStart.parsing_value()).unwrap().is_match(line) => {

                if Regex::new(oxide_widgets::ox_tokens::OxTokens::TagEnd.parsing_value()).unwrap().is_match(line) {

                    let mut fixed_line = "".to_owned();
                    for (_index, iter) in line.replace("<", "").replace(">", "").split_whitespace().enumerate()  {

                        // TODO: Change
                        match iter.as_ref() {
                            iter if Regex::new(oxide_widgets::ox_tokens::OxTokens::Property.parsing_value()).unwrap().is_match(iter) => {
                                fixed_line += &iter.to_string();
                                if line.replace("<", "").replace(">", "").split_whitespace().enumerate().count() - 1 == _index {
                                    fixed_line += "]";
                                } else {
                                    fixed_line += ",";
                                }
                            },
                            iter if Regex::new(oxide_widgets::ox_tokens::OxTokens::BaseTag.parsing_value()).unwrap().is_match(iter) => {
                                // TODO: Change
                                fixed_line += &iter;
                                fixed_line += "|";
                                fixed_line += &actual_tag_index.to_string();
                                if line.replace("<", "").replace(">", "").split_whitespace().enumerate().count() > 1 {
                                    fixed_line += "[";
                                }
                            },
                            _ => {
                                println!("ParseError at [row,col]:[{},{}]: {:?}", actual_tag_index, _index, iter.to_string());
                            }
                        }

                    }
                    ox_structure_stringified.push(fixed_line.to_string());
                    actual_tag_index += 1;

                } else {

                }

            },
            line if Regex::new(oxide_widgets::ox_tokens::OxTokens::DirectiveEnd.parsing_value()).unwrap().is_match(line) => {

                let fixed_line                  = &line.replace('<', "").replace('>', "").replace('/', "").split_whitespace().next().unwrap().to_string();
                let deduplicated_structure      = filter_structure_without_duplicates(ox_structure_stringified.clone().into_iter().filter(|x| x.contains(fixed_line)).collect::<Vec<String>>());
                let deduplicated_structure_size = deduplicated_structure.len();
                ox_structure_stringified.push(deduplicated_structure[deduplicated_structure_size - 1].to_string().split('[').next().unwrap().to_string());

            },
            line if Regex::new("").unwrap().is_match(line) => {

            },
            _ => {
                println!("{:?}", line);
            }
        }

    }

//    println!("{:?}", ox_structure);
    println!("{:?}", ox_structure_stringified);

}

pub fn filter_structure_without_duplicates(ox_structure: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = ox_structure;
    let mut previous_node: String = "".to_string();
    let mut nodes_to_remove: Vec<usize> = vec![];
    for i in 0..result.len() {
        if result[i] == previous_node {
            nodes_to_remove.push(i);
            nodes_to_remove.push(i-1);
        }
        previous_node = result[i].to_string();
    }
    for index in nodes_to_remove {
        result.remove(index);
    }
    return result;
}

pub fn parse_ox_line(line: &str) {

}
