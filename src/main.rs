use chrono::prelude::*;
use copypasta::{ClipboardContext, ClipboardProvider};
use std::process::Command;

fn main() {
    let cmd = Command::new("cmd")
        .args(["/C", "whoami.exe /UPN"])
        .output()
        .expect("failed to execute process")
        .stdout;
    let full_user_name = String::from_utf8(cmd).unwrap();

    let username: Vec<&str> = full_user_name.split('@').collect();
    let username = *username.first().unwrap();

    let current_time = Local::now().format("%d-%m-%Y %H:%M").to_string();
    let message = format!("{current_time} {username} \n");

    let mut clipboard_ctx = ClipboardContext::new().unwrap();
    clipboard_ctx.set_contents(message).unwrap();
}
