#![warn(clippy::cognitive_complexity)]

pub mod display;
pub mod correlation_match;
pub mod cross_correlation;
pub mod fft;
pub mod math;
pub mod ring_buffer;
pub mod util;

#[cfg(test)]
mod test;
