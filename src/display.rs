use crate::math::*;
use crate::util::{shift_right, shift_left, shift_right_fill, shift_left_fill};
use crate::correlation_match::CorrelationMatch;

pub struct DisplayBuffer {
    size: usize,
    correlation_matcher: CorrelationMatch,
    buffer: Vec<Num>,
    display: Vec<Num>,
    memory: Vec<Num>,
    weight: Vec<Num>,
    offset: usize,
    residual: Num,
    average_interval: Num,
}

impl DisplayBuffer {
    pub fn new(input_size: usize, display_size: usize) -> Self {
        let weight: Vec<_> = (0..display_size)
            .map(|index| index as isize - (input_size / 2) as isize)
            .map(|offset| offset as f32 / input_size as f32)
            .map(|x| 1. + (2. * PI * x).cos())
            .collect();
        DisplayBuffer {
            size: display_size,
            correlation_matcher: CorrelationMatch::new(input_size),
            buffer: vec![0.; input_size],
            display: vec![0.; display_size],
            memory: vec![0.; display_size],
            weight,
            offset: 0,
            residual: 0.,
            average_interval: 0.,
        }
    }

    pub fn scroll(&mut self, amount: i32) {
        if amount > 0 {
            let amount = amount as usize;
            shift_right_fill(&mut self.buffer, amount, 0.);
            shift_right(&mut self.display, &self.buffer[self.offset..][..amount]);
            shift_right(&mut self.memory, &self.buffer[self.offset..][..amount]);
        } else if amount < 0 {
            let amount = -amount as usize;
            shift_left_fill(&mut self.buffer, amount, 0.);
            shift_left(&mut self.display, &self.buffer[self.offset + self.size - amount..][..amount]);
            shift_left(&mut self.memory, &self.buffer[self.offset + self.size - amount..][..amount]);
        }
    }

    pub fn get_buffer_mut(&mut self) -> &mut [Num] {
        &mut self.buffer
    }

    pub fn update_match(&mut self, stabilize: bool, memory_decay: Num, interval_decay: Num) {
        if stabilize {
            let (offset, interval) =
                self.correlation_matcher
                .compute(&self.buffer, &self.memory, &self.weight);
            let rounded = offset.round();
            self.offset = rounded as usize;
            self.residual += offset - rounded;
            self.offset = (self.offset as i64 + self.residual as i64).clamp(0, self.buffer.len() as i64 - 1) as usize;
            self.residual = self.residual.fract();
            if let Some(interval) = interval {
                self.average_interval = interval_decay * interval + (1. - interval_decay) * self.average_interval;
            }
        }
        for (index, item) in self.memory.iter_mut().enumerate() {
            *item = memory_decay * self.buffer[index + self.offset] + (1. - memory_decay) * *item;
        }
    }

    pub fn update_display(&mut self, display_decay: Num) {
        for (index, item) in self.display.iter_mut().enumerate() {
            *item = display_decay * self.buffer[index + self.offset] + (1. - display_decay) * *item;
        }
    }

    pub fn get_display(&self) -> &[Num] {
        &self.display
    }

    pub fn get_memory(&self) -> &[Num] {
        &self.memory
    }

    pub fn get_interval(&self) -> Num {
        self.average_interval
    }

    pub fn get_offset(&self) -> (usize, Num) {
        (self.offset, self.residual)
    }
}
