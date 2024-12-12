// Helpers for working with 2d indices of the Grid struct.

pub type Idx2 = [usize; 2];

pub const DIRECTIONS: [Idx2; 4] = [[0, 1], [0, usize::MAX], [1, 0], [usize::MAX, 0]];

pub fn add([i1, j1]: Idx2, [i2, j2]: Idx2) -> Idx2 {
    [i1.wrapping_add(i2), j1.wrapping_add(j2)]
}

pub fn sub([i1, j1]: Idx2, [i2, j2]: Idx2) -> Idx2 {
    [i1.wrapping_sub(i2), j1.wrapping_sub(j2)]
}

pub fn neg([i, j]: Idx2) -> Idx2 {
    [0_usize.wrapping_sub(i), 0_usize.wrapping_sub(j)]
}

pub fn scalar_mul([i, j]: Idx2, n: usize) -> Idx2 {
    [i.wrapping_mul(n), j.wrapping_mul(n)]
}

pub fn rotate_clockwise([i, j]: Idx2) -> Idx2 {
    [j, 0_usize.wrapping_sub(i)]
}

pub fn rotate_counterclockwise([i, j]: Idx2) -> Idx2 {
    [0_usize.wrapping_sub(j), i]
}
