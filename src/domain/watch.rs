use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::select;
use tokio::time::{self, Duration};
use std::io::Write;
use colored::*;

use crate::{
    application::action::Action,
    infra::string_constants::*
};

pub trait Watch {
    fn title(&self) -> &str;
    fn label(&self) -> &str;
    fn watch(&self);
}


pub async fn display_watch(watch: &Box<dyn Watch>) -> Action {
    let mut ticker = time::interval(Duration::from_secs(5));
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();
    let mut action: Action = Action::Up;
    loop {
        select! {
            _ = ticker.tick() => {
                print!("{}", FLUSH_LINE);
                let msg = format!("Refreshing watch: {}. To finish enter 'Up' or 'Exit'", watch.title());
                println!("{}", &msg.green().bold());
                watch.watch();
                print!("{}", CMD_PROMPT);
                std::io::stdout().flush().expect("Failed flushing the prompts");
            }
            line = lines.next_line() => {
                match line {
                    Ok(Some(line)) => {
                        let line = line.trim();
                        if line.eq_ignore_ascii_case(EXIT) {
                            action = Action::Exit;
                            break;
                        } else if line.eq_ignore_ascii_case(UP) {
                            action = Action::Up;
                            break;
                        }
                        else {
                            print!("{}", CMD_PROMPT);
                        }
                        
                        std::io::stdout().flush().expect("Failed flushing the prompts");
                    }
                    Ok(None) => break,
                    Err(e) => {
                        eprintln!("stdin error: {e}");
                        break;
                    }
                }
            }
        }
    }
    action
}