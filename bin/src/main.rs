use assembler::Assembler;
use bevy::prelude::*;
use bevy_pixel_buffer::prelude::*;
use emulator::{
    cpu::Cpu,
    memory::{memory_hexdump, Memory},
};

const SIZE: u32 = 32;
const PIXEL_SIZE: u32 = 16;
const WIDTH: u32 = SIZE * PIXEL_SIZE;

fn main() {
    let s = r#"
LDA #$02
CMP #$01
BNE FOO
LDA #$01
STA $00
BRK

FOO:
    LDA #$02
    STA $01
    BRK
    "#;
    let src = Assembler::new(s.to_string()).assemble().unwrap();
    println!("{:?}", src);

    let mut memory = Memory::new();
    memory.set_debug_callback(Box::new(|msg| println!("Memory Debug      : {msg}")));

    let mut emulator = Cpu::<Memory>::new(memory);
    emulator.set_debug_callback(Box::new(|msg| println!("CPU Debug         : {msg}")));
    emulator
        .registers
        .set_debug_callback(Box::new(|msg| println!("Register Debug    : {msg}")));

    emulator.reset();
    emulator.load(&src);
    emulator.execute();

    println!("{}", memory_hexdump(emulator.memory.mem, 0x0000, 0x0020));
    // println!("{}", memory_hexdump(&emulator.memory, 0x8000, 0x800F));
    let size = PixelBufferSize {
        size: UVec2::new(SIZE, SIZE),
        pixel_size: UVec2::new(PIXEL_SIZE, PIXEL_SIZE),
    };

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "6502 Emulator".to_string(),
                    resolution: (WIDTH as f32, WIDTH as f32).into(),
                    resizable: false,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            PixelBufferPlugin,
        ))
        .add_systems(Startup, pixel_buffer_setup(size))
        .add_systems(Startup, clear_color)
        .add_systems(Update, update)
        .run();
}

fn clear_color(mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = Color::BLACK;
}

fn update(input: Res<Input<KeyCode>>, mut pb: QueryPixelBuffer) {
    if input.pressed(KeyCode::Space) {
        let l = SIZE / 2;
        pb.frame().set(UVec2::new(l, l), Pixel::random()).unwrap();
    }
}
