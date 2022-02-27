#![allow(dead_code)]
#![deny(unused_must_use)]

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OxDomId {
  id: usize,
  parent: Option<(Box<OxDomId>, )>,
}

pub struct OxNodeProperty {
  name: String,
  value: String,
}

pub struct OxNode {
  name: String,
  children: Vec<OxNode>,
//    properties: HashMap<String, Property>,
}

pub struct OxDOM {
  main: OxNode,
}