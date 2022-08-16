use chrono::prelude::*;
use copypasta::{ClipboardContext, ClipboardProvider};

fn main() {
    let username = whoami::username();
    let current_time = Local::now().format("%d-%m-%Y %H:%M").to_string();
    let message = format!("{current_time} {username} \n");

    let mut clipboard_ctx = ClipboardContext::new().unwrap();
    clipboard_ctx.set_contents(message).unwrap();

    // without getting from clip it is not working on linux - tested on Gnome Fedora 36
    clipboard_ctx.get_contents().unwrap();
}
