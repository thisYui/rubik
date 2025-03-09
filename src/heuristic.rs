use std::collections::{HashMap, VecDeque};
use std::sync::OnceLock;
use crate::cube::*;

static PDB: OnceLock<PatternDB> = OnceLock::new();

pub struct PatternDB {
    pub data: HashMap<Cube, usize>, // Map from a cube state to the heuristic value
}

pub fn get_pdb() -> &'static PatternDB {
    PDB.get_or_init(PatternDB::generate_pdb)
}

impl PatternDB {
    const DEPTH: usize = 7;
    const DEFAULT_VALUE: usize = 10;

    fn generate_pdb() -> Self {
        let mut pdb = HashMap::new();
        let mut queue = VecDeque::new();

        let solved_cube = Cube::new(None).expect("Error"); // Trạng thái ban đầu đã giải
        pdb.insert(solved_cube, 0);
        queue.push_back((solved_cube, 0));

        while let Some((cube, depth)) = queue.pop_front() {
            if depth >= Self::DEPTH {
                break;
            } // Giới hạn độ sâu

            for &(face, direction) in Cube::MOVES.iter() {
                let mut new_cube = cube; // Tránh clone, thay vào đó dùng giá trị cũ
                new_cube.apply_move(face, direction);

                if !pdb.contains_key(&new_cube) {
                    pdb.insert(new_cube, depth + 1);
                    queue.push_back((new_cube, depth + 1));
                }
            }
        }

        PatternDB { data: pdb }
    }
}

fn heuristic_pdb(cube: &Cube) -> usize {
    let pdb = get_pdb();
    *pdb.data.get(cube).unwrap_or(&PatternDB::DEFAULT_VALUE)
}

// Count the number of misplaced stickers
fn misplaced_stickers(cube: &Cube) -> usize {
    cube.state.iter()
        .enumerate()
        .filter(|(i, &color)| color != (i / Cube::FACE_SIZE) as u8)
        .count()
}

/**
The function caculates the Manhattan distance between the current position of each sticker and its solved position.
Idea:
- In 2D space, the Manhattan distance between two points is the sum of the absolute differences of their coordinates:
    d(p, q) = |p.x - q.x| + |p.y - q.y|
- With the Rubik's Cube, we caculate sum of all Manhattan distances between the current position of each sticker and its solved position.
- Because rotating a face of the cube can change some of the stickers' positions, the Manhattan distance doesn't heuristic correctly.
- But heuristic use for estimate the number of moves to solve the cube.
*/
fn manhattan_distance(cube: &Cube) -> usize {
    let mut total_distance = 0;

    for (i, &color) in cube.state.iter().enumerate() {
        let (x, y, z) = index_to_xyz(i); // Chuyển chỉ số thành tọa độ 3D
        let (cx, cy, cz) = correct_position(color, i); // Lấy tọa độ đúng của màu

        total_distance += (x as isize - cx as isize).abs()
                      + (y as isize - cy as isize).abs()
                      + (z as isize - cz as isize).abs();
    }

    total_distance as usize
}

// Hàm giúp chuyển chỉ số `i` thành tọa độ (x, y, z)
fn index_to_xyz(i: usize) -> (usize, usize, usize) {
    let x = (i % Cube::FACE_SIZE) % 3;
    let y = (i % Cube::FACE_SIZE) / 3;
    let z = i / Cube::FACE_SIZE;
    (x, y, z)
}

// Xác định vị trí đúng của một ô màu
fn correct_position(color: u8, index: usize) -> (usize, usize, usize) {
    let face = color as usize;
    let offset = index % Cube::FACE_SIZE;
    let x = offset % 3;
    let y = offset / 3;
    let z = face;
    (x, y, z)
}

// Number of faces that are not completely solved
fn unfinished_faces(cube: &Cube) -> usize {
    cube.state.chunks(Cube::FACE_SIZE)
        .filter(|face| face.iter().any(|&c| c != face[0]))
        .count()
}

// Number of edges that are not correctly placed
fn misplaced_edges(cube: &Cube) -> usize {
    Cube::EDGES.iter()
        .filter(|&&(a, b)| cube.state[a] != cube.state[b])
        .count()
}

fn edge_orientation_heuristic(cube: &Cube) -> usize {
    Cube::EDGES.iter()
        .filter(|&&(a, b)| (cube.state[a] / 9) != (cube.state[b] / 9)) // Kiểm tra orientation
        .count()
}

fn corner_orientation_heuristic(cube: &Cube) -> usize {
    Cube::CORNERS.iter()
        .filter(|&&(a, b, c)| {
            let colors = [cube.state[a], cube.state[b], cube.state[c]];
            let correct_colors = [(a / 9) as u8, (b / 9) as u8, (c / 9) as u8];
            colors != correct_colors
        })
        .count()
}

fn edge_permutation_heuristic(cube: &Cube) -> usize {
    Cube::EDGES.iter()
        .filter(|&&(a, b)| {
            let correct_a = (a / 9) as u8;
            let correct_b = (b / 9) as u8;
            (cube.state[a] / 9) != correct_a || (cube.state[b] / 9) != correct_b
        })
        .count()
}

fn corner_permutation_heuristic(cube: &Cube) -> usize {
    Cube::CORNERS.iter()
        .filter(|&&(a, b, c)| {
            let correct_a = (a / 9) as u8;
            let correct_b = (b / 9) as u8;
            let correct_c = (c / 9) as u8;
            (cube.state[a] / 9) != correct_a || (cube.state[b] / 9) != correct_b || (cube.state[c] / 9) != correct_c
        })
        .count()
}

fn parity_heuristic(cube: &Cube) -> usize {
    let mut inversions = 0;
    for i in 0..cube.state.len() {
        for j in (i + 1)..cube.state.len() {
            if cube.state[i] > cube.state[j] {
                inversions += 1;
            }
        }
    }
    inversions % 2 // Nếu lẻ thì trả về 1, chẵn trả về 0
}

pub fn combined_heuristic(cube: &Cube) -> usize {
    [
        misplaced_stickers(cube),
        manhattan_distance(cube),
        unfinished_faces(cube),
        misplaced_edges(cube),
        edge_orientation_heuristic(cube),
        corner_orientation_heuristic(cube),
        edge_permutation_heuristic(cube),
        corner_permutation_heuristic(cube),
        parity_heuristic(cube),
        heuristic_pdb(cube),
    ]
    .into_iter()
    .max()
    .unwrap_or(0) // Tránh lỗi nếu không có giá trị nào (trường hợp đặc biệt)
}