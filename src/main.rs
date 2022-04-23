mod wave;
use raylib::prelude::*;

fn main() {
    let x_size: usize = 15;
    let y_size: usize = 15;

    let mut wave = wave::Wave::new_load(x_size, y_size);
    wave.reveal();

    let (mut rl, thread) = raylib::init()
        .size(x_size as i32 * 60, y_size as i32 * 60)
        .title("WCF")
        .build();

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            wave = wave::Wave::new_load(x_size, y_size);
            wave.reveal();
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RED);
        for i in 0..x_size {
            for j in 0..y_size {
                d.draw_rectangle(i as i32 * 60, j as i32 * 60, 60, 60, wave.color(i, j));
            }
        }
    }
}
