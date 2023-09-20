use std::sync::atomic::AtomicBool;

use eframe::egui;
use emulator::DebugKind;

pub mod app;
pub mod ui;

/// (time, message, kind)
pub static mut DEBUG_OUTPUT: (String, String, DebugKind) =
    (String::new(), String::new(), DebugKind::Info);
pub static mut DEBUG_UPDATE: bool = false;
pub static IS_RUNNING: AtomicBool = AtomicBool::new(false);

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui, app: &mut app::App);
}
