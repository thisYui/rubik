/**
* Diameter of the Rubik's Cube Group is 20
* The maximum number of moves to solve the Rubik's Cube is 20 if the move be chosen is the best move
* In rubik space have more 43 trillion states
*/

use crate::cube::{Cube, Face, RotationDirection};
use crate::heuristic::*;
use std::collections::HashSet;

// Using IDA* algorithms to find answer
pub fn ida_star(cube: &Cube) -> Option<Vec<(Face, RotationDirection)>> {
    let mut bound = combined_heuristic(cube);
    let mut path = Vec::new();
    let mut best_state = *cube; // Lưu trạng thái tốt nhất tìm thấy

    loop {
        let result = search(cube, 0, bound, &mut path, &mut HashSet::new(), 5, &mut best_state);

        if result == 0 {
            return Some(path); // Found solution
        }
        if result == usize::MAX {
            return None; // No solution
        }

        bound = result; // Update bound
    }
}

/// Find the solution using IDA* algorithms
fn search(
    cube: &Cube,
    g: usize,
    bound: usize,
    path: &mut Vec<(Face, RotationDirection)>,
    visited: &mut HashSet<Cube>,
    depth_limit: usize,
    best_state: &mut Cube,
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
        // Don't rotate the same face twice
        if !path.is_empty() && path.last().unwrap().0 == face {
            continue;
        }

        // Avoid repeating states
        if visited.contains(&new_cube) {
            continue;
        }

        path.push((face, dir));
        // If the new state is visited successfully or not successfully
        // Both are added to the visited set because if is successful, it will be the solution
        // If not successful, it will be the state that has been visited and don't need to visit again
        visited.insert(new_cube);

        let t = search(&new_cube, g + 1, bound, path, visited, depth_limit, best_state);

        if t < min_cost {
            min_cost = t;
            *best_state = *cube;
        }

        if t == 0 {
            return 0; // Find solution
        }

        path.pop();
        //visited.remove(&new_cube);
    }

    min_cost
}
