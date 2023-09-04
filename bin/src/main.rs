use assembler::Assembler;
use eframe::egui;
use egui::*;
use emulator::{memory::Memory, Cpu6502, CpuDebugger, Debugger};

// const SIZE: f32 = 25.;
// const RESOLUTION: usize = 10;
// const WIDTH: f32 = SIZE * RESOLUTION as f32;

static mut DEBUG_OUTPUT: String = String::new();

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(900., 600.)),
        ..Default::default()
    };
    // options.resizable = false;

    let app = App::new(
        r#"
BNE FOO
LDA #$01
STA $00
BRK

FOO:
LDA #$02
STA $01
BRK
"#
        .trim(),
    );

    eframe::run_native("Emulator", options, Box::new(|_| Box::new(app)))
}

#[derive(Default)]
struct AppDebugger;

impl Debugger for AppDebugger {
    fn debug(&mut self, msg: &str) {
        println!("Debug: {}", msg);

        unsafe {
            DEBUG_OUTPUT.push_str(msg);
            DEBUG_OUTPUT.push('\n');
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
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
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
            });
        });

        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add(egui::Label::new(
                    self.emulator
                        .registers
                        .to_string()
                        .as_str()
                        .replace('\n', " | "),
                ));
            });
        });

        egui::SidePanel::left("left")
            .default_width(400.)
            .width_range(100.0..=600.)
            .resizable(true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_sized(
                                ui.available_size(),
                                TextEdit::multiline(&mut self.source_input),
                            );
                        });
                    });
                });
            });

        egui::TopBottomPanel::top("central_bottom")
            .default_height(300.)
            .height_range(100.0..=600.)
            .resizable(true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add_sized(
                            ui.available_size(),
                            Label::new(RichText::new("Hello <b>world!</b>")),
                        );
                    });
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_sized(
                            ui.available_size(),
                            TextEdit::multiline(&mut unsafe { DEBUG_OUTPUT.clone() })
                                .cursor_at_end(false),
                        );
                    });
                });
            });
        });

        if self.is_running {
            let op = self.emulator.step();
            if op == 0x00 {
                self.is_running = false;
            }
        }
    }
}

/*
CentralPanel::default().show(ctx, |ui| {
            TopBottomPanel::top("toxp")
                .resizable(false)
                .show_inside(ui, |ui| {
                    if ui.button("run").clicked() {
                        self.is_running = true;
                    }
                });
            SidePanel::left("left")
                .default_width(400.)
                .resizable(true)
                .show_inside(ui, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.vertical_centered_justified(|ui| {
                            ui.add_sized(
                                ui.available_size(),
                                TextEdit::multiline(&mut self.source_input),
                            )
                        });
                    });
                });

            CentralPanel::default().show_inside(ui, |ui| {
                        TopBottomPanel::top("top")
                            .resizable(true)
                            .show_inside(ui, |ui| {
                                ui.add_sized(
                                    ui.available_size(),
                                    TextEdit::multiline(&mut memory_hexdump(
                                        self.emulator.memory.mem,
                                        0x0000,
                                        0x0020,
                                    )),
                                );
                            })
        });

                    CentralPanel::default().show_inside(ui, |ui| {
                        TopBottomPanel::top("2top")
                            .resizable(true)
                            .show_inside(ui, |ui| {
                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    ui.vertical_centered_justified(|ui| {
                                        ui.add_sized(
                                            ui.available_size(),
                                            TextEdit::multiline(
                                                &mut self.emulator.registers.to_string(),
                                            ),
                                        );
                                    });
                                });
                            });
                        CentralPanel::default().show_inside(ui, |ui| {
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                ui.vertical_centered_justified(|ui| {
                                    ui.add_sized(
                                        ui.available_size(),
                                        TextEdit::multiline(&mut memory_hexdump(
                                            self.emulator.memory.mem,
                                            0x0000,
                                            0xFFFF,
                                        )),
                                    );
                                });
                            });
                        });
                    });
                });

            if self.is_running {
                let op = self.emulator.step();
                if op == 0x00 {
                    self.is_running = false;
                }
            }
        });

*/

// BNE FOO
// LDA #$01
// STA $00
// BRK

// FOO:
//     LDA #$02
//     STA $01
//     BRK
//     "#;
//     let src = Assembler::new(s.to_string()).assemble().unwrap();
//     println!("{:?}", src);

//     let mut memory = Memory::new();
//     memory.set_debug_callback(Box::new(|msg| println!("Memory Debug      : {msg}")));

//     let mut emulator = Cpu::<Memory>::new(memory);
//     emulator.set_debug_callback(Box::new(|msg| println!("CPU Debug         : {msg}")));
//     emulator
//         .registers
//         .set_debug_callback(Box::new(|msg| println!("Register Debug    : {msg}")));

//     emulator.reset();
//     emulator.load(&src);
//     emulator.execute();

//     println!("{}", memory_hexdump(emulator.memory.mem, 0x0000, 0x0020));
//     // println!("{}", memory_hexdump(&emulator.memory, 0x8000, 0x800F));
// }
