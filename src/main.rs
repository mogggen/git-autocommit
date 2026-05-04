use chrono;
use notify::{Event, RecursiveMode, Result, Watcher};
use std::{path::Path, sync::mpsc};
// use std::env;
use std::process::Command;
use std::time::SystemTime;

fn call_terminal(args: Vec<&str>) {
    Command::new(if cfg!(target_os = "windows") { "powershell" } else { "sh" }).args(args) .output().expect("Should be able to ls here").stdout;
}

fn main() -> Result<()> {
    call_terminal(vec!["git", "init"]); 
    call_terminal(vec!["git", "config", "user.name", "\"Jane Doe\""]); 
    call_terminal(vec!["git", "config", "user.email", "\"jane.doe@example.com\""]);

    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;
    let path = std::env::current_dir().expect("Should get the current directory");
    // let args: Vec<_> = env::args().collect();
    // let path = if args.len() > 1 { Path::new(&args[1]) };
    // println!("Watching in directory {}", path.display());

    watcher.watch(&path, RecursiveMode::Recursive)?;
    for res in rx {
        match res {
            Ok(event) => {
                println!("event: {:?}", event);
                call_terminal(vec!["git", "add", "*"]);
                call_terminal(vec!["git", "commit", "-m", &*format!("{:?}", chrono::offset::Local::now()) 
                ]);
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
    Ok(())
}