#![warn(clippy::cognitive_complexity)]

mod display;
pub use display::DisplayBuffer;
mod correlation_match;
pub use correlation_match::parabolic_interpolation::parabolic_interpolation_minimum;
pub use correlation_match::CorrelationMatch;
mod cross_correlation;
pub use cross_correlation::CrossCorrelation;
pub mod ring_buffer;
pub mod util;
mod float;
pub use float::Float;

#[cfg(test)]
mod test;
