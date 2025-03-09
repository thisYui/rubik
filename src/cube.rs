#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Face {
    // The value of each face is the index of the first sticker in the state array
    U = 0,  // Up
    D = 9,  // Down
    R = 18,  // Right
    L = 27,  // Left
    F = 36,  // Front
    B = 45,  // Back
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RotationDirection {
    Clockwise = 1,        // Rotate 90° turn clockwise 1 time
    CounterClockwise = 3, // 3 times of clockwise rotation is equal to 1 time of counter-clockwise rotation
    DoubleTurn = 2,       // Rotate 180° (2 times of clockwise rotation)
}

impl RotationDirection {
    pub fn opposite(&self) -> Self {
        match self {
            RotationDirection::Clockwise => RotationDirection::CounterClockwise,
            RotationDirection::CounterClockwise => RotationDirection::Clockwise,
            RotationDirection::DoubleTurn => RotationDirection::DoubleTurn,
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Copy)]
pub struct Cube {
    /*  6 face and 9 stickers per face converted to 54 elements array
        Color: 0 = White, 1 = Yellow, 2 = Red, 3 = Orange, 4 = Green, 5 = Blue
        Face order: U, D, R, L, F, B
        Face Up (U): 0 - 8 in state
        Face Down (D): 9 - 17 in state
        Face Right (R): 18 - 26 in state
        Face Left (L): 27 - 35 in state
        Face Front (F): 36 - 44 in state
        Face Back (B): 45 - 53 in state
    */

    pub state: [u8; 54],  // u8 - 8 bit unsigned integer
}

impl Cube {
    pub const CELL_COUNT: usize = 54;
    pub const FACE_SIZE: usize = 9;
    pub const EDGE_SIZE: usize = 3;

    pub const EDGES: [(usize, usize); 24] = [
        (1, 3), (1, 5), (3, 7), (5, 7), // Face U
        (10, 12), (10, 14), (12, 16), (14, 16), // Face D
        (19, 21), (19, 23), (21, 25), (23, 25), // Face R
        (28, 30), (28, 32), (30, 34), (32, 34), // Face L
        (37, 39), (37, 41), (39, 43), (41, 43), // Face F
        (46, 48), (46, 50), (48, 52), (50, 52), // Face B
    ];

    pub const CORNERS: [(usize, usize, usize); 24] = [
        (0, 2, 6), (0, 8, 6), (2, 6, 8), (2, 0, 8), // Face U
        (11, 15, 17), (11, 17, 13), (15, 17, 13), (15, 13, 11), // Face D
        (18, 24, 26), (18, 26, 20), (24, 26, 20), (24, 20, 18), // Face R
        (27, 33, 35), (27, 35, 29), (33, 35, 29), (33, 29, 27), // Face L
        (36, 42, 44), (36, 44, 38), (42, 44, 38), (42, 38, 36), // Face F
        (45, 51, 53), (45, 53, 47), (51, 53, 47), (51, 47, 45), // Face B
    ];

    pub const MOVES: [(Face, RotationDirection); 18] = [
        (Face::U, RotationDirection::Clockwise),
        (Face::U, RotationDirection::CounterClockwise),
        (Face::U, RotationDirection::DoubleTurn),
        (Face::D, RotationDirection::Clockwise),
        (Face::D, RotationDirection::CounterClockwise),
        (Face::D, RotationDirection::DoubleTurn),
        (Face::R, RotationDirection::Clockwise),
        (Face::R, RotationDirection::CounterClockwise),
        (Face::R, RotationDirection::DoubleTurn),
        (Face::L, RotationDirection::Clockwise),
        (Face::L, RotationDirection::CounterClockwise),
        (Face::L, RotationDirection::DoubleTurn),
        (Face::F, RotationDirection::Clockwise),
        (Face::F, RotationDirection::CounterClockwise),
        (Face::F, RotationDirection::DoubleTurn),
        (Face::B, RotationDirection::Clockwise),
        (Face::B, RotationDirection::CounterClockwise),
        (Face::B, RotationDirection::DoubleTurn),
    ];

    pub fn new(state: Option<&[u8]>) -> Result<Self, String> {
    let cube_state = match state {
        Some(s) => {
            if s.len() != Self::CELL_COUNT {
                return Err(format!("Invalid state length: {}", s.len()));
            }
            let mut temp_state = [0; Self::CELL_COUNT];
            temp_state.copy_from_slice(s);
            temp_state
        }
        None => Self::default_state(), // Nếu None, dùng trạng thái đã giải
    };

    Ok(Self { state: cube_state })
}

    // Hàm trả về trạng thái đã giải của Rubik
    fn default_state() -> [u8; Self::CELL_COUNT] {
        // Giả sử trạng thái mặc định là một dãy số nào đó đại diện cho Rubik đã giải
        let mut default_state = [0; Self::CELL_COUNT];
        for (i, val) in default_state.iter_mut().enumerate() {
            *val = (i / Self::FACE_SIZE) as u8; // Mỗi mặt của Rubik có 9 ô, giả sử màu được đánh số từ 0-5
        }
        default_state
    }


    // Print state of the cube
    pub fn print(&self) {
        let faces = ["U", "D", "R", "L", "F", "B"];
        for (i, face) in faces.iter().enumerate() {
            println!("{} Face:", face);
            for row in 0..Self::EDGE_SIZE {
                println!("{:?}", &self.state[i * Self::FACE_SIZE
                    + row * Self::EDGE_SIZE..i * Self::FACE_SIZE
                    + (row + 1) * Self::EDGE_SIZE]);
            }
            println!();
        }
    }

