use crate::{application::menu::Menu, domain::WatchBuilder};

pub enum Action<'a> { 
    DisplayMenu(&'a Menu), 
    DisplayWatch(Box<dyn WatchBuilder>), 
    Up, 
    Exit, 
}