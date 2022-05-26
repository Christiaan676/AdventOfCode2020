
use std::{
        fmt,
        fmt::{Display, Formatter},
    };



/// Matrix with M rows and N colums of typw T
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Matrix<T, const M: usize, const N: usize> {
    data: [[T; N]; M],
}

impl<T: Default + Copy, const M: usize, const N: usize> Default for Matrix<T, M, N> {
    fn default() -> Self {
        Self {
            data: [[T::default(); N]; M],
        }
    }
}

impl<T: Copy, const M: usize, const N: usize> Matrix<T, M, N> {
    fn new(default: T) -> Self {
        Self {
            data: [[default; N]; M],
        }
    }
}

impl<T, const M: usize, const N: usize> From<[[T; N]; M]> for Matrix<T, M, N> {
    fn from (data: [[T; N]; M]) -> Self {
        Self {
            data
        }
    }
}

impl<T, const M: usize, const N: usize> Display for Matrix<T, M, N> where T: Display{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for line in self.data.iter() {
            for item in line {
                item.fmt(f);
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl<T, const M: usize, const N: usize> AsRef<[[T; N]; M]> for Matrix<T, M, N> {
    fn as_ref(&self) -> &[[T; N]; M] {
        &self.data
    }
}

impl<T, const N: usize> Matrix<T, N, N> {

    pub fn rotate_l90(&mut self) {
        // ring numbering:
        // 0000000
        // 0111110
        // 0122210
        // 012*210
        // 0122210
        // 0000000
        for ring in 0..(N/2) {
            // Start and finish index for this ring
            let start = ring;
            let finish = N - 1 - ring;
            
            // v is offset in the ring
            for n in  0..(N -1 - ring*2) {
                // Swap top and right
                self.swap((start + n, start), (finish, start + n));
                // Swap right and bottom
                self.swap((finish, start + n), (finish - n, finish));
                // Swap bottom and left
                self.swap((finish - n, finish), (start, finish - n));
            }
        }
    }

    // Swap the data of two points in the matrix
    fn swap(&mut self, (a_x, a_y): (usize, usize), (b_x, b_y): (usize, usize)) {
        // println!("a({}.{}) b({}.{})", a_x, a_y, b_x, b_y);
        if a_y == b_y {
            self.data[a_y].swap(a_x, b_x);
        } else if a_y > b_y {
            let (top, bot) = self.data.split_at_mut(a_y);
            std::mem::swap(&mut top[b_y][b_x], &mut bot[0][a_x]);
        } else {
            let (top, bot) = self.data.split_at_mut(b_y);
            std::mem::swap(&mut top[a_y][a_x], &mut bot[0][b_x]);
        }
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {

    pub fn flip_horizontal(&mut self) {
        for i in 0..(M/2) {
            self.data.swap(i, (M-1) - i);
        }
    }

    // fn colums(&self) -> impl Iterator<item=[T;N]]> {
    //     self.data.i
    // }

    // fn rows(&self) ->  impl Iterator<item=[T;M]> {

    // }
}

impl<T: Clone, const M: usize, const N: usize> Matrix<T, M, N> {

    pub fn colum(&self, i :usize) -> [T;M] {
        let mut data: [T; M];
        for m in 0..M {
            data[m] = self.data[m][i].clone();
        }
        data
    }

    pub fn row(&self, i :usize) -> [T;N] {
        self.data[i].clone()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_flip_h() {
        let mut a = Matrix::from([[1,1], [2, 2], [3,3]]);
        a.flip_horizontal();
        assert_eq!(a, Matrix::from([[3,3], [2, 2], [1,1]]));
    }

    
    #[test]
    fn test_rotate_l90() {
        let mut a = Matrix::from([[1,1,1], [2, 2, 2], [3,3, 3]]);
        a.rotate_l90();
        assert_eq!(a, Matrix::from([[1, 2, 3], [1, 2, 3], [1, 2, 3]]));
    }
}
