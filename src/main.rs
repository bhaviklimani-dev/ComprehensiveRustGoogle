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

// fn main() {
//     let v = vec![10, 20, 30];

//     for x in v {
//         println!("x: {x}");
//     }
    
//     for i in (0..10).step_by(3) {
//         println!("i: {i}");
//     }
// }

// fn main() {
//     let mut x = 10;
//     while x != 1 {
//         x = if x % 2 == 0 {
//             x / 2
//         } else {
//             3 * x + 1
//         };
//     }
//     println!("Final x: {x}");
// }

// fn main() {
//     let v = vec![4, 20, 30];
//     let mut iter = v.into_iter();
//     'outer: while let Some(x) = iter.next() {
//         //println!("x: {x}");
//         let mut i = 0;
//         while i < x {
//           i += 1;
//           let mut q = 0;
//           while q < i {
//             println!("{x}, {i}, {q}");
//             q += 1;
//             if q == 3 {
//                 break 'outer;
//             }
//           }
//         }
//     }
// }


// fn main() {
//     let mut x = 10;
//     loop {
//         x = if x % 2 == 0 {
//           x / 2
//         } else {
//             3 * x + 1
//         };
//         if x == 1 {
//             break;
//         }
//     }
//     println!("Final x: {x}");
// }

fn main() {
    let x: i32 = 10;
    println!("x: {x}");
    // x = 20;
    // println!("x: {x}");
}