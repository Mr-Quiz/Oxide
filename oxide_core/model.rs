#[derive(Debug)]
pub struct Component {
  pub name: String,
  pub styles: Vec<StyleSheet>,
}

#[derive(Debug)]
pub struct StyleSheet {
  pub name: String,
  pub path: String,
}