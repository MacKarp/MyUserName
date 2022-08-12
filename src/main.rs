use chrono::prelude::*;
use copypasta::{ClipboardContext, ClipboardProvider};

fn main() {
    let mut clipboard_ctx = ClipboardContext::new().unwrap();

    let username = whoami::username();
    let current_time = Local::now().format("%d-%m-%Y %H:%M").to_string();
    let message = format!("{current_time} {username}");
    println!("{}", &message);
    clipboard_ctx.set_contents(message).unwrap();
}