    pub fn is_solved(&self) -> bool {
        self.state.chunks(Self::FACE_SIZE). // Traverse the cube state by face
            all(|face| face.iter().   // Traverse the face by sticker
                all(|&c| c == face[0])) // Check if all stickers in the face have the same color
    }

    // Rotate the face clockwise
    fn rotate_face_clockwise(&mut self, face: Face) {
        /** Example:
        * Before rotate:      After rotate:
             |1 2 3|             |7 4 1|
             |4 5 6|     =>      |8 5 2|
             |7 8 9|             |9 6 3|

        In face be rotated, the row '1 2 3' will be rotated to column '1 2 3'.
        Number 5 in the center of the face will not change.
        Like the matrix rotation, we can rotate the face by 90° clockwise by swapping the elements.

        **/

        let start = face as usize;// Transport enum Face to usize
        let mut new_face = [0; Self::FACE_SIZE];

        for i in 0..Self::EDGE_SIZE {
            for j in 0..Self::EDGE_SIZE {
                let old_idx = start + i * Self::EDGE_SIZE + j;
                let new_idx = start + j * Self::EDGE_SIZE + (Self::EDGE_SIZE - 1 - i);
                new_face[new_idx - start] = self.state[old_idx];
            }
        }

        self.state[start..start + Self::FACE_SIZE].copy_from_slice(&new_face);
    }

    // Function to rotate the adjacent faces
    fn rotate_adjacent(&mut self, face: Face) {
        /** Example:
          Before rotate:          After rotate:
                0 1 2                  2 1 0
             0 |_ _ _| 4            7 |_ _ _| 0
             1 |_ _ _| 5    =>      8 |_ _ _| 1
             2 |_ _ _| 6            9 |_ _ _| 2
                7 8 9                 6 5 4

        Function rotate_face_clockwise will rotate the face clockwise.
        - You can see row '0 1 2' in up square will be rotated to column '0 1 2' in right square.
        - And column '4 5 6' in right square will be rotated to row '6 5 4' in down square.

        Example with Up face (U):
        - Face Front (F): indices 36,37,38
        - Face Right (R): indices 18,19,20
        - Face Back  (B): indices 45,46,47
        - Face Left  (L): indices 27,28,29

        Rules for rotation:
            F.top <- L.top
            L.top <- B.top
            B.top <- R.top
            R.top <- F.top (temp)

        In Face (example with U face):
                0 1 2
                3 4 5
                6 7 8
        - 0 1 2: Row is top of face -> First 3 elements of the face is top
        - 6 7 8: Row is bottom of face -> Last 3 elements of the face is bottom
        - 0 3 6: Column is left of face -> First element of each row is left
        - 2 5 8: Column is right of face -> Last element of each row is right
        **/

        let indices = match face {
            // Define the indices of the stickers in the adjacent faces
            // Order of adjacent faces in array turn clockwise
            Face::U => [
                [36, 37, 38], // F.top
                [27, 28, 29], // L.top
                [45, 46, 47], // B.top
                [18, 19, 20], // R.top
            ],
            Face::D => [
                [42, 43, 44], // F.bottom
                [24, 25, 26], // R.bottom
                [51, 52, 53], // B.bottom
                [33, 34, 35], // L.bottom
            ],
            Face::R => [
                [38, 41, 44], // F.right
                [20, 23, 26], // U.right
                [47, 50, 53], // B.left
                [9, 12, 15],  // D.right
            ],
            Face::L => [
                [36, 39, 42], // F.left
                [11, 14, 17], // D.left
                [45, 48, 51], // B.right
                [0, 3, 6],    // U.left
            ],
            Face::F => [
                [6, 7, 8],    // U.bottom
                [27, 30, 33], // L.right
                [15, 16, 17], // D.top
                [18, 21, 24], // R.left
            ],
            Face::B => [
                [0, 1, 2],    // U.top
                [9, 10, 11],  // D.bottom
                [29, 32, 35], // L.left
                [51, 48, 45], // R.right
            ],
        };

        let temp = [self.state[indices[0][0]], self.state[indices[0][1]], self.state[indices[0][2]]];

        for i in 0..Self::EDGE_SIZE {
            self.state[indices[0][i]] = self.state[indices[3][i]];
            self.state[indices[3][i]] = self.state[indices[2][i]];
            self.state[indices[2][i]] = self.state[indices[1][i]];
            self.state[indices[1][i]] = temp[i];
        }
    }

    // Function to rotate the face
    // face: Face need to rotate
    // direction: diriction rotate (Clockwise, CounterClockwise, DoubleTurn)
    pub fn rotate(&mut self, face: Face, direction: RotationDirection) {
        /** Example:
        * Before rotate:          After rotate:
                0 1 2                  2 1 0
             0 |1 2 3| 4            7 |7 4 1| 0
             1 |4 5 6| 5    =>      8 |8 5 2| 1
             2 |7 8 9| 6            9 |9 6 3| 2
                7 8 9                 6 5 4

        Function rotate_face_clockwise will rotate the face clockwise.

        Identifying the number of times to rotate the face clockwise
        - Clockwise: 1 time
        - CounterClockwise: 3 times (because 3 times of clockwise rotation is equal to 1 time of counter-clockwise rotation)
        - DoubleTurn: 2 times (180°)
        **/
        let times = direction as usize;  // Transport enum RotationDirection to usize
        // Thực hiện quay theo số lần đã tính
        for _ in 0..times {
            // 1. Rotate sticker of the face
            self.rotate_face_clockwise(face);
            // 2. Update the adjacent faces
            self.rotate_adjacent(face);
        }
    }

    pub fn apply_move(&self, face: Face, direction: RotationDirection) -> Cube {
        let mut new_cube = self.clone(); // Make a copy of the cube
        new_cube.rotate(face, direction); // Rotate the face
        new_cube
    }
}
