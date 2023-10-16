use raylib::prelude::*;
use std::collections::HashSet;
use std::fs::{remove_file, File};
use std::io::prelude::*;

fn usize_vec_to_string(w: Vec<usize>) -> String {
    let mut out = String::new();
    if !w.is_empty() {
        for v in w.iter().take(w.len() - 1) {
            out.push_str(&w[*v].to_string()[..]);
            out.push(' ');
        }
        out.push_str(&w[w.len() - 1].to_string()[..]);
    }
    out.push('\n');
    out
}

fn to_offset(rel: usize) -> (i32, i32) {
    match rel {
        0 => (-1, -1),
        1 => (-1, 0),
        2 => (-1, 1),
        3 => (0, -1),
        4 => (0, 1),
        5 => (1, -1),
        6 => (1, 0),
        7 => (1, 1),
        _ => panic!("unexpected rel"),
    }
}

fn offset_eq(offset: (i32, i32), e: [[usize; 3]; 3], c: [[usize; 3]; 3]) -> bool {
    for o1 in 0..=2 {
        for o2 in 0..=2 {
            if 0 <= o1 + offset.0
                && o1 + offset.0 < 3
                && 0 <= o2 + offset.1
                && o2 + offset.1 < 3
                && e[o1 as usize][o2 as usize]
                    != c[(o1 + offset.0) as usize][(o2 + offset.1) as usize]
            {
                return false;
            }
        }
    }
    true
}

fn find_match(i: usize, rel: usize, elems: &[(usize, [[usize; 3]; 3])]) -> Vec<usize> {
    let e = elems[i].1;
    let off = to_offset(rel);
    let mut ans: Vec<usize> = vec![];

    for (i, elem) in elems.iter().enumerate() {
        if offset_eq(off, elem.1, e) {
            ans.push(i);
        }
    }
    ans
}

fn as_path(s: usize) -> String {
    let mut st = String::from("textures/");
    st.push_str(&s.to_string()[..]);
    st.push_str(".png");
    st
}

fn string_from_buffer(x_size: usize, y_size: usize, buff: &[Vec<usize>]) -> String {
    let mut elems: HashSet<[[usize; 3]; 3]> = HashSet::new();
    let mut elem: [[usize; 3]; 3] = [[0; 3]; 3];
    for i in 1..x_size - 1 {
        for j in 1..y_size - 1 {
            for o1 in 0..3 {
                for o2 in 0..3 {
                    elem[o1][o2] = buff[i + o1 - 1][j + o2 - 1];
                }
            }
            elems.insert(elem);
        }
    }
    let mut num_elems: Vec<(usize, [[usize; 3]; 3])> = vec![];

    for (t, e) in elems.iter().enumerate() {
        println!("{}", t);
        println!("{:?}", e[0]);
        println!("{:?}", e[1]);
        println!("{:?}\n", e[2]);
        num_elems.push((t, *e));
    }

    let tile_types = num_elems.len();

    let mut sst: String = String::new();

    for i in 0..tile_types {
        sst.push_str(&num_elems[i].0.to_string()[..]);
        sst.push('\n');
        sst.push_str(&as_path(num_elems[i].1[1][1])[..]);
        sst.push_str(" 0\n");
        sst.push_str(&usize_vec_to_string(find_match(i, 0, &num_elems))[..]);
        sst.push_str(&usize_vec_to_string(find_match(i, 1, &num_elems))[..]);
        sst.push_str(&usize_vec_to_string(find_match(i, 2, &num_elems))[..]);
        sst.push_str(&usize_vec_to_string(find_match(i, 3, &num_elems))[..]);
        sst.push_str(&usize_vec_to_string(find_match(i, 4, &num_elems))[..]);
        sst.push_str(&usize_vec_to_string(find_match(i, 5, &num_elems))[..]);
        sst.push_str(&usize_vec_to_string(find_match(i, 6, &num_elems))[..]);
        sst.push_str(&usize_vec_to_string(find_match(i, 7, &num_elems))[..]);
        sst.push('\n');
    }
    sst
}

