use assembler::Assembler;
use chrono::prelude::*;
use eframe::egui;
use egui::*;
use emulator::{
    memory::{memory_hexdump, Memory, MemoryDumpResult},
    registers::Registers,
    Cpu6502, CpuDebugger, Debugger,
};

/// (time, message)
static mut DEBUG_OUTPUT: (String, String) = (String::new(), String::new());
static mut DEBUG_UPDATE: bool = false;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(1000., 600.)),
        ..Default::default()
    };
    // options.resizable = false;

    let app = App::new(
        r#"BNE FOO
LDA #$01
STA $00
BRK

FOO:
LDA #$02
STA $01
BRK
"#,
    );

    eframe::run_native("6502 Emulator", options, Box::new(|_| Box::new(app)))
}

#[derive(Default)]
struct AppDebugger;

impl Debugger for AppDebugger {
    fn debug(&mut self, msg: &str) {
        unsafe {
            DEBUG_OUTPUT
                .0
                .push_str(Local::now().format("%H:%M:%S").to_string().as_str());
            DEBUG_OUTPUT.1.push('\n');
            DEBUG_OUTPUT.1.push_str(msg);
            DEBUG_OUTPUT.1.push('\n');
            DEBUG_UPDATE = true;
        }
    }
}

#[derive(Default)]
struct App {
    emulator: Cpu6502<AppDebugger>,
    is_running: bool,
    source_input: String,
}

impl App {
    fn new(program: &str) -> App {
        let src = Assembler::new(program.to_string()).assemble().unwrap();

        let memory = Memory::new();

        let mut emulator = Cpu6502::<AppDebugger>::new(memory);

        emulator.reset();
        emulator.load(&src);

        Self {
            emulator,
            is_running: false,
            source_input: program.to_string(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();

        TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("run").clicked() {
                    self.is_running = true;
                }

                if ui.button("load").clicked() {
                    let src = Assembler::new(self.source_input.clone())
                        .assemble()
                        .unwrap();

                    self.emulator.reset();
                    self.emulator.load(&src);
                }

                if ui.button("reset").clicked() {
                    self.emulator.reset();
                }
            });
        });

        TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.horizontal(|ui| {
                    let r @ Registers {
                        a, x, y, p, sp, pc, ..
                    } = &self.emulator.registers;

                    ui.add(Label::new(
                        RichText::new(format!("A: {a:02X}")).strong().monospace(),
                    ));
                    ui.separator();
                    ui.add(Label::new(
                        RichText::new(format!("X: {x:02X}")).strong().monospace(),
                    ));
                    ui.separator();
                    ui.add(Label::new(
                        RichText::new(format!("Y: {y:02X}")).strong().monospace(),
                    ));
                    ui.separator();
                    ui.add(Label::new(
                        RichText::new(format!("P: {p:02X}")).strong().monospace(),
                    ));
                    ui.separator();
                    ui.add(Label::new(
                        RichText::new(format!("SP: {sp:02X}")).strong().monospace(),
                    ));
                    ui.separator();
                    ui.add(Label::new(
                        RichText::new(format!("PC: {pc:04X}")).strong().monospace(),
                    ));
                    ui.separator();
                    ui.add(Label::new(
                        RichText::new(format!(
                            "{} {} - {}  {} {} {} {} (NV-B DIZC)",
                            r.get_flag_negative() as u8,
                            r.get_flag_overflow() as u8,
                            r.get_flag_break() as u8,
                            r.get_flag_decimal() as u8,
                            r.get_flag_interrupt_disable() as u8,
                            r.get_flag_zero() as u8,
                            r.get_flag_carry() as u8
                        ))
                        .strong()
                        .monospace(),
                    ));
                });
            });
        });

        SidePanel::left("left")
            .default_width(400.)
            .width_range(100.0..=600.)
            .resizable(true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ScrollArea::vertical().show(ui, |ui| {
                        ui.style_mut().visuals.extreme_bg_color = Color32::default();
                        ui.vertical_centered(|ui| {
                            ui.add_sized(
                                ui.available_size(),
                                TextEdit::multiline(&mut self.source_input)
                                    // .font(TextStyle::Monospace)
                                    .font(FontId::new(15., FontFamily::Monospace))
                                    .text_color(Color32::WHITE),
                            );
                        });
                    });
                });
            });

        TopBottomPanel::top("central_top")
            .default_height(400.)
            .height_range(100.0..=frame.info().window_info.size.y - 200.)
            .resizable(true)
            .show(ctx, |ui| {
                memory_dump(ui, memory_hexdump(self.emulator.memory.mem, 0x0000, 0x01FF));
            });

        TopBottomPanel::top("central_top2").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("step").clicked() {
                    self.emulator.step();
                }

                if ui.button("clear").clicked() {
                    unsafe {
                        DEBUG_OUTPUT.0.clear();
                        DEBUG_OUTPUT.1.clear();
                    }
                }
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let debug = unsafe { DEBUG_OUTPUT.clone() };
                        let time = Label::new(
                            RichText::new(debug.0)
                                .monospace()
                                .color(Color32::LIGHT_BLUE),
                        );
                        let msg =
                            Label::new(RichText::new(debug.1).monospace().color(Color32::WHITE));

                        if unsafe { DEBUG_UPDATE } {
                            ui.add(time);
                            ui.add(msg).scroll_to_me(Some(Align::BOTTOM));
                        } else {
                            ui.add(time);
                            ui.add(msg);
                        }
                    });
                });

            if unsafe { DEBUG_UPDATE } {
                unsafe {
                    DEBUG_UPDATE = false;
                }
            }
        });

        if self.is_running {
            let op = self.emulator.step();
            if op == 0x00 {
                self.is_running = false;
            }
        }
    }
}

fn memory_dump(ui: &mut Ui, dump: MemoryDumpResult) {
    ScrollArea::both()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for i in dump {
                ui.horizontal(|ui| {
                    ui.add(Label::new(
                        RichText::new(format!("0x{:04X}", i.0))
                            .strong()
                            .color(Color32::from_rgb(50, 180, 80))
                            .monospace(),
                    ));
                    ui.separator();
                    for j in i.1 {
                        ui.add(Label::new(RichText::new(format!(" {:02X}", j)).monospace()));
                    }
                    ui.separator();
                    for j in i.2 {
                        ui.add(Label::new(
                            RichText::new(j)
                                .color(Color32::from_rgb(50, 180, 80))
                                .monospace(),
                        ));
                    }
                    ui.separator();
                });
                ui.separator();
            }
        });
}
