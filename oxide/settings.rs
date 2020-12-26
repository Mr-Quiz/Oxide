#[derive(Debug)]
pub struct Settings {
    pub height:             i16,
    pub width:              i16,
    pub min_width:          i16,
    pub max_width:          i16,
    pub minimizable:        bool,
    pub maximizable:        bool,
    pub always_on_top:      bool,
    pub full_screen:        bool,
    pub movable:            bool,
    pub title:              String,
    pub background_color:   String,
}

impl Settings {

    pub fn new(height: i16, width: i16) -> Settings {
        return Settings {
            height,
            width,
            min_width:          0,
            max_width:          0,
            minimizable:        true,
            maximizable:        true,
            always_on_top:      false,
            full_screen:        false,
            movable:            true,
            title:              "App".to_string(),
            background_color:   "#FFFFFF".to_string()
        }
    }

    pub fn set_min_width(&mut self, min_width: i16) {
        self.min_width = min_width;
    }
    pub fn set_max_width(&mut self, max_width: i16) {
        self.max_width = max_width;
    }
    pub fn set_minimizable(&mut self, minimizable: bool) {
        self.minimizable = minimizable;
    }
    pub fn set_maximizable(&mut self, maximizable: bool) {
        self.maximizable = maximizable;
    }
    pub fn set_always_on_top(&mut self, always_on_top: bool) {
        self.always_on_top = always_on_top;
    }
    pub fn set_full_screen(&mut self, full_screen: bool) {
        self.full_screen = full_screen;
    }
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn set_background_color(&mut self, background_color: String) {
        if background_color.contains("#") && background_color.len() == 7 {
            self.background_color = background_color;
        } else {
            panic!("Error: Background Color is wrong!");
        }
    }

}