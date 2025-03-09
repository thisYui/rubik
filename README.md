rubik_solver/
├── src/
│   ├── main.rs             # Chương trình chính, chạy thuật toán giải Rubik
│   ├── cube.rs             # Định nghĩa cấu trúc dữ liệu cho khối Rubik
│   ├── solver.rs           # Cài đặt thuật toán giải
│   ├── parser.rs           # Chuyển trạng thái Rubik từ chuỗi sang cấu trúc dữ liệu
│   └── utils.rs            # Các hàm hỗ trợ khác
│
├── tests/
│   ├── cube_tests.rs       # Kiểm tra chức năng của cube.rs
│   ├── solver_tests.rs     # Kiểm tra thuật toán giải
│   └── integration_tests.rs # Kiểm thử tích hợp toàn bộ hệ thống
│
├── Cargo.toml              # Cấu hình dự án Rust
├── Cargo.lock              # Khóa phiên bản dependency
└── README.md               # Hướng dẫn sử dụng và mô tả thuật toán
