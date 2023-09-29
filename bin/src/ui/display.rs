use crate::{app::App, View};
use eframe::egui::*;

pub struct Display;

impl View for Display {
    fn ui(&mut self, ui: &mut Ui, app: &mut App) {
        ui.input(|ui| {
            let k = ui
                .events
                .iter()
                .filter_map(|e| match e {
                    Event::Key { key, .. } => {
                        if key.name().len() == 1 {
                            Some(key.name())
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();

            if !k.is_empty() {
                let ascii = k[0].as_bytes()[0];
                app.key_input = format!("{} ({ascii}) (0x00FF = 0x{ascii:X})", k[0]);
                app.emulator.memory.mem[0x00FF] = ascii;
            }
        });
        ui.vertical_centered(|ui| {
            let width = 32.;
            let height = 32.;
            let mut color_image =
                ColorImage::new([width as usize, height as usize], Color32::TRANSPARENT);

            let mut i = 0;
            for y in 0..32 {
                for x in 0..32 {
                    let data = app.emulator.memory.mem[0x0200 + i];
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

            let texture = ui
                .ctx()
                .load_texture("display", color_image, TextureOptions::NEAREST);

            ui.image(&texture, Vec2::new(width * 5., height * 5.));
        });

        ui.add(Label::new(
            RichText::new(app.key_input.clone())
                .monospace()
                .color(Color32::WHITE),
        ));
    }
}
