use crate::{application::{action::Action, sample::sample_menu::get_menu}, domain::display_watch};

pub mod application;
pub mod domain;
pub mod infra;

#[tokio::main]
async fn main() {
    let binding = get_menu();
    let mut stack: Vec<Action> = vec![Action::DisplayMenu(&binding)];

    loop {
        let is_main = stack.len() == 1;
        match stack.pop() {
            Some(Action::DisplayMenu(menu)) => {
                let next_action = menu.display(is_main).await;
                stack.push(Action::DisplayMenu(menu));
                stack.push(next_action)
            },
            Some(Action::DisplayWatch(watch_builder)) => {
                let watch = watch_builder.set_input().await;
                let next_action = display_watch(watch).await;
                stack.push(next_action);
            },
            Some(Action::Up) => {
                let _ = stack.pop();
            },
            Some(Action::Exit) => break,
            None => break,
        }
    }
}
