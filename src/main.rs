use rand::{thread_rng, Rng};
use raylib::prelude::*;
use std::fs;

#[allow(dead_code)]
#[derive(Clone)]
struct TileProp {
    id: usize,               //id
    color: (u8, u8, u8, u8), //Цвет тайла
    up: Vec<usize>,          //possible neibors in directions
    down: Vec<usize>,
    left: Vec<usize>,
    right: Vec<usize>,
    up_superpos: SuperPos,
    down_superpos: SuperPos,
    left_superpos: SuperPos,
    right_superpos: SuperPos,
}

impl TileProp {
    fn new() -> TileProp {
        TileProp {
            id: usize::MAX,
            color: (0, 0, 0, 0),
            up: vec![],
            down: vec![],
            left: vec![],
            right: vec![],
            up_superpos: SuperPos::tr(0),
            down_superpos: SuperPos::tr(0),
            left_superpos: SuperPos::tr(0),
            right_superpos: SuperPos::tr(0),
        }
    }
    fn from(
        id: usize,
        color: (u8, u8, u8, u8),
        up: Vec<usize>,
        down: Vec<usize>,
        left: Vec<usize>,
        right: Vec<usize>,
        tile_types: usize,
    ) -> TileProp {
        TileProp {
            id,
            color,
            up: up.clone(),
            down: down.clone(),
            left: left.clone(),
            right: right.clone(),
            up_superpos: SuperPos::from(tile_types, up),
            down_superpos: SuperPos::from(tile_types, down),
            left_superpos: SuperPos::from(tile_types, left),
            right_superpos: SuperPos::from(tile_types, right),
        }
    }
}

fn string_to_vec_of_usize(s: &String) -> Vec<usize> {
    s.split(' ')
        .into_iter()
        .map(|s| match s.parse::<usize>() {
            Ok(a) => a,
            Err(_) => panic!("failed to parce file"),
        })
        .collect::<Vec<usize>>()
}

fn string_to_color(s: &String) -> (u8, u8, u8, u8) {
    let cls = s
        .split(' ')
        .into_iter()
        .map(|s| match s.parse::<u8>() {
            Ok(a) => a,
            Err(_) => panic!("failed to parce color"),
        })
        .collect::<Vec<u8>>();
    if cls.len() != 4 {
        panic!("wrong color format");
    }
    (cls[0], cls[1], cls[2], cls[3])
}

fn read_file() -> Vec<TileProp> {
    let data: String = fs::read_to_string("tiles-data.txt").expect("Unable to read file");
    let strings = data.split('\n');
    let strings = strings.collect::<Vec<&str>>();
    let strings = strings
        .into_iter()
        .map(|s| String::from(s))
        .collect::<Vec<String>>();

    let tile_types: usize = strings.len() / 7 + 1;
    let mut tiles_prop: Vec<TileProp> = vec![TileProp::new(); tile_types];

    let mut id: usize;
    let mut color: (u8, u8, u8, u8);
    let mut up: Vec<usize>;
    let mut down: Vec<usize>;
    let mut left: Vec<usize>;
    let mut right: Vec<usize>;

    for v in 0..tile_types {
        id = match (&strings[v * 7][..]).parse() {
            Ok(a) => a,
            Err(_) => panic!("failed to parce file"),
        };
        color = string_to_color(&strings[v * 7 + 1]);
        up = string_to_vec_of_usize(&strings[v * 7 + 2]);
        down = string_to_vec_of_usize(&strings[v * 7 + 3]);
        left = string_to_vec_of_usize(&strings[v * 7 + 4]);
        right = string_to_vec_of_usize(&strings[v * 7 + 5]);
        tiles_prop[v] = TileProp::from(id, color, up, down, left, right, tile_types);
    }
    tiles_prop
}

#[derive(Clone, PartialEq, Eq)]
struct SuperPos {
    cont: Vec<bool>,
}

