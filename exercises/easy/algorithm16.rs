/*
    Rotate Matrix 90 Degrees
    Given a 2D matrix, rotate it 90 degrees in place. 
    You need to perform the rotation without using any additional matrix storage.

    You need to implement the function `rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>)`.
    The function should rotate the input matrix in place.

    Hint: Consider rotating the matrix layer by layer, starting from the outermost layer and working your way inward.
*/

use std::fmt::{self, Display, Formatter};

pub fn rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>) {
    let rows = matrix.len();
    let cols = if rows > 0 { matrix[0].len() } else { 0 };
    // 创建一个新的矩阵用于存储旋转后的结果
    let mut rotated = vec![vec![0; rows]; cols];
    for i in 0..rows {
        for j in 0..cols {
            // 将原矩阵元素放到旋转后矩阵的对应位置
            rotated[j][rows - 1 - i] = matrix[i][j];
        }
    }
    // 将旋转后的矩阵复制回原矩阵
    *matrix = rotated;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix_1() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3],
        ]);
    }

    #[test]
    fn test_rotate_matrix_2() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![3, 1],
            vec![4, 2],
        ]);
    }

    #[test]
    fn test_rotate_matrix_3() {
        let mut matrix = vec![
            vec![1],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![1],
        ]);
    }

    #[test]
    fn test_rotate_matrix_4() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![5, 3, 1],
            vec![6, 4, 2],
        ]);
    }
}
