use bin::app::App;
use eframe::egui;
use egui::*;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(1000., 550.)),
        ..Default::default()
    };

    let app = App::new(
        r#"LDA #$02
CMP #$01
BNE FOO
LDA #$01
STA $00
BRK

FOO:
    LDA #$01
    STA $01
    BRK
"#,
    );

    eframe::run_native("6502 Emulator", options, Box::new(|_| Box::new(app)))
}
