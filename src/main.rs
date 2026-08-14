mod cli;
mod config;
mod sys;
mod ui;

fn main() {
    ui::theme::setup_ch_theme();

    if let Err(err) = cli::run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}