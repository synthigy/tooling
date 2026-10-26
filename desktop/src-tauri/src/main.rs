#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if std::env::args().any(|a| a == "--mcp") {
        synthigy_desktop_lib::mcp_stdio()
    } else {
        synthigy_desktop_lib::run()
    }
}
