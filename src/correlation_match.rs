pub mod parabolic_interpolation;
use parabolic_interpolation::parabolic_interpolation_minimum;

use crate::Float;
use crate::cross_correlation::CrossCorrelation;
use crate::util::IterWindows;

/// Finds the closest match of a shorter piece of audio from a larger piece of audio.
///
/// This structure is prepared to perform correlation matches up to a given size.
///
/// Design sketch in Finnish:
///
/// Algoritmi, joka etsii pidemmästä äänenpätkästä A sen kohdan, jossa
/// lyhyempi äänenpätkä B esiintyy kaikista lähimpänä.
///
/// Käyttötarkoituksena on oskilloskoopin näkymän vakautus. Silloin algoritmille
/// annettaisiin pätkä A uutta signaalia, ja pätkä B, joka vastaa viimeksi näytettyä
/// kuvaajaa. Algoritmi etsii uudesta signaalista sellaisen kohdan, jonka näyttämällä
/// kuvaaja muuttuu mahdollisimman vähän. Vakautettua kuvaajaa on toivottavasti helpompi
/// seurata, koska se ei liiku jatkuvasti taajuudesta riippuvalla tavalla.
///
/// Olkoon signaalit `A[0..n]` ja `B[0..m]`, `n >= 2m`.
/// Algoritmi etsii sellaisen aikasiirroksen t, jolla summa x:n yli
///
/// `w(x) * (A[x+t] - B[x])^2`
///
/// on minimaalinen. Tässä `w(x)` on painofunktio, jonka avulla voidaan esimerkiksi
/// painottaa oskilloskoopin näkymän keskikohtia enemmän kuin reunoja.
///
/// Jos tämä summa esitetään muodossa
///
/// `w(x) * A[x+t]^2 - 2(w(x) * B[x]) * A[x+t] + w(x) * B[x]^2`,
///
/// nähdään, että se voidaan laskea kahtena ristikorrelaationa (summat x:n yli muotoa
/// `f(x+t) * g(x)`) ja yhtenä suorana tulona (summa x:n yli muotoa `f(x) * g(x)`).
pub struct CorrelationMatch<Num> {
    max_size: usize,
    cross_correlation: CrossCorrelation<Num>,
    f_buffer: Vec<Num>,
    g_buffer: Vec<Num>,
    result_buffer: Vec<Num>,
    minima: Vec<(Num, Num)>,
}

impl<Num: Float> CorrelationMatch<Num> {
    /// Allocate and prepare a correlation match algorithm. `max_size` is
    /// the maximum size of any of the input arrays.
    pub fn new(max_size: usize) -> Self {
        CorrelationMatch {
            max_size,
            cross_correlation: CrossCorrelation::new(max_size),
            f_buffer: vec![Num::zero(); max_size],
            g_buffer: vec![Num::zero(); max_size],
            result_buffer: vec![Num::zero(); max_size],
            minima: vec![(Num::zero(), Num::zero()); max_size],
        }
    }

    /// Compute how much `b` should be shifted (to the right) to most closely match with `a`. The
    /// array `w` is used for weighting, and it should be as long as `b`. If it can be estimated,
    /// the period of the signal is also returned as the second item in the tuple. This can be used
    /// to compute the fundamental frequency of the signal.
    ///
    /// No array should exceed the maximum size given on `new`.
    pub fn compute(&mut self, a: &[Num], b: &[Num], w: &[Num]) -> (Num, Option<Num>) {
        assert!(a.len() <= self.max_size);
        assert!(b.len() <= a.len());
        assert!(w.len() == b.len());
        self.zero_buffers(a.len(), b.len());
        self.compute_a_squared_term(a, w);
        self.compute_cross_term(a, b, w);
        self.compute_b_squared_term(b, w);
        self.find_minimum_and_period()
    }

    fn zero_buffers(&mut self, a_len: usize, b_len: usize) {
        self.f_buffer.resize(a_len, Num::zero());
        self.g_buffer.resize(b_len, Num::zero());
        self.result_buffer.clear();
        self.result_buffer.resize(a_len - b_len + 1, Num::zero());
        self.minima.clear();
    }

    fn compute_a_squared_term(&mut self, a: &[Num], w: &[Num]) {
        // Compute term w[x] * a[x+t]^2. f = a^2, g = w
        for (f, &a) in self.f_buffer.iter_mut().zip(a.iter()) {
            *f = a.powi(2);
        }
        for (g, &w) in self.g_buffer.iter_mut().zip(w.iter()) {
            *g = w;
        }
        let cross_correlation_result = self
            .cross_correlation
            .compute_truncated(&self.f_buffer, &self.g_buffer);
        for (result, cross_correlation_result) in
            self.result_buffer.iter_mut().zip(cross_correlation_result)
        {
            *result += cross_correlation_result;
        }
    }

    fn compute_cross_term(&mut self, a: &[Num], b: &[Num], w: &[Num]) {
        // Compute term -2(w[x] * b[x]) * a[x+t]. f = a, g = w[x] * b[x]
        for (f, &a) in self.f_buffer.iter_mut().zip(a.iter()) {
            *f = a;
        }
        for (g, (&w, &b)) in self.g_buffer.iter_mut().zip(w.iter().zip(b.iter())) {
            *g = w * b;
        }
        let cross_correlation_result = self
            .cross_correlation
            .compute_truncated(&self.f_buffer, &self.g_buffer);
        for (result, cross_correlation_result) in
            self.result_buffer.iter_mut().zip(cross_correlation_result)
        {
            *result -= Num::v(2.) * cross_correlation_result;
        }
    }

    fn compute_b_squared_term(&mut self, b: &[Num], w: &[Num]) {
        // Compute term w[x] * b[x]^2. This is constant in t.
        let term: Num = w.iter().zip(b.iter()).map(|(&w, &b)| w * b.powi(2)).sum();
        for result in self.result_buffer.iter_mut() {
            *result += term;
        }
    }

    fn find_minimum_and_period(&mut self) -> (Num, Option<Num>) {
        let mut max_value = Num::one();
        for value in &self.result_buffer {
            max_value = value.max(max_value);
        }
        let mut min_position = Num::zero();
        let mut min_value = self.result_buffer[0];
        let end = self.result_buffer.len() - 1;
        if self.result_buffer[end] < min_value {
            min_position = Num::from_usize(end).unwrap();
            min_value = self.result_buffer[end];
        }
        for (index, [a, b, c]) in IterWindows::from(self.result_buffer.iter().copied()).enumerate()
        {
            if let Some((x, y)) = parabolic_interpolation_minimum(a, b, c) {
                let position = Num::from_usize(index).unwrap() + x;
                self.minima.push((position, y));
                if y < min_value {
                    min_position = position;
                    min_value = y;
                }
            }
        }
        let threshold = min_value + (max_value - min_value) * Num::v(0.1);
        self.minima.retain(|(_x, y)| *y < threshold);
        let mut valid_intervals = 0;
        for [(a_position, _), (b_position, _)] in IterWindows::from(self.minima.iter().copied()) {
            if b_position - a_position > Num::v(1.5) {
                valid_intervals += 1;
            }
        }
        let interval = if valid_intervals >= 2 {
            let total = self.minima.last().unwrap().0 - self.minima.first().unwrap().0;
            Some(total / Num::from_usize(valid_intervals).unwrap())
        } else {
            None
        };
        (min_position, interval)
    }
}
