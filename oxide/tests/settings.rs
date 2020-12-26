#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::Settings;

    #[test]
    fn set_min_width() {
        let mut settings = Settings::new(800, 600);
        assert_eq!(0, settings.min_width);
        settings.set_min_width(200);
        assert_eq!(200, settings.min_width);
    }

    #[test]
    fn set_max_width() {
        let mut settings = Settings::new(800, 600);
        assert_eq!(0, settings.max_width);
        settings.set_max_width(200);
        assert_eq!(200, settings.max_width);
    }

    #[test]
    fn set_minimizable() {
        let mut settings = Settings::new(800, 600);
        assert_eq!(true, settings.minimizable);
        settings.set_minimizable(false);
        assert_eq!(false, settings.minimizable);
    }

    #[test]
    fn set_maximizable() {
        let mut settings = Settings::new(800, 600);
        assert_eq!(true, settings.maximizable);
        settings.set_maximizable(false);
        assert_eq!(false, settings.maximizable);
    }

    #[test]
    fn set_always_on_top() {
        let mut settings = Settings::new(800, 600);
        assert_eq!(false, settings.always_on_top);
        settings.set_always_on_top(true);
        assert_eq!(true, settings.always_on_top);
    }

    #[test]
    fn set_full_screen() {
        let mut settings = Settings::new(800, 600);
        assert_eq!(false, settings.full_screen);
        settings.set_full_screen(true);
        assert_eq!(true, settings.full_screen);
    }

    #[test]
    fn set_title() {
        let mut settings = Settings::new(800, 600);
        assert_eq!("App", settings.title);
        settings.set_title("My App".to_string());
        assert_eq!("My App", settings.title);
    }

    #[test]
    fn set_background_color() {
        let mut settings = Settings::new(800, 600);
        assert_eq!("#FFFFFF", settings.background_color);
        settings.set_background_color("#696969".to_string());
        assert_eq!("#696969", settings.background_color);
    }

    #[test]
    #[should_panic]
    fn set_background_color_panic_not_hex() {
        let mut settings = Settings::new(800, 600);
        assert_eq!("#FFFFFF", settings.background_color);
        settings.set_background_color("6969695".to_string());
    }

    #[test]
    #[should_panic]
    fn set_background_color_panic_not_seven_chars() {
        let mut settings = Settings::new(800, 600);
        assert_eq!("#FFFFFF", settings.background_color);
        settings.set_background_color("#69696".to_string());
    }

}