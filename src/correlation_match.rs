//! Design sketch in Finnish:
//!
//! Algoritmi, joka etsii pidemmästä äänenpätkästä A sen kohdan, jossa
//! lyhyempi äänenpätkä B esiintyy kaikista lähimpänä.
//!
//! Käyttötarkoituksena on oskilloskoopin näkymän vakautus. Silloin algoritmille
//! annettaisiin pätkä A uutta signaalia, ja pätkä B, joka vastaa viimeksi näytettyä
//! kuvaajaa. Algoritmi etsii uudesta signaalista sellaisen kohdan, jonka näyttämällä
//! kuvaaja muuttuu mahdollisimman vähän. Vakautettua kuvaajaa on toivottavasti helpompi
//! seurata, koska se ei liiku jatkuvasti taajuudesta riippuvalla tavalla.
//!
//! Olkoon signaalit `A[0..n]` ja `B[0..m]`, `n >= 2m`.
//! Algoritmi etsii sellaisen aikasiirroksen t, jolla summa x:n yli
//!
//! `w(x) * (A[x+t] - B[x])^2`
//!
//! on minimaalinen. Tässä `w(x)` on painofunktio, jonka avulla voidaan esimerkiksi
//! painottaa oskilloskoopin näkymän keskikohtia enemmän kuin reunoja.
//!
//! Jos tämä summa esitetään muodossa
//!
//! `w(x) * A[x+t]^2 - 2(w(x) * B[x]) * A[x+t] + w(x) * B[x]^2`,
//!
//! nähdään, että se voidaan laskea kahtena ristikorrelaationa (summat x:n yli muotoa
//! `f(x+t) * g(x)`) ja yhtenä suorana tulona (summa x:n yli muotoa `f(x) * g(x)`).

use crate::cross_correlation::CrossCorrelation;
use crate::math::*;

pub struct CorrelationMatch {
    max_size: usize,
    cross_correlation: CrossCorrelation,
    f_buffer: Vec<Num>,
    g_buffer: Vec<Num>,
    result_buffer: Vec<Num>,
}

impl CorrelationMatch {
    /// Allocate and prepare a correlation match algorithm. `max_size` is
    /// the maximum size of any of the input arrays.
    pub fn new(max_size: usize) -> Self {
        CorrelationMatch {
            max_size,
            cross_correlation: CrossCorrelation::new(max_size),
            f_buffer: vec![0.; max_size],
            g_buffer: vec![0.; max_size],
            result_buffer: vec![0.; max_size],
        }
    }

    /// Compute how much `b` should be shifted (to the right) to most closely match with `a`. The
    /// array `w` is used for weighting, and it should be as long as `b`. All arrays must be less
    /// than the maximum size given on `new`.
    pub fn compute(&mut self, a: &[Num], b: &[Num], w: &[Num]) -> f32 {
        assert!(a.len() <= self.max_size);
        assert!(b.len() <= a.len());
        assert!(w.len() == b.len());
        self.f_buffer.resize(a.len(), 0.);
        self.g_buffer.resize(b.len(), 0.);
        self.result_buffer.clear();
        self.result_buffer.resize(a.len() - b.len() + 1, 0.);
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
            *result -= 2. * cross_correlation_result;
        }
        // Compute term w[x] * b[x]^2. This is constant in t, so it shouldn't affect
        // the rest of the algorithm (but it may in the future).
        let term: Num = w.iter().zip(b.iter()).map(|(&w, &b)| w * b.powi(2)).sum();
        for result in self.result_buffer.iter_mut() {
            *result += term;
        }
        // For now, find the minimum index and don't try to interpolate.
        let mut min_index = 0;
        let mut min_value = Num::INFINITY;
        for (index, &value) in self.result_buffer.iter().enumerate() {
            if value < min_value {
                min_index = index;
                min_value = value;
            }
        }
        min_index as Num
    }
}
