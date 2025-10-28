use crate::{application::menu::Menu, domain::Watch};

pub enum Action<'a> {
    DisplayMenu(&'a Menu),
    DisplayWatch(&'a Box<dyn Watch>),
    Up,
    Exit,
}
