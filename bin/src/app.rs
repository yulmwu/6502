use crate::{ui::*, View, DEBUG_OUTPUT, DEBUG_UPDATE};
use assembler::Assembler;
use chrono::prelude::*;
use eframe::egui::*;
use emulator::{memory::Memory, Cpu6502, CpuDebugger, Debugger};

#[derive(Default)]
pub struct AppDebugger;

impl Debugger for AppDebugger {
    fn debug(&mut self, msg: &str) {
        unsafe {
            DEBUG_OUTPUT
                .0
                .push_str(Local::now().format("%H:%M:%S").to_string().as_str());
            #[allow(clippy::single_char_add_str)]
            DEBUG_OUTPUT.0.push_str("\n");
            DEBUG_OUTPUT.1.push_str(msg);
            #[allow(clippy::single_char_add_str)]
            DEBUG_OUTPUT.1.push_str("\n");
            DEBUG_UPDATE = true;
        }
    }
}

#[derive(Default)]
pub struct App {
    pub emulator: Cpu6502<AppDebugger>,
    pub is_running: bool,
    pub source_input: String,
    pub memory_dump_range: (u16, u16),
    pub memory_dump_range_input: (String, String),
    pub is_display_open: bool,
}

impl App {
    pub fn new(program: &str) -> App {
        let src = Assembler::new(program.to_string()).assemble().unwrap();

        let memory = Memory::new();

        let mut emulator = Cpu6502::<AppDebugger>::new(memory);

        emulator.reset();
        emulator.load(&src);

        Self {
            emulator,
            is_running: false,
            source_input: program.to_string(),
            memory_dump_range: (0x0000, 0x00FF),
            memory_dump_range_input: ("0000".to_string(), "00FF".to_string()),
            is_display_open: false,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();

        TopBottomPanel::top("menu").show(ctx, |ui| MenuBar.ui(ui, self));

        TopBottomPanel::bottom("status").show(ctx, |ui| StatusBar.ui(ui, self));

        // TopBottomPanel::bottom("bottom2").show(ctx, |ui| {});

        SidePanel::left("input")
            .default_width(400.)
            .width_range(100.0..=600.)
            .resizable(true)
            .show(ctx, |ui| SourceInput.ui(ui, self));

        TopBottomPanel::top("memory_dump_option").show(ctx, |ui| MemoryDumpOptions.ui(ui, self));

        TopBottomPanel::top("central_top")
            .default_height(300.)
            .height_range(100.0..=frame.info().window_info.size.y - 200.)
            .resizable(true)
            .show(ctx, |ui| MemoryDump.ui(ui, self));

        TopBottomPanel::top("central_top2").show(ctx, |ui| DebuggerUi.ui(ui, self));

        CentralPanel::default().show(ctx, |ui| DebuggerOutput.ui(ui, self));

        if self.is_running {
            let op = self.emulator.step();
            if op == 0x00 {
                self.is_running = false;
            }
        }
    }
}
