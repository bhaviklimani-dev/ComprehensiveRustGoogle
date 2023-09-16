// array
// fn main() {
//     let array = [10, 20, 30];
//     println!("array: {array:?}");
// }


// Rust lets you iterate over things like arrays and ranges using the for keyword:
// fn main() {
//     let array = [10, 20, 30];
//     print!("Iterating over array:");
//     for n in &array {
//         print!(" {n}");
//     }
//     println!();

//     print!("Iterating over range:");
//     for i in 0..3 {
//         print!(" {}", array[i]);
//     }
//     println!();
// }

// TODO: remove this when you're done with your implementation.
// #![allow(unused_variables, dead_code)]

// fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
//     unimplemented!()
// }

// fn pretty_print(matrix: &[[i32; 3]; 3]) {
//       println!("Matrix: {matrix:?}");
// }

// fn main() {
//     let matrix = [
//         [101, 102, 103], // <-- the comment makes rustfmt add a newline
//         [201, 202, 203],
//         [301, 302, 303],
//     ];

//     println!("matrix:");
//     pretty_print(&matrix);

//     let transposed = transpose(matrix);
//     println!("transposed:");
//     pretty_print(&transposed);
// }

// fn main() {
//     let x = {
//         let y = 10;
//         println!("y: {y}");
//         let z = {
//             let w = {
//                 3 + 4
//             };
//             println!("w: {w}");
//             y * w
//         };
//         println!("z: {z}");
//         z - y
//     };
//     println!("x: {x}");
// }

// fn double(x: i32) -> i32 {
//     x + x
// }

// fn main() {
//     println!("doubled: {}", double(7));
// }

