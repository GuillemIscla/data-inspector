
use crate::application::menu::{Menu, MenuItem};
use crate::domain::sample::sample_watch::SampleWatch;

pub fn get_menu() -> Menu {
    let sub_menu = Menu::init(
        "Choose an option from the submenu", 
        "Submenu", 
        vec![MenuItem::Watch(Box::new(SampleWatch {}))]);
    let items: Vec<MenuItem> = vec![
        MenuItem::Menu(sub_menu),
        MenuItem::Watch(Box::new(SampleWatch {})),
        MenuItem::Watch(Box::new(SampleWatch {}))
    ];

    Menu::init(
        "Choose an option from the main menu", 
        "Main", 
        items)
}