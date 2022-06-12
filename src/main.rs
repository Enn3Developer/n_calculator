use n_calculator::app::App;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "N Calculator",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
}
