//! Computes cross correlation efficiently, using FFT.

use crate::fft;
use crate::math::*;
use std::array::IntoIter;

/// A structure prepared to perform cross correlations up to a given maximum size.
pub struct CrossCorrelation {
    base_size: usize,
    fft_size: usize,
    fft: fft::Prepared,
    buffer: Vec<Complex>,
}

impl CrossCorrelation {
    /// Allocate and prepare a cross correlation. `max_size` is the maximum size
    /// of either of the input arrays.
    pub fn new(max_size: usize) -> Self {
        let base_size = max_size.next_power_of_two();
        let fft_size = base_size * 2;
        CrossCorrelation {
            base_size,
            fft_size,
            fft: fft::Prepared::new(fft_size),
            buffer: vec![(0., 0.).into(); fft_size],
        }
    }

    /// Compute cross correlation including partially overlapping positions.
    /// Length of `a` and `b` must not exceed the maximum size given in `new`.
    /// Returns an interator of the results. The length of the result is `a.len() + b.len() - 1`.
    #[allow(dead_code)]
    pub fn compute(&mut self, a: &[Num], b: &[Num]) -> impl Iterator<Item = Num> + '_ {
        self.compute_raw(a, b);
        // The beginning of the result is read from the end of the buffer, rest normally
        // from the beginning of the buffer.
        // This is to correctly output the partially overlapping positions on the left
        // as well.
        self.buffer[self.fft_size - b.len() + 1..]
            .iter()
            .chain(self.buffer[..a.len()].iter())
            .map(|z| z.real)
    }

    /// Compute cross correlation excluding partially overlapping positions.
    /// Length of `a` and `b` must not exceed the maximum size given in `new`.
    /// Returns an interator of the results. The length of the result is `a.len() - b.len() + 1`.
    pub fn compute_truncated(&mut self, a: &[Num], b: &[Num]) -> impl Iterator<Item = Num> + '_ {
        assert!(a.len() >= b.len());
        self.compute_raw(a, b);
        self.buffer[..a.len() - b.len() + 1].iter().map(|z| z.real)
    }

    /// Performs the computation without extracting results from the `buffer`.
    fn compute_raw(&mut self, a: &[Num], b: &[Num]) {
        assert!(a.len() <= self.base_size);
        assert!(b.len() <= self.base_size);
        // We use a trick to perform FFTs for two non-complex signals at once.
        //
        // The arrays are packed as z[k] = (a[k] + i*b[k]), then z' = fft(z) is performed.
        // Now z' = a' + i*b' where a' = fft(a) and b' = fft(b).
        //
        // For all frequencies w, a[w] and b[w] can be solved when
        // a[w] + i*b[w] and a[-w] + i*b[-w] are known.
        //
        // The cross correlation requires computing a[w] * conj(b[w]) for each frequency,
        // and then taking the inverse FFT.
        use std::iter;
        for (zk, (ak, bk)) in self.buffer.iter_mut().zip(
            a.iter()
                .cloned()
                .chain(iter::repeat(0.))
                .zip(b.iter().cloned().chain(iter::repeat(0.))),
        ) {
            *zk = (ak, bk).into();
        }

        self.fft.fft(&mut self.buffer);

        // Split buffer into left and right half because we need to iterate both at once.
        let (left, right) = self.buffer.split_at_mut(self.fft_size / 2);
        // a[0] = a[-0] and a[N/2] = a[-N/2] so they must be handled separately.
        for zw in IntoIter::new([&mut left[0], &mut right[0]]) {
            let Complex { real: aw, imag: bw } = *zw;
            *zw = Complex {
                real: aw * bw,
                imag: 0.,
            };
        }
        for (zw, zmw) in left[1..].iter_mut().zip(right[1..].iter_mut().rev()) {
            // zw = z[w], zmw = z[-w]
            // Solve a[w] and b[w] first
            let aw = (*zw + zmw.conj()) / 2.;
            let bw = (zmw.conj() - *zw) * IMAG_UNIT / 2.;
            // Then store a[w] * conj(b[w]) and conj(a[w] * conj(b[w]))
            let res = aw * bw.conj();
            *zw = res;
            *zmw = res.conj();
        }

        self.fft.ifft(&mut self.buffer);
    }
}
