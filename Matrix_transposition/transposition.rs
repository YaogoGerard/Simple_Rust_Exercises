/*
Arrays can contain other arrays:
let array = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
What is the type of this variable?
Use an array such as the above to write a function transpose which will transpose a matrix
(turn rows into columns):
           ⎛⎡1 2 3⎤⎞
"transpose"⎜⎢4 5 6⎥⎟
           ⎝⎣7 8 9⎦⎠

            ⎡1 4 7⎤
"=="        ⎢2 5 8⎥
            ⎣3 6 9⎦
Hard-code both functions to operate on 3 × 3 matrices.
 */

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut res = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            res[j][i] = matrix[i][j];
        }
    }
    res
}

#[test]
fn test_transpose() {
    let matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [[101, 201, 301], [102, 202, 302], [103, 203, 303],]
    );
}
fn main() {
    let matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];
    println!("matrix: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("transposed: {:#?}", transposed);
}
