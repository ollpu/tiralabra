
use crate::math::*;
use crate::fft;
use super::util::*;

#[test]
fn fft_ifft_results_in_original() {
    let prepared = fft::Prepared::new(8);
    let array = complex_vec(
        &[(0.99939176, 0.00832791), (-0.08551277, -0.89168611),
          (0.13879818, 0.63602133), (0.26292814, 0.96916421),
          (0.15374602, 0.90759436), (0.37355538, 0.78602014),
          (-0.50546959, 0.32142292), (-0.89801354, 0.30184848)]
    );
    let mut buffer = array.clone();
    prepared.fft(&mut buffer);
    prepared.ifft(&mut buffer);
    assert!(complex_slice_eq(&buffer, &array, 6));
}

#[test]
fn fft_unit_impulse() {
    let prepared = fft::Prepared::new(8);
    let mut buffer = vec![(0., 0.).into(); 8];
    buffer[1] = (1., 0.).into();

    prepared.fft(&mut buffer);

    let w = |x| Complex::euler(-2. * PI / 8. * x);
    let correct = complex_vec(
        &[w(0.), w(1.), w(2.), w(3.),
          w(4.), w(5.), w(6.), w(7.)]
    );
    assert!(complex_slice_eq(&buffer, &correct, 6));
}
