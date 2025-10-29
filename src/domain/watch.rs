use anyhow::Result;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use async_trait::async_trait;
use colored::*;
use dyn_clone::DynClone;
use uuid::Uuid;
use std::io::Write;
use tokio::select;
use tokio::time::{self, Duration};

use crate::{
    application::action::Action,
    infra::string_constants::*
};

#[async_trait]
pub trait WatchBuilder: DynClone  {
    fn label(&self) -> String;
    async fn set_input(&self) -> Box<dyn Watch>;
}
dyn_clone::clone_trait_object!(WatchBuilder);

#[async_trait]
pub trait Watch {
    fn title(&self) -> &str;
    fn label(&self) -> &str;
    async fn watch(&self, since_ms:u32) -> Result<String>;
}


pub async fn display_watch(watch: Box<dyn Watch>) -> Action<'static> {
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
                match watch.watch(100000000).await {
                    Ok(result) => println!("{}", result),
                    Err(error) => println!("An error occurred: '{}'", error),
                }
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


pub async fn prompt_bucket_uuid(input_label:&str) -> tokio::io::Result<Option<Uuid>> {
    let mut stdout = tokio::io::stdout();
    let mut reader = BufReader::new(tokio::io::stdin());
    let mut line = String::new();

    loop {
        let prompt = format!("Enter the uuid for the {}. If you want no value, press enter> ", input_label).yellow().bold();
        stdout
            .write_all(prompt.to_string().as_bytes())
            .await?;
        stdout.flush().await?;

        line.clear();
        let n = reader.read_line(&mut line).await?;
        if n == 0 {
            return Ok(None); // EOF
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            let msg = format!("Accepting 'empty data' as input").blue().bold();
            println!("{}", msg);
            return Ok(None);
        }

        match Uuid::parse_str(trimmed) {
            Ok(uuid) => {
                let msg = format!("Accepting '{}' as input", trimmed).blue().bold();
                println!("{}", msg);
                return Ok(Some(uuid));
            },
            Err(_) => {
                let error = format!("You entered '{}' which is not a Uuid> ", trimmed).red().bold();
                println!("{}", error);
            }
        }
    }
}