fn main() {
    let colors: Vec<(u8, u8, u8, u8)> = vec![
        (0, 0, 0, 255),
        (128, 0, 0, 255),
        (255, 0, 0, 255),
        (255, 0, 255, 255),
        (0, 128, 128, 255),
        (0, 128, 0, 255),
        (0, 255, 0, 255),
        (0, 255, 255, 255),
        (0, 0, 128, 255),
        (128, 0, 128, 255),
        (0, 0, 255, 255),
        (192, 192, 192, 255),
        (128, 128, 128, 255),
        (128, 128, 0, 255),
        (255, 0, 255, 255),
        (255, 255, 255, 255),
    ];

    let x_size: usize = 16;
    let y_size: usize = 16;

    let mut buffer: Vec<Vec<usize>> = vec![vec!(0; x_size); y_size];

    for i in 0..x_size {
        for j in 0..y_size {
            if (i + j) % 2 == 0 {
                buffer[i][j] = 15;
            }
        }
    }

    let mut cursor_x: i32 = 0;
    let mut cursor_y: i32 = 0;
    let mut base_x = 0;
    let mut base_y = 0;
    let mut copy_color: usize = 0;
    let mut vis_mode = false;

    let mut block_s_x: i32 = 0;
    let mut block_s_y: i32 = 0;
    let mut block_e_x: i32 = 0;
    let mut block_e_y: i32 = 0;

    let (mut rl, thread) = raylib::init().size(900, 900).title("map-to-data").build();

    rl.set_target_fps(60);

    let mut t: usize = 0;

    while !rl.window_should_close() {
        t = (t + 1) % 60;
        if let Some(key) = rl.get_key_pressed() {
            match key {
                KeyboardKey::KEY_UP => {
                    cursor_y = (cursor_y - 1).max(0);
                }
                KeyboardKey::KEY_DOWN => {
                    cursor_y = (cursor_y + 1).min(y_size as i32 - 1);
                }
                KeyboardKey::KEY_LEFT => {
                    cursor_x = (cursor_x - 1).max(0);
                }
                KeyboardKey::KEY_RIGHT => {
                    cursor_x = (cursor_x + 1).min(x_size as i32 - 1);
                }
                KeyboardKey::KEY_N => {
                    if vis_mode {
                        for i in block_s_x..=block_e_x {
                            for j in block_s_y..=block_e_y {
                                buffer[i as usize][j as usize] += 1;
                                buffer[i as usize][j as usize] %= 16;
                            }
                        }
                    } else {
                        buffer[cursor_x as usize][cursor_y as usize] += 1;
                        buffer[cursor_x as usize][cursor_y as usize] %= 16;
                    }
                }
                KeyboardKey::KEY_Y => {
                    copy_color = buffer[cursor_x as usize][cursor_y as usize];
                }
                KeyboardKey::KEY_P => {
                    buffer[cursor_x as usize][cursor_y as usize] = copy_color;
                    if vis_mode {
                        for i in block_s_x..=block_e_x {
                            for j in block_s_y..=block_e_y {
                                buffer[i as usize][j as usize] = copy_color;
                            }
                        }
                    }
                }
                KeyboardKey::KEY_S => {
                    println!("data loaded to file");
                    let mut s = String::new();
                    for i in 0..x_size {
                        for j in 0..y_size {
                            s.push((48 + buffer[i][j] as u8) as char);
                        }
                    }

                    let _ = remove_file("data/map.txt");
                    let mut file = File::create("data/map.txt")
                        .expect("Error encountered while creating file!");
                    file.write_all(s[..].as_bytes())
                        .expect("Error while writing to file");
                }
                KeyboardKey::KEY_L => {
                    let s = std::fs::read_to_string("data/map.txt").expect("Unable to read file");
                    let s = s.as_bytes();
                    for i in 0..x_size {
                        for j in 0..y_size {
                            buffer[i][j] = (s[i * x_size + j] - 48) as usize;
                        }
                    }
                    println!("loaded data from file");
                }
                KeyboardKey::KEY_V => {
                    //Перейти в режим выделения/выйти
                    vis_mode = !vis_mode;
                    if vis_mode {
                        base_x = cursor_x;
                        base_y = cursor_y;
                    }
                }
                KeyboardKey::KEY_T => {
                    //Трансформируем данные из буффера в данные для генерации
                    let s = string_from_buffer(x_size, y_size, &buffer);

                    let _ = remove_file("data/tiles-data.txt");
                    let mut file = File::create("data/tiles-data.txt")
                        .expect("Error encountered while creating file!");
                    file.write_all(s[..].as_bytes())
                        .expect("Error while writing to file");
                    println!("burref transformed to tiles-data");
                }
                _ => {}
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::ORANGE);
        for i in 0..x_size {
            for j in 0..y_size {
                d.draw_rectangle_rec(
                    Rectangle {
                        x: i as f32 / x_size as f32 * 900f32,
                        y: j as f32 / y_size as f32 * 900f32,
                        width: 900f32 / x_size as f32,
                        height: 900f32 / y_size as f32,
                    },
                    Color::from(colors[buffer[i][j]]),
                );
            }
        }
        d.draw_rectangle_lines_ex(
            Rectangle {
                x: cursor_x as f32 / x_size as f32 * 900f32,
                y: cursor_y as f32 / y_size as f32 * 900f32,
                width: 900f32 / x_size as f32,
                height: 900f32 / y_size as f32,
            },
            5,
            if t < 30 {
                Color::ORANGE
            } else {
                Color::DARKPURPLE
            },
        );
        if vis_mode {
            block_s_x = base_x.min(cursor_x);
            block_e_x = base_x.max(cursor_x);
            block_s_y = base_y.min(cursor_y);
            block_e_y = base_y.max(cursor_y);
            d.draw_rectangle_lines_ex(
                Rectangle {
                    x: block_s_x as f32 / x_size as f32 * 900f32,
                    y: block_s_y as f32 / y_size as f32 * 900f32,
                    width: 900.0 * (block_e_x - block_s_x + 1) as f32 / x_size as f32,
                    height: 900.0 * (block_e_y - block_s_y + 1) as f32 / y_size as f32,
                },
                5,
                if t < 30 { Color::RED } else { Color::DARKGREEN },
            );
        }
    }
}
