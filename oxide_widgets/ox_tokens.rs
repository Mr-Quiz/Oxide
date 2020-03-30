pub enum OxWidgetType {
    Component,
    Template,
    Button,
    Layout,
    LayoutItem,
    Box,
    Menu,
    Input,
    Image,
    Attribute
}

impl OxWidgetType {
    pub fn parsing_value(&self) -> &str {
        match *self {
            OxWidgetType::Component => "<ox:component",
            _                       => "test"
        }
    }
}

pub enum OxTokens {
    BaseTagStart,
    BaseTag,
    TagEnd,
    Property,
    Comma,
    DirectiveStart,
    DirectiveEnd,
    ShortDirectiveEnd,
    EndOfStream,
}

impl OxTokens {
    pub fn parsing_value(&self) -> &str {
        match *self {
            OxTokens::BaseTagStart      => "<ox:[a-zA-Z]",
            OxTokens::BaseTag           => "ox:[a-zA-Z]",
            OxTokens::TagEnd            => ">",
            OxTokens::Property          => "w*=\"([a-zA-Z]+)\"",
            OxTokens::DirectiveStart    => "<[a-zA-Z]",
            OxTokens::Comma             => ",",
            OxTokens::DirectiveEnd      => "</",
            OxTokens::ShortDirectiveEnd => "/>",
            _ => ""
        }
    }
}