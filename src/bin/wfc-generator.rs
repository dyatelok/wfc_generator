mod wave;
use raylib::prelude::*;

fn rot_to_xshift(rot: usize) -> f32 {
    match rot {
        0 => 0.0,
        1 => 1.0,
        2 => 1.0,
        3 => 0.0,
        _ => panic!("wrong rot"),
    }
}

fn rot_to_yshift(rot: usize) -> f32 {
    match rot {
        0 => 0.0,
        1 => 0.0,
        2 => 1.0,
        3 => 1.0,
        _ => panic!("wrong rot"),
    }
}

fn main() {
    let x_size: usize = 30;
    let y_size: usize = x_size;

    let ww = wave::Wave::new_load(x_size, y_size);
    let mut wave = ww.0;
    let textures_ref = ww.1;
    wave.reveal();

    let (mut rl, thread) = raylib::init().size(900, 900).title("WCF").build();

    let mut textures: Vec<(Texture2D, usize)> = vec![];
    for t in 0..textures_ref.len() {
        let empty = rl
            .load_texture(&thread, &(textures_ref[t].0)[..])
            .expect("could not load texture billboard");
        textures.push((empty, textures_ref[t].1));
    }

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            let ww = wave::Wave::new_load(x_size, y_size);
            wave = ww.0;
            wave.reveal();
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RED);
        for i in 0..x_size {
            for j in 0..y_size {
                d.draw_texture_ex(
                    &textures[wave.get_texture_id(i, j)].0,
                    Vector2 {
                        x: (i as f32 + rot_to_xshift(textures[wave.get_texture_id(i, j)].1))
                            / x_size as f32
                            * 900f32,
                        y: (j as f32 + rot_to_yshift(textures[wave.get_texture_id(i, j)].1))
                            / y_size as f32
                            * 900f32,
                    },
                    90.0 * textures[wave.get_texture_id(i, j)].1 as f32,
                    15.0 / x_size as f32,
                    Color::WHITE,
                );
            }
        }
    }
}
