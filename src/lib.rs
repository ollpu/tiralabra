#![warn(clippy::cognitive_complexity)]
// #![warn(missing_docs)]

pub mod correlation_match;
pub mod cross_correlation;
pub mod fft;
pub mod math;
pub mod ring_buffer;

#[cfg(test)]
mod test;
