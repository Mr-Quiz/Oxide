#[cfg(test)]
mod tests {
    use super::*;
    use crate::oxide::Oxide;

    #[test]
    fn create_oxide_app() {
        let app = Oxide::new(300, 400);
        assert_eq!(400  , app.settings.width);
        assert_eq!(300  , app.settings.height);
        assert_eq!(0    , app.settings.min_width);
        assert_eq!(0    , app.settings.max_width);
        assert_eq!(false, app.settings.always_on_top);
        assert_eq!(false, app.settings.full_screen);
        assert_eq!(true , app.settings.minimizable);
        assert_eq!(true , app.settings.maximizable);
        assert_eq!(true , app.settings.movable);
        assert_eq!("App", app.settings.title);
        assert_eq!("#FFFFFF", app.settings.background_color);
    }

}