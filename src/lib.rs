#![warn(clippy::cognitive_complexity)]
// #![warn(missing_docs)]

pub mod correlation_match;
pub mod cross_correlation;
pub mod fft;
pub mod math;

#[cfg(test)]
mod test;
