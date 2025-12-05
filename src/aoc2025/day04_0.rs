use ndarray::{array, Array2};
use ndarray_conv::{ConvExt, ConvMode, PaddingMode};

#[derive(aoc_macros::AocDay)]
#[output_type("usize")]
#[expected_short(Some(13))]
#[expected_long(Some(1445))]
pub struct Day;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &'static [u8]) -> usize {
    let grid: Vec<Vec<u8>> = input
        .trim_ascii()
        .split(|c| *c == b'\n')
        .map(|l| l.iter().map(|c| if *c == b'@' { 1 } else { 0 }).collect())
        .collect();

    let matrix: Array2<u8> =
        Array2::from_shape_fn((grid.len(), grid[0].len()), |(i, j)| grid[i][j]);
    let kernel: Array2<u8> = array![[1, 1, 1], [1, 1, 1], [1, 1, 1],];

    let out = matrix
        .conv(&kernel, ConvMode::Same, PaddingMode::Zeros)
        .unwrap();

    out.iter()
        .zip(matrix)
        .filter(|(v, p)| *p == 1 && **v < 5)
        .count()
}
