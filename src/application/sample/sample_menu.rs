
use crate::application::menu::{Menu, MenuItem};
use crate::domain::SampleWatchBuilder;

pub fn get_menu() -> Menu {
    let sub_menu = Menu::init(
        "Choose an option from the submenu", 
        "Submenu", 
        vec![MenuItem::Watch(Box::new(SampleWatchBuilder{}))]);
    let items: Vec<MenuItem> = vec![
        MenuItem::Menu(sub_menu),
        MenuItem::Watch(Box::new(SampleWatchBuilder{})),
        MenuItem::Watch(Box::new(SampleWatchBuilder{}))
    ];

    Menu::init(
        "Choose an option from the main menu", 
        "Main", 
        items)
}