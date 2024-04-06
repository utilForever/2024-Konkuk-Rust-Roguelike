pub fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed_matrix = [[0; 3]; 3]; 

    for i in 0..3 {
        for j in 0..3 {
            transposed_matrix[j][i] = matrix[i][j]; 
        }
    }

    transposed_matrix 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose_3x3_unit() {
        let matrix = [
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, 1],
        ];

        let ret = transpose(matrix);
        assert_eq!(ret, [
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, 1],
        ]);
    }

    #[test]
    fn transpose_3x3_random() {
        let matrix = [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
        ];

        let ret = transpose(matrix);
        assert_eq!(ret, [
            [1, 4, 7],
            [2, 5, 8],
            [3, 6, 9],
        ]);
    }
}