impl SuperPos {
    fn tr(n: usize) -> SuperPos {
        SuperPos {
            cont: vec![true; n],
        }
    }
    fn fs(n: usize) -> SuperPos {
        SuperPos {
            cont: vec![false; n],
        }
    }
    fn or(&mut self, s: &SuperPos) {
        for i in 0..self.cont.len() {
            self.cont[i] = self.cont[i] || s.cont[i];
        }
    }
    fn and(&mut self, s: &SuperPos) {
        for i in 0..self.cont.len() {
            self.cont[i] = self.cont[i] && s.cont[i];
        }
    }
    fn from(n: usize, l: Vec<usize>) -> SuperPos {
        let mut cont = vec![false; n];
        for i in 0..l.len() {
            cont[l[i]] = true;
        }
        SuperPos { cont }
    }
    fn contains(&self, obj: &SuperPos) -> bool {
        for i in 0..self.cont.len() {
            if self.cont[i] == true && obj.cont[i] == false {
                return false;
            }
        }
        true
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Entropy {
    pos: usize,
    unrev: usize,
}

impl Entropy {
    fn new(pos: usize) -> Entropy {
        Entropy { pos, unrev: 0 }
    }
    fn max() -> Entropy {
        Entropy {
            pos: 100,
            unrev: 100,
        }
    }
    fn is_obs(&self) -> bool {
        self.pos == 1
    }
}

#[derive(Clone)]
struct Tile {
    sup: SuperPos,
    ent: Entropy,
}

impl Tile {
    fn new(n: usize) -> Tile {
        Tile {
            sup: SuperPos::tr(n),
            ent: Entropy::new(n),
        }
    }
}

struct WaveCopy {
    tiles: Vec<Vec<Tile>>,
}

struct Wave {
    x_size: usize,
    y_size: usize,
    tiles: Vec<Vec<Tile>>,
    tile_types: usize,
    tiles_prop: Vec<TileProp>,
    stack: Vec<WaveCopy>,
}

impl Wave {
    fn new(x_size: usize, y_size: usize, tile_types: usize, tiles_prop: Vec<TileProp>) -> Wave {
        Wave {
            x_size,
            y_size,
            tiles: vec![vec!(Tile::new(tile_types); x_size); y_size],
            tile_types,
            tiles_prop,
            stack: vec![],
        }
    }
    fn load(&mut self, copy: WaveCopy) {
        self.tiles = copy.tiles;
    }
    fn copy(&self) -> WaveCopy {
        WaveCopy {
            tiles: self.tiles.clone(),
        }
    }
    fn color(&self, x: usize, y: usize) -> Color {
        let mut r: f32 = 0.0;
        let mut g: f32 = 0.0;
        let mut b: f32 = 0.0;
        for i in 0..self.tile_types {
            if self.tiles[x][y].sup.cont[i] == true {
                r += self.tiles_prop[i].color.0 as f32;
                g += self.tiles_prop[i].color.1 as f32;
                b += self.tiles_prop[i].color.2 as f32;
            }
        }
        r /= self.tiles[x][y].ent.pos as f32;
        g /= self.tiles[x][y].ent.pos as f32;
        b /= self.tiles[x][y].ent.pos as f32;
        Color::from((r as u8, g as u8, b as u8, 255))
    }
    fn set_tile(&mut self, x: usize, y: usize, typ: usize) {
        self.tiles[x][y].sup.cont = vec![false; self.tile_types];
        self.tiles[x][y].sup.cont[typ] = true;
    }
    fn update_around(&mut self, x: usize, y: usize) -> [bool; 4] {
        let mut is_updated: [bool; 4] = [true; 4];
        if x as i32 - 1 >= 0 {
            let mut suppos = SuperPos::fs(self.tile_types);
            for i in 0..self.tile_types {
                if self.tiles[x][y].sup.cont[i] == true {
                    suppos.or(&self.tiles_prop[i].left_superpos);
                }
            }
            is_updated[2] = self.tiles[x - 1][y].sup.contains(&suppos);
            self.tiles[x - 1][y].sup.and(&suppos);
        }
        if x + 1 < self.x_size {
            let mut suppos = SuperPos::fs(self.tile_types);
            for i in 0..self.tile_types {
                if self.tiles[x][y].sup.cont[i] == true {
                    suppos.or(&self.tiles_prop[i].right_superpos);
                }
            }
            is_updated[3] = self.tiles[x + 1][y].sup.contains(&suppos);
            self.tiles[x + 1][y].sup.and(&suppos);
        }
        if y as i32 - 1 >= 0 {
            let mut suppos = SuperPos::fs(self.tile_types);
            for i in 0..self.tile_types {
                if self.tiles[x][y].sup.cont[i] == true {
                    suppos.or(&self.tiles_prop[i].up_superpos);
                }
            }
            is_updated[0] = self.tiles[x][y - 1].sup.contains(&suppos);
            self.tiles[x][y - 1].sup.and(&suppos);
        }
        if y + 1 < self.y_size {
            let mut suppos = SuperPos::fs(self.tile_types);
            for i in 0..self.tile_types {
                if self.tiles[x][y].sup.cont[i] == true {
                    suppos.or(&self.tiles_prop[i].down_superpos);
                }
            }
            is_updated[1] = self.tiles[x][y + 1].sup.contains(&suppos);
            self.tiles[x][y + 1].sup.and(&suppos);
        }
        is_updated
    }
    fn update_entropy(&mut self) -> bool {
        for i in 0..self.x_size {
            for j in 0..self.y_size {
                self.tiles[i][j].ent.pos = 0;
                for k in 0..self.tile_types {
                    if self.tiles[i][j].sup.cont[k] {
                        self.tiles[i][j].ent.pos += 1;
                    }
                }
                if self.tiles[i][j].ent.pos == 0 {
                    return true;
                }
            }
        }
        for i in 0..self.x_size {
            for j in 0..self.y_size {
                if i as i32 - 1 >= 0 {
                    if self.tiles[i - 1][j].ent.pos != 1 {
                        self.tiles[i][j].ent.unrev += 1;
                    }
                }
                if i + 1 < self.x_size {
                    if self.tiles[i + 1][j].ent.pos != 1 {
                        self.tiles[i][j].ent.unrev += 1;
                    }
                }
                if j as i32 - 1 >= 0 {
                    if self.tiles[i][j - 1].ent.pos != 1 {
                        self.tiles[i][j].ent.unrev += 1;
                    }
                }
                if j + 1 < self.y_size {
                    if self.tiles[i][j + 1].ent.pos != 1 {
                        self.tiles[i][j].ent.unrev += 1;
                    }
                }
            }
        }
        false
    }
    fn update_from(&mut self, x: usize, y: usize) {
        let ups = self.update_around(x, y);
        if !ups[0] {
            self.update_from(x, y - 1);
        }
        if !ups[1] {
            self.update_from(x, y + 1);
        }
        if !ups[2] {
            self.update_from(x - 1, y);
        }
        if !ups[3] {
            self.update_from(x + 1, y);
        }
    }
    fn observe(&mut self) -> Option<(usize, usize)> {
        let mut posses: Vec<(usize, usize)> = vec![];
        let mut sample = Entropy::max();

        for i in 0..self.x_size {
            for j in 0..self.y_size {
                if self.tiles[i][j].ent < sample && !self.tiles[i][j].ent.is_obs() {
                    sample = self.tiles[i][j].ent;
                    posses = vec![(i, j)];
                } else {
                    if self.tiles[i][j].ent == sample {
                        posses.push((i, j));
                    }
                }
            }
        }

        if sample == Entropy::max() {
            return None;
        }

        let mut rng = thread_rng();
        let index: usize = rng.gen_range(0..posses.len());

        return Some(posses[index]);
    }
    fn reveal(&mut self) {
        let mut rng = thread_rng();
        loop {
            let stc = self.update_entropy();
            if stc {
                let pop = self.stack.pop();
                match pop {
                    Some(pop) => {
                        self.load(pop);
                    }
                    None => {
                        panic!("No possible tilings");
                    }
                }
                continue;
            }
            self.stack.push(self.copy());
            let obs = self.observe();
            match obs {
                Some((x, y)) => {
                    let mut typs = rng.gen_range(1..=self.tiles[x][y].ent.pos);
                    let mut typ: usize = 0;
                    let mut i: usize = 0;
                    while typs != 0 {
                        if self.tiles[x][y].sup.cont[i] {
                            typs -= 1;
                            if typs == 0 {
                                typ = i;
                            }
                        }
                        i += 1;
                    }
                    self.set_tile(x, y, typ);
                    self.update_from(x, y);
                }
                None => {
                    println!("Tiling finished");
                    break;
                }
            }
        }
    }
}

fn main() {
    let x_size: usize = 15;
    let y_size: usize = 15;
    let data = read_file();
    let tile_types = data.len();
    let mut wave = Wave::new(x_size, y_size, tile_types, data);

    wave.reveal();

    let (mut rl, thread) = raylib::init()
        .size(x_size as i32 * 60, y_size as i32 * 60)
        .title("WCF")
        .build();

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RED);
        for i in 0..x_size {
            for j in 0..y_size {
                d.draw_rectangle(i as i32 * 60, j as i32 * 60, 60, 60, wave.color(i, j));
            }
        }
    }
}
