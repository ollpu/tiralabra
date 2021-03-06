use crate::math::*;

/// Implements the FFT, i.e. Fast Fourier Transform, and its inverse.
///
/// This structure is initialized beforehand, and contains twiddle-factors for
/// a specific transform size.
pub struct Fft {
    size: usize,
    twiddle_factors: Vec<Complex>,
}

impl Fft {
    /// Prepare FFT. Size has to be a power of two.
    pub fn new(size: usize) -> Self {
        assert!(size.count_ones() == 1);
        let half_size = size / 2;
        let half_size_inverse = 1. / half_size as Num;
        let twiddle_factors = (0..half_size)
            .map(|i| Complex::euler(-(i as Num) * half_size_inverse * PI))
            .collect();
        Fft {
            size,
            twiddle_factors,
        }
    }

    /// Perform the transform. The size of the array has to be the same as what
    /// this instance was prepared with.
    pub fn fft(&self, array: &mut [Complex]) {
        assert!(array.len() == self.size);
        permute_binary_reverse(array);
        // The "butterfly figure" - the main computation. Performed for each size
        // 2, 4, 8 ... in increasing order.
        for half_width in (0..).map(|e| 1 << e).take_while(|w| *w < self.size) {
            let width = 2 * half_width;
            let twiddle_step = self.size / width;
            for pos in (0..self.size).step_by(width) {
                for i in 0..half_width {
                    let l = array[pos + i];
                    let r = array[pos + half_width + i];
                    // This expression is taken from the precomputed array instead:
                    // Complex::euler(-(i as Num) * PI / half_width as Num)
                    let r = r * self.twiddle_factors[i * twiddle_step];
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

/// Permutes an array such that the binary representation of each index is
/// reversed.
///
/// Assumes that `array` has a length that is a power of two.
fn permute_binary_reverse<T>(array: &mut [T]) {
    let shift_amount = array.len().leading_zeros() + 1;
    for index in 0..array.len() {
        let reversed = index.reverse_bits() >> shift_amount;
        if reversed > index {
            array.swap(index, reversed);
        }
    }
}
