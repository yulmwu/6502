use crate::{ui::*, View, DEBUG_OUTPUT, DEBUG_UPDATE, IS_RUNNING};
use assembler::Assembler;
use chrono::prelude::*;
use eframe::egui::*;
use emulator::{memory::Memory, Cpu6502, CpuDebugger, DebugKind, Debugger};
use std::{sync::atomic::Ordering, thread, time::Duration};

#[derive(Default)]
pub struct AppDebugger;

impl Debugger for AppDebugger {
    fn debug(&mut self, msg: &str, kind: DebugKind) {
        unsafe {
            DEBUG_OUTPUT.push((
                Local::now().format("%H:%M:%S").to_string(),
                msg.to_string(),
                kind,
            ));

            DEBUG_UPDATE = true;

            // if true && kind != DebugKind::Info {
            //     IS_RUNNING.store(false, Ordering::Relaxed);
            // }
        }
    }
}

pub struct Settings {
    pub panel_ui: bool,
    pub reactive_mode: bool,
    pub step_delay_input: String,
    pub step_delay: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            panel_ui: true,
            reactive_mode: true,
            step_delay_input: "0".to_string(),
            step_delay: 0,
        }
    }
}

pub struct WindowVisibility {
    pub display: bool,
    pub source: bool,
    pub memory_dump: bool,
    pub debugger: bool,
    pub disassembler: bool,
}

impl Default for WindowVisibility {
    fn default() -> Self {
        Self {
            display: true,
            source: true,
            memory_dump: true,
            debugger: true,
            disassembler: false,
        }
    }
}

#[derive(Default)]
pub struct App {
    pub emulator: Cpu6502<AppDebugger>,
    pub source_input: String,
    pub memory_dump_range: (u16, u16),
    pub memory_dump_range_input: (String, String),
    pub error: Option<String>,
    pub key_input: String,
    pub settings: Settings,
    pub window_visibility: WindowVisibility,
    pub disassembled: Vec<(usize, String, String)>,
}

impl App {
    pub fn new(program: &str) -> App {
        let src = Assembler::new(program).assemble().unwrap();

        let memory = Memory::new();

        let mut emulator = Cpu6502::<AppDebugger>::new(memory);

        emulator.reset();
        emulator.load(&src);

        Self {
            emulator,
            source_input: program.to_string(),
            memory_dump_range: (0x0000, 0x00FF),
            memory_dump_range_input: ("0000".to_string(), "00FF".to_string()),
            error: None,
            key_input: String::new(),
            ..Default::default()
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        if !self.settings.reactive_mode {
            ctx.request_repaint();
        }

        if self.window_visibility.disassembler {
            Window::new("Disassembler")
                .default_width(250.)
                .default_height(200.)
                .default_pos((700., 100.))
                .resizable(true)
                .show(ctx, |ui| DisassemblerUi.ui(ui, self));
        }

        TopBottomPanel::top("menu_bar").show(ctx, |ui| MenuBar.ui(ui, self));
        TopBottomPanel::bottom("status_bar").show(ctx, |ui| StatusBar.ui(ui, self));

        if self.settings.panel_ui {
            SidePanel::left("display_settings")
                .default_width(150.)
                .width_range(100.0..=300.)
                .resizable(true)
                .show(ctx, |ui| {
                    if self.window_visibility.display {
                        Display.ui(ui, self);
                        ui.separator();
                    }
                    SettingsUi.ui(ui, self);
                });

            if self.window_visibility.source {
                SidePanel::left("source_input")
                    .default_width(400.)
                    .width_range(100.0..=600.)
                    .resizable(true)
                    .show(ctx, |ui| SourceInput.ui(ui, self));
            }

            if self.window_visibility.memory_dump {
                TopBottomPanel::top("memory_dump_option")
                    .show(ctx, |ui| MemoryDumpOptions.ui(ui, self));
                if self.window_visibility.debugger {
                    TopBottomPanel::top("memory_dump")
                        .default_height(300.)
                        .height_range(100.0..=frame.info().window_info.size.y - 200.)
                        .resizable(true)
                        .show(ctx, |ui| MemoryDump.ui(ui, self));
                } else {
                    CentralPanel::default().show(ctx, |ui| MemoryDump.ui(ui, self));
                }
            }

            if self.window_visibility.debugger {
                TopBottomPanel::top("debugger_ui").show(ctx, |ui| DebuggerUi.ui(ui, self));
                CentralPanel::default().show(ctx, |ui| DebuggerOutput.ui(ui, self));
            } else {
                CentralPanel::default().show(ctx, |ui| MemoryDump.ui(ui, self));
            }
        } else {
            CentralPanel::default().show(ctx, |_| {});

            if self.window_visibility.display {
                Window::new("Display")
                    .default_width(160.)
                    .resizable(false)
                    .show(ctx, |ui| Display.ui(ui, self));
            }

            Window::new("Settings")
                .default_width(300.)
                .resizable(true)
                .show(ctx, |ui| SettingsUi.ui(ui, self));

            if self.window_visibility.source {
                Window::new("Source")
                    .default_width(400.)
                    .default_height(450.)
                    .resizable(true)
                    .show(ctx, |ui| SourceInput.ui(ui, self));
            }

            if self.window_visibility.memory_dump {
                Window::new("Memory Dump")
                    .default_width(300.)
                    .default_height(200.)
                    .resizable(true)
                    .show(ctx, |ui| {
                        MemoryDumpOptions.ui(ui, self);
                        ui.separator();
                        MemoryDump.ui(ui, self);
                    });
            }

            if self.window_visibility.debugger {
                Window::new("Debugger")
                    .default_width(300.)
                    .default_height(200.)
                    .resizable(true)
                    .show(ctx, |ui| {
                        DebuggerUi.ui(ui, self);
                        ui.separator();
                        DebuggerOutput.ui(ui, self);
                    });
            }
        }

        if IS_RUNNING.load(Ordering::Relaxed) {
            let op = self.emulator.step();

            thread::sleep(Duration::from_millis(self.settings.step_delay));

            if op == 0x00 {
                IS_RUNNING.store(false, Ordering::Relaxed);
                self.emulator.debug("Program finished");
            }
        }
    }
}
