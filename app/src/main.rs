#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod shell;

fn main() {
    env_logger::init();

    let _ = shell::start_interactive_shell();
}
