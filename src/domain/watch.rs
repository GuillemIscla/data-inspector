use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::select;
use tokio::time::{self, Duration};
use std::io::Write;
use colored::*;

use crate::application::action::Action;

pub trait Watch {
    fn title(&self) -> String;
    fn label(&self) -> String;
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
                print!("\r\x1b[2K");
                let msg = format!("Refreshing watch: {}. To finish enter 'Up' or 'Exit'", watch.title());
                println!("{}", &msg.green().bold());
                watch.watch();
                print!("cmd> ");
                std::io::stdout().flush().unwrap();
            }
            line = lines.next_line() => {
                match line {
                    Ok(Some(line)) => {
                        let line = line.trim().to_string();
                        if line.eq_ignore_ascii_case("exit") {
                            action = Action::Exit;
                            break;
                        } else if line.eq_ignore_ascii_case("up") {
                            action = Action::Up;
                            break;
                        }
                        else {
                            print!("cmd> ");
                        }
                        
                        std::io::stdout().flush().unwrap();
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