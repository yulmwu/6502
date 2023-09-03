use assembler::Assembler;
use eframe::egui;
use egui::*;
use emulator::{
    cpu::Cpu,
    memory::{memory_hexdump, Memory},
    Debugger,
};

// const SIZE: f32 = 25.;
// const RESOLUTION: usize = 10;
// const WIDTH: f32 = SIZE * RESOLUTION as f32;

fn main() -> Result<(), eframe::Error> {
    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(egui::Vec2::new(900., 600.));
    options.resizable = false;

    eframe::run_native(
        "Emulator",
        options,
        Box::new(|_| {
            Box::new(App::new(
                r#"
BNE FOO
LDA #$01
STA $00
BRK

FOO:
    LDA #$02
    STA $01
    BRK
    "#,
            ))
        }),
    )
}

// #[derive(Debug)]
struct App {
    emulator: Cpu<Memory>,
    is_running: bool,
    counter: usize,
    source_input: String,
}

impl App {
    fn new(program: &str) -> Self {
        let src = Assembler::new(program.to_string()).assemble().unwrap();

        let mut memory = Memory::new();
        memory.set_debug_callback(Box::new(|msg| println!("Memory Debug      : {msg}")));

        let mut emulator = Cpu::<Memory>::new(memory);
        emulator.set_debug_callback(Box::new(|msg| println!("CPU Debug         : {msg}")));
        emulator
            .registers
            .set_debug_callback(Box::new(|msg| println!("Register Debug    : {msg}")));

        emulator.reset();
        emulator.load(&src);

        Self {
            emulator,
            is_running: false,
            counter: 0,
            source_input: String::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();
        egui::CentralPanel::default().show(ctx, |ui| {
            // std::thread::sleep(std::time::Duration::from_millis(100));
            egui::SidePanel::left("left")
                .default_width(500.)
                .resizable(true)
                .show_inside(ui, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        if ui.button("run").clicked() {
                            // self.emulator.reset();
                            self.is_running = true;
                        }
                        // });
                        ui.add(
                            // [right_panel_width, 50.],
                            egui::TextEdit::multiline(&mut self.source_input),
                        )
                    });
                });

            egui::CentralPanel::default()
                // .resizable(true)
                .show_inside(ui, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.add(
                            // [right_panel_width, 50.],
                            egui::TextEdit::multiline(&mut self.emulator.registers.to_string()),
                        );
                        ui.add(
                            // [right_panel_width, 200.],
                            egui::TextEdit::multiline(&mut memory_hexdump(
                                self.emulator.memory.mem,
                                0x0000,
                                0x0020,
                            )),
                        );
                    });
                });

            // ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
            //     // ui.add_sized(
            //     //     [left_panel_width, 200.],
            //     //     egui::TextEdit::multiline(&mut self.source_input),
            //     // );
            // });
            // ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
            //     ui.add_sized(
            //         [right_panel_width, 50.],
            //         egui::TextEdit::multiline(&mut self.emulator.registers.to_string()),
            //     );
            //     ui.add_sized(
            //         [right_panel_width, 200.],
            //         egui::TextEdit::multiline(&mut memory_hexdump(
            //             self.emulator.memory.mem,
            //             0x0000,
            //             0x0020,
            //         )),
            //     );
            // });

            if self.is_running {
                let op = self.emulator.step();
                self.counter += 1;
                if op == 0x00 {
                    self.is_running = false;
                }
            }
        });
    }
}

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
