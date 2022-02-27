#[derive(Debug)]
pub struct OxWidget {
  pub name: String,
  pub children: Vec<OxWidget>,
}

pub struct OxWidgetProperty {
  name: String,
  // Value can be string
  //    value:  Option,
}