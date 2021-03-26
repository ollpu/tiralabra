
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

#[test]
fn fft_random() {
    let prepared = fft::Prepared::new(8);
    let mut buffer = complex_vec(
        &[(0.99939176, 0.00832791), (-0.08551277, -0.89168611),
          (0.13879818, 0.63602133), (0.26292814, 0.96916421),
          (0.15374602, 0.90759436), (0.37355538, 0.78602014),
          (-0.50546959, 0.32142292), (-0.89801354, 0.30184848)]
    );
    prepared.fft(&mut buffer);
    let correct = complex_vec(
        &[(0.43942358, 3.03871324), (-0.69972977, -3.6980147),
          (0.14313053, -0.96464999), (0.96211328, 0.90688273),
          (1.13350916, 0.7080198), (3.02021807, 0.61094626),
          (2.89648785, 0.88160603), (0.09998138, -1.41688009)]
    );
    assert!(complex_slice_eq(&buffer, &correct, 6));
}
