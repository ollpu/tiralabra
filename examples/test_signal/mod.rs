use std::f32::consts::PI;

const SRATE: f32 = 44100.;
const BASE_FREQUENCY: f32 = 216.2747907;

#[derive(Default)]
pub struct TestSignal {
    counter: usize,
    oscillator_phase: f32,
    modulator_phase: f32,
}

impl TestSignal {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(&mut self, buffer: &mut [f32], modulate: bool) -> bool {
        if self.counter == 0 {
            for sample in buffer.iter_mut() {
                let modulator;
                if modulate {
                    modulator = osc(&mut self.modulator_phase, 0.5);
                } else {
                    modulator = 0.;
                }
                *sample = 0.8
                    * osc(
                        &mut self.oscillator_phase,
                        BASE_FREQUENCY * (1. + 0.109 * modulator),
                    );
            }
            self.counter = 1;
            true
        } else {
            self.counter = 0;
            false
        }
    }
}

fn osc(phase: &mut f32, freq: f32) -> f32 {
    *phase += freq / SRATE;
    *phase = phase.fract();
    (*phase * 2. * PI).sin()
}
