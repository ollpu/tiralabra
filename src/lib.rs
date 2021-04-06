#![warn(clippy::cognitive_complexity)]
// #![warn(missing_docs)]

pub mod correlation_match;
mod cross_correlation;
mod fft;
mod math;

#[cfg(test)]
mod test;
