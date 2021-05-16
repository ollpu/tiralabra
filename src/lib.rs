#![warn(clippy::cognitive_complexity)]

mod display;
pub use display::DisplayBuffer;
mod correlation_match;
pub use correlation_match::parabolic_interpolation::parabolic_interpolation_minimum;
pub use correlation_match::CorrelationMatch;
mod cross_correlation;
pub use cross_correlation::CrossCorrelation;
mod fft;
pub use fft::Fft;
pub mod math;
pub mod ring_buffer;
pub mod util;

#[cfg(test)]
mod test;
