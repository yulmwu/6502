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
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();

        TopBottomPanel::top("menu_bar").show(ctx, |ui| MenuBar.ui(ui, self));

        TopBottomPanel::bottom("status_bar").show(ctx, |ui| StatusBar.ui(ui, self));

        SidePanel::left("bottom2")
            .default_width(100.)
            .resizable(true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    let width = 32.;
                    let height = 32.;
                    let mut color_image =
                        ColorImage::new([width as usize, height as usize], Color32::TRANSPARENT);

                    let mut i = 0;
                    for y in 0..32 {
                        for x in 0..32 {
                            let data = self.emulator.memory.mem[0x0200 + i];
                            let color = match data {
                                /* BLACK */ 0x00 => Color32::from_rgb(0, 0, 0),
                                /* WHITE */ 0x01 => Color32::from_rgb(255, 255, 255),
                                /* RED */ 0x02 => Color32::from_rgb(255, 0, 0),
                                /* CYAN */ 0x03 => Color32::from_rgb(0, 255, 255),
                                /* PURPLE */ 0x04 => Color32::from_rgb(255, 0, 255),
                                /* GREEN */ 0x05 => Color32::from_rgb(0, 255, 0),
                                /* BLUE */ 0x06 => Color32::from_rgb(0, 0, 255),
                                /* YELLOW */ 0x07 => Color32::from_rgb(255, 255, 0),
                                /* ORANGE */ 0x08 => Color32::from_rgb(255, 165, 0),
                                /* BROWN */ 0x09 => Color32::from_rgb(165, 42, 42),
                                /* LIGHT_RED */ 0x0A => Color32::from_rgb(255, 128, 128),
                                /* DARK_GRAY */ 0x0B => Color32::from_rgb(96, 96, 96),
                                /* GRAY */ 0x0C => Color32::from_rgb(160, 160, 160),
                                /* LIGHT_GREEN */ 0x0D => Color32::from_rgb(0x90, 0xEE, 0x90),
                                /* LIGHT_BLUE */ 0x0E => Color32::from_rgb(0xAD, 0xD8, 0xE6),
                                /* LIGHT_GRAY */ 0x0F => Color32::from_rgb(220, 220, 220),
                                _ => Color32::TRANSPARENT,
                            };
                            color_image[(x, y)] = color;
                            i += 1;
                        }
                    }

                    let texture =
                        ui.ctx()
                            .load_texture("test", color_image, TextureOptions::NEAREST);

                    ui.image(&texture, Vec2::new(width * 5., height * 5.));
                });
            });

        SidePanel::left("source_input")
            .default_width(400.)
            .width_range(100.0..=600.)
            .resizable(true)
            .show(ctx, |ui| SourceInput.ui(ui, self));

        TopBottomPanel::top("memory_dump_option").show(ctx, |ui| MemoryDumpOptions.ui(ui, self));

        TopBottomPanel::top("memory_dump")
            .default_height(300.)
            .height_range(100.0..=frame.info().window_info.size.y - 200.)
            .resizable(true)
            .show(ctx, |ui| MemoryDump.ui(ui, self));

        TopBottomPanel::top("debugger_ui").show(ctx, |ui| DebuggerUi.ui(ui, self));

        CentralPanel::default().show(ctx, |ui| DebuggerOutput.ui(ui, self));

        if self.is_running {
            let op = self.emulator.step();
            if op == 0x00 {
                self.is_running = false;
                self.emulator.debug("Program finished");
            }
        }
    }
}
