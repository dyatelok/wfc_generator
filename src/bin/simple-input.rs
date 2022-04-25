use std::collections::HashSet;
use std::fs::{self, remove_file, File};
use std::io::prelude::*;

// Простой формат ввода
// каждая строка - путь до текстуры ,вверх, показывающая тип ввода - вывода в эту сторону , строка влево, строка вправо, строка вниз
// в данном варианте мы подразумеваем, что каждый тайл может распологаться в любой ориентайии
// режим по факту сделан для демонстрации возможностей для примеров по типу труб или микросхем

fn parce_to_usize(s: &str) -> usize {
    match s.parse::<usize>() {
        Ok(a) => a,
        Err(_) => panic!("failed to parce file"),
    }
}

fn read_file() -> Vec<(usize, String, (usize, usize, usize, usize))> {
    let data: String = fs::read_to_string("simple-input.txt").expect("Unable to read file");
    let strings = data.split('\n');
    let strings = strings
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|s| String::from(s))
        .collect::<Vec<String>>();

    let tile_types: usize = strings.len() - 1;

    let mut tiles_prop: Vec<(usize, String, (usize, usize, usize, usize))> = vec![];
    let mut spl;

    for i in 0..tile_types {
        spl = strings[i].split(' ').collect::<Vec<&str>>();
        tiles_prop.push((
            i,
            String::from(spl[0]),
            (
                parce_to_usize(spl[1]),
                parce_to_usize(spl[2]),
                parce_to_usize(spl[3]),
                parce_to_usize(spl[4]),
            ),
        ));
    }
    tiles_prop
}

fn rot(
    e: (usize, String, (usize, usize, usize, usize)),
) -> (usize, String, (usize, usize, usize, usize)) {
    (e.0, e.1, (e.2 .1, e.2 .3, e.2 .0, e.2 .2))
}

fn get_rot(
    b: (usize, String, (usize, usize, usize, usize)),
    e: &(usize, String, (usize, usize, usize, usize)),
) -> usize {
    let mut ee = e.clone();
    if b == ee {
        return 0;
    }
    ee = rot(ee);
    if b == ee {
        return 3;
    }
    ee = rot(ee);
    if b == ee {
        return 2;
    }
    ee = rot(ee);
    if b == ee {
        return 1;
    }
    panic!("can't defin rot");
}

fn set_rot(
    e: &(usize, String, (usize, usize, usize, usize)),
    rot: usize,
) -> (usize, String, usize, (usize, usize, usize, usize)) {
    (e.0, e.1.clone(), rot, (e.2 .0, e.2 .1, e.2 .2, e.2 .3))
}

fn set_id(ch: &mut (usize, String, usize, (usize, usize, usize, usize)), id: usize) {
    ch.0 = id;
}

fn get_by_dir(e: &(usize, String, usize, (usize, usize, usize, usize)), dir: usize) -> usize {
    match dir {
        0 => e.3 .0,
        1 => e.3 .1,
        2 => e.3 .2,
        3 => e.3 .3,
        _ => panic!("unexpected dir"),
    }
}

fn find_match(
    id: usize,
    dir: usize,
    rot_set: &Vec<(usize, String, usize, (usize, usize, usize, usize))>,
) -> Vec<usize> {
    let op_dir = match dir {
        0 => 3,
        1 => 2,
        2 => 1,
        3 => 0,
        _ => panic!("unexpected dir"),
    };
    let mut matches: Vec<usize> = vec![];

    for i in 0..rot_set.len() {
        if get_by_dir(&rot_set[id], dir) == get_by_dir(&rot_set[i], op_dir) {
            matches.push(i);
        }
    }

    matches
}

fn usize_vec_to_string(w: Vec<usize>) -> String {
    let mut out = String::new();
    for v in 0..w.len() - 1 {
        out.push_str(&w[v].to_string()[..]);
        out.push(' ');
    }
    out.push_str(&w[w.len() - 1].to_string()[..]);
    out.push('\n');
    out
}

fn to_tilesdata(tiles_prop: Vec<(usize, String, (usize, usize, usize, usize))>) -> String {
    let mut hash_set: HashSet<(usize, String, (usize, usize, usize, usize))> = HashSet::new();
    let mut rot_set: Vec<(usize, String, usize, (usize, usize, usize, usize))> = vec![];
    let mut tep;
    for i in 0..tiles_prop.len() {
        tep = tiles_prop[i].clone();
        hash_set.insert(tep.clone());
        tep = rot(tep);
        hash_set.insert(tep.clone());
        tep = rot(tep);
        hash_set.insert(tep.clone());
        tep = rot(tep);
        hash_set.insert(tep.clone());
    }
    for elem in &hash_set {
        rot_set.push(set_rot(elem, get_rot(tiles_prop[elem.0].clone(), elem)));
    }
    let tile_types: usize = rot_set.len();

    for r in 0..tile_types {
        set_id(&mut rot_set[r], r);
        println!("{:?}", rot_set[r]);
    }

    let mut diag: Vec<usize> = vec![];
    for i in 0..tile_types {
        diag.push(i);
    }
    let diag_string = &usize_vec_to_string(diag.clone())[..];

    let mut sst: String = String::new();

    for i in 0..tile_types {
        sst.push_str(&rot_set[i].0.to_string()[..]);
        sst.push('\n');
        sst.push_str(&rot_set[i].1[..]);
        sst.push(' ');
        sst.push_str(&rot_set[i].2.to_string()[..]);
        sst.push('\n');
        sst.push_str(diag_string);
        sst.push_str(&usize_vec_to_string(find_match(i, 0, &rot_set))[..]);
        sst.push_str(diag_string);
        sst.push_str(&usize_vec_to_string(find_match(i, 1, &rot_set))[..]);
        sst.push_str(&usize_vec_to_string(find_match(i, 2, &rot_set))[..]);
        sst.push_str(diag_string);
        sst.push_str(&usize_vec_to_string(find_match(i, 3, &rot_set))[..]);
        sst.push_str(diag_string);
        sst.push('\n');
    }
    //println!("{}", sst);
    //println!("{:?}", find_match(0, 0, &rot_set));
    sst
}

fn main() {
    let s = to_tilesdata(read_file());

    let _ = remove_file("tiles-data.txt");
    let mut file = File::create("tiles-data.txt").expect("Error encountered while creating file!");

    file.write_all((&s[..]).as_bytes())
        .expect("Error while writing to file");
}
