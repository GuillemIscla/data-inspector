use crate::{application::{action::Action, sample::sample_menu::get_menu}, domain::display_watch};

pub mod application;
pub mod domain;
pub mod infra;

#[tokio::main]
async fn main() {
    let binding = get_menu();
    let mut stack: Vec<Action> = vec![Action::DisplayMenu(&binding)];

    loop {
        match stack.last() {
            Some(Action::DisplayMenu(menu)) => {
                let is_main = stack.len() == 1;
                stack.push(menu.display(is_main).await)
            },
            Some(Action::DisplayWatch(watch)) => stack.push(display_watch(watch).await),
            Some(Action::Up) => {
                stack.pop();
                stack.pop();
            },
            Some(Action::Exit) => break,
            None => break,
        }
    }
}
