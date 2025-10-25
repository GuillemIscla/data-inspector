
use crate::{application::action::Action, domain::watch::Watch};
use dialoguer::{theme::ColorfulTheme, Select};
pub enum MenuItem {
    Menu(Menu),
    Watch(Box<dyn Watch>),
}

impl MenuItem {
    fn get_label(&self) -> String {
        match self {
            MenuItem::Menu(menu) => menu.label.clone(),
            MenuItem::Watch(watch) => watch.label(),
        }
    }

    fn to_action(&self) -> Action {
        match self {
            MenuItem::Menu(menu) => Action::DisplayMenu(menu),
            MenuItem::Watch(watch) => Action::DisplayWatch(watch),
        }
    }
}


pub struct Menu {
    title: String,
    pub label: String,
    items: Vec<MenuItem>
}

impl Menu {
    pub fn init(title: &str, label:&str, items:Vec<MenuItem>) -> Menu {
        let title = title.to_string();
        let label = label.to_string();
        Menu {
            title,
            label,
            items
        }
    }

    pub async fn display(&self, is_main:bool) -> Action {
        let theme = ColorfulTheme::default();
        let mut options = self.items.iter().enumerate()
                                    .map(|(index, menu_item)| format!("{}. {}", index + 1, menu_item.get_label()))
                                    .collect::<Vec<String>>();
        if is_main {
            options.push(format!("{}. Exit", options.len() + 1));
        }
        else {
            options.push(format!("{}. Up", options.len() + 1));
            options.push(format!("{}. Exit", options.len() + 1));
        }

        let selection = Select::with_theme(&theme)
            .with_prompt(self.title.clone())
            .items(&options)
            .default(0)
            .interact()
            .expect("Prompt failed");
        
        if selection == self.items.len() && !is_main {
            Action::Up
        }
        else if selection >= self.items.len() {
            Action::Exit
        }
        else {
            self.items[selection].to_action()
        }
    }
}