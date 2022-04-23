mod wave;
use raylib::prelude::*;

fn main() {
    let x_size: usize = 15;
    let y_size: usize = 15;

    let ww = wave::Wave::new_load(x_size, y_size);
    let mut wave = ww.0;
    let textures_ref = ww.1;
    wave.reveal();

    let (mut rl, thread) = raylib::init()
        .size(x_size as i32 * 60, y_size as i32 * 60)
        .title("WCF")
        .build();

    let mut textures: Vec<Texture2D> = vec![];
    for t in 0..textures_ref.len() {
        let empty = rl
            .load_texture(&thread, &(textures_ref[t])[..])
            .expect("could not load texture billboard");
        textures.push(empty);
    }

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            let ww = wave::Wave::new_load(x_size, y_size);
            let mut wave = ww.0;
            //Тут мы не перегружаем текстуры при перезапуске
            wave.reveal();
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RED);
        for i in 0..x_size {
            for j in 0..y_size {
                d.draw_texture(
                    &textures[wave.get_texture_id(i, j)],
                    i as i32 * 60,
                    j as i32 * 60,
                    Color::WHITE,
                );
            }
        }
    }
}
