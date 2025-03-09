pub mod cube;
pub mod ida_star;
pub mod heuristic;

use crate::cube::*;
use crate::ida_star::*;

fn main() {
    // 0 - trắng
    // 1 - đỏ
    // 2 - xanh lá cây
    // 3 - xanh dương
    // 4 - cam
    // 5 - vàng

    let rubik : [u8; 54] = [
        3, 2, 3, 4, 0, 2, 2, 4, 0,
        1, 4, 2, 0, 5, 1, 1, 0, 4,
        1, 5, 5, 5, 2, 0, 0, 2, 3,
        1, 2, 4, 1, 3, 1, 2, 3, 3,
        5, 5, 2, 3, 4, 3, 5, 3, 4,
        4, 0, 0, 4, 1, 5, 0, 1, 5,
    ];


    // Khởi tạo một trạng thái Cube ngẫu nhiên
    let mut cube = Cube::new(Some(& rubik)).expect("Error initializing cube");
    println!("Initial scrambled cube:");
    cube.print(); // Giả sử có phương thức hiển thị trạng thái cube

    // Giải khối Rubik bằng IDA*
    if let Some(solution) = ida_star(&cube) {
        println!("Solution found with {} moves:\n", solution.len());

        for (i, (face, direction)) in solution.iter().enumerate() {
            println!("Step {}: Rotate {:?} {:?}", i + 1, face, direction);
            cube.rotate(*face, *direction);
            cube.print(); // Hiển thị trạng thái Cube sau mỗi bước xoay
        }

        println!("Final solved cube:");
        cube.print();
    } else {
        println!("No solution found.");
    }
}
