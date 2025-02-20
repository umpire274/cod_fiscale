use log::info;

extern crate env_logger;
extern crate log;

mod utils;
mod console;
mod gui;

fn main() {
    // enable logging, since log defaults to silent
    //std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("Application starting...");

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Codice Fiscale Generator",
        native_options,
        Box::new(|_| Box::new(gui::CodiceFiscaleApp::default())),
    )
    .expect("Errore nell'avvio dell'interfaccia");

    info!("Application ended.");
}
