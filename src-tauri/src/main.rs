//! legado_tauri 入口
//! 模式: CLI (booksource-eval/booksource-test) 或 Tauri App

use legado_tauri_lib::{run_cli, run_app};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 && args[1] == "cli" {
        eprintln!("[BOOT][Rust] CLI ");
        if let Err(e) = run_cli(&args[2..]) {
            eprintln!("legado_tauri cli error: {e}");
            std::process::exit(1);
        }
    } else {
        run_app();
    }
}
