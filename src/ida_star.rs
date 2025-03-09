use crate::cube::{Cube, Face, RotationDirection};
use crate::heuristic::*;
use std::collections::HashSet;

// Using IDA* algorithms to find answer
pub fn ida_star(start: &Cube) -> Option<Vec<(Face, RotationDirection)>> {
    let mut bound = combined_heuristic(start);
    let mut path = Vec::new();
    let depth_limit: usize = 5;
    let mut sum = 0;
    loop {
        println!("Current bound: {}", bound);
        let t = search(start, 0, bound, &mut path, &mut HashSet::new(), depth_limit, &mut sum);

        if t == 0 {
            println!("Solution found!");
            return Some(path);
        }
        if t == usize::MAX {
            println!("No solution found.");
            return None;
        }

        bound = t; // Update limit to find
    }
}

/// Tìm kiếm sâu có giới hạn trong IDA*
fn search(
    cube: &Cube,
    g: usize,
    bound: usize,
    path: &mut Vec<(Face, RotationDirection)>,
    visited: &mut HashSet<Cube>,
    depth_limit: usize,
    sum: &mut i32,
) -> usize {
    if g > depth_limit {
        return usize::MAX;
    }

    let f = g + combined_heuristic(cube);
    if f > bound {
        // When value exceeds the bound, return the value
        // This not solution
        return f;
    }
    if cube.is_solved() {
        // This is solution
        return 0;
    }

    *sum += 1;

    println!("Current depth: {}, f: {}, bound: {}, sum: {}", g, f, bound, sum);

    let mut min_cost = usize::MAX;

    // Create all possible moves
    let mut moves: Vec<_> = Cube::MOVES.iter()
        .map(|&(face, dir)| {
            let mut new_cube = *cube;
            new_cube.rotate(face, dir);
            let new_h = combined_heuristic(&new_cube);
            ((face, dir), new_cube, new_h)
        })
        .collect();

    // Sorting by heuristic value
    moves.sort_by_key(|&(_, _, new_h)| new_h);

    for ((face, dir), new_cube, _) in moves {
        // Remove the move that is the opposite of the previous move
        if !path.is_empty()
            && path.last().unwrap().0 == face { // Don't rotate the same face before
            continue;
        }

        if visited.contains(&new_cube) {
            continue; // Avoid repeating approved status
        }

        path.push((face, dir));
        /// If the new state is visited successfully or not successfully
        /// Both are added to the visited set because if is successful, it will be the solution
        /// If not successful, it will be the state that has been visited and don't need to visit again
        visited.insert(new_cube);

        let t = search(&new_cube, g + 1, bound, path, visited, depth_limit, sum);

        if t == 0 {
            return 0; // Tìm thấy lời giải
        }

        path.pop();
        visited.remove(&new_cube);
        min_cost = min_cost.min(t);
    }

    min_cost
}