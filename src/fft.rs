//! This module implements the FFT, i.e. Fast Fourier Transform, and its inverse.

use crate::math::*;

/// A structure that is initialized beforehand, and contains twiddle-factors for
/// a specific transform size. For now, nothing is saved here and everything is
/// computed in the main routine.
pub struct Prepared {
    size: usize,
}

impl Prepared {
    /// Prepare FFT. Size has to be a power of two.
    pub fn new(size: usize) -> Self {
        assert!(size.count_ones() == 1);
        Prepared { size }
    }

    /// Perform the transform. The size of the array has to be the same as what
    /// this instance was prepared with.
    pub fn fft(&self, array: &mut [Complex]) {
        assert!(array.len() == self.size);
        // Interlacing. Indexes are permuted such that their binary representation is reversed.
        for index in 0..self.size {
            let reversed = index.reverse_bits() >> (self.size.leading_zeros() + 1);
            if reversed > index {
                array.swap(index, reversed);
            }
        }
        // The "butterfly figure" - the main computation. Performed for each size
        // 2, 4, 8 ... in increasing order.
        for half_width in (0..).map(|e| 1 << e).take_while(|w| *w < self.size) {
            for pos in (0..self.size).step_by(2 * half_width) {
                for i in 0..half_width {
                    let l = array[pos + i];
                    let r = array[pos + half_width + i];
                    let r = r * Complex::euler(-(i as Num) * PI / half_width as Num);
                    array[pos + i] = l + r;
                    array[pos + half_width + i] = l - r;
                }
            }
        }
    }

    /// Perform the inverse transform. The size of the array has to be the same as
    /// what this instance was prepared with.
    pub fn ifft(&self, array: &mut [Complex]) {
        assert!(array.len() == self.size);
        self.fft(array);
        // The inverse transform is otherwise identical, except the indexes of
        // the result have to be inverted modulo size, in practive meaning that
        // the range [1..size[ is reversed.
        for index in 1..(self.size / 2) {
            array.swap(index, self.size - index);
        }
        // ...and finally, the result is multiplied with a normalization factor
        // so that the inverse transform actually restores the original array.
        for z in array.iter_mut() {
            *z = *z / self.size as Num;
        }
    }
}
