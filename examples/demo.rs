use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::convert::TryInto;
use std::vec::Vec;
use std::{error::Error, result::Result};
use tuix::*;

static THEME: &'static str = include_str!("theme.css");
fn main() -> Result<(), Box<dyn Error>> {
    let host = cpal::default_host();
    let device = host.default_input_device().unwrap();
    eprintln!("Käytetään äänilaitetta: \"{}\"", device.name()?);
    let mut config: cpal::StreamConfig = device.default_input_config()?.into();
    config.channels = 1;
    let (plot, mut plot_ingest) = Plot::new_and_ingestor(config.sample_rate.0);
    let audio_cb = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        plot_ingest.process(data);
    };
    let input_stream = device.build_input_stream(&config, audio_cb, err_fn)?;
    input_stream.play()?;
    let app = Application::new(move |state, window| {
        state.add_theme(style::themes::DEFAULT_THEME);
        state.add_theme(THEME);
        window.set_layout_type(state, LayoutType::Horizontal);
        Control::default().build(state, window.entity(), |builder| {
            builder.set_width(Stretch(1.)).set_min_width(Pixels(200.))
        });
        plot.build(state, window.entity(), |builder| {
            builder.set_width(Stretch(4.)).set_height(Stretch(1.))
        });
        window.set_title("Tiralabra demo").set_inner_size(800, 600);
    });
    app.run();
    Ok(())
}
fn err_fn(err: cpal::StreamError) {
    eprintln!("Virhe äänilaitteen kanssa: {}", err);
}

#[derive(Clone, PartialEq, Debug)]
enum PlotControlEvent {
    Stabilize(bool),
    ShowMemory(bool),
    DisplayDecayTime(f32),
    MemoryDecayTime(f32),
}

/// Hard-coded to read pieces of size 44100/30 for now.
const N: usize = 2 * 735;
const M: usize = 2 * 360;

struct PlotIngest {
    publish_handle: triple_buffer::Input<[f32; N]>,
    buffer: Vec<f32>,
    clock: usize,
}

impl PlotIngest {
    fn process(&mut self, data: &[f32]) {
        for sample in data {
            if self.clock < N {
                self.buffer.push(*sample);
                if self.buffer.len() == N {
                    if let Ok(array) = self.buffer[..].try_into() {
                        self.publish_handle.write(array);
                    }
                    self.buffer.clear();
                }
            }
            self.clock += 1;
            if self.clock == N {
                self.clock = 0;
            }
        }
    }
}

use tiralabra::correlation_match::CorrelationMatch;
struct Plot {
    consume_handle: triple_buffer::Output<[f32; N]>,
    correlation_matcher: CorrelationMatch,
    last_displayed: [f32; M],
    memory: [f32; M],
    weight: [f32; M],
    stabilize_enabled: bool,
    show_memory: bool,
    display_decay: f32,
    memory_decay: f32,
}

fn decay_time_to_factor(time: f32) -> f32 {
    // arbitrary constant that gives a useful range
    1. - (-1. / 6. / time).exp()
}

impl Plot {
    pub fn new_and_ingestor(_sample_rate: u32) -> (Self, PlotIngest) {
        let buffer = triple_buffer::TripleBuffer::new([0.; N]);
        let (buf_in, buf_out) = buffer.split();
        let mut weight = [0.; M];
        for (i, w) in weight.iter_mut().enumerate() {
            *w = 1.
                + (2. * std::f32::consts::PI * (i as isize - (M / 2) as isize) as f32 / M as f32)
                    .cos()
        }
        (
            Plot {
                consume_handle: buf_out,
                correlation_matcher: CorrelationMatch::new(N),
                last_displayed: [0.; M],
                memory: [0.; M],
                weight,
                stabilize_enabled: true,
                show_memory: false,
                display_decay: decay_time_to_factor(0.2),
                memory_decay: decay_time_to_factor(0.8),
            },
            PlotIngest {
                publish_handle: buf_in,
                buffer: Vec::with_capacity(N),
                clock: 0,
            },
        )
    }
}

use femtovg::{renderer::OpenGl, Canvas, Color, Paint, Path};

impl Widget for Plot {
    type Ret = Entity;
    fn on_build(&mut self, _state: &mut State, entity: Entity) -> Self::Ret {
        //state.style.insert_element(entity, "element");
        entity
    }
    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas<OpenGl>) {
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));
        let BoundingBox { x, y, h, w } = state.data.get_bounds(entity);

        let buf = self.consume_handle.read();
        let offset;
        if self.stabilize_enabled {
            offset = self
                .correlation_matcher
                .compute(buf, &self.memory, &self.weight) as usize;
        } else {
            offset = 0;
        }
        let factor = self.display_decay;
        for (i, tr) in self.last_displayed.iter_mut().enumerate() {
            *tr = factor * buf[i + offset] + (1. - factor) * *tr;
        }
        let factor = self.memory_decay;
        for (i, tr) in self.memory.iter_mut().enumerate() {
            *tr = factor * buf[i + offset] + (1. - factor) * *tr;
        }
        if self.show_memory {
            let mut path = Path::new();
            let mut points = self
                .memory
                .iter()
                .enumerate()
                .map(|(i, v)| (x + w / M as f32 * i as f32, y + h / 2. - v * h / 2.));
            let (x, y) = points.next().unwrap();
            path.move_to(x, y);
            for (x, y) in points {
                path.line_to(x, y);
            }
            canvas.stroke_path(&mut path, Paint::color(Color::rgb(12, 170, 255)));
        }
        let mut path = Path::new();
        let mut points = self
            .last_displayed
            .iter()
            .enumerate()
            .map(|(i, v)| (x + w / M as f32 * i as f32, y + h / 2. - v * h / 2.));
        let (x, y) = points.next().unwrap();
        path.move_to(x, y);
        for (x, y) in points {
            path.line_to(x, y);
        }
        canvas.stroke_path(&mut path, Paint::color(Color::rgb(255, 255, 0)));
    }

    fn on_event(&mut self, _state: &mut State, _entity: Entity, event: &mut Event) {
        if let Some(control) = event.message.downcast() {
            match control {
                PlotControlEvent::Stabilize(enable) => {
                    self.stabilize_enabled = *enable;
                }
                PlotControlEvent::ShowMemory(enable) => {
                    self.show_memory = *enable;
                }
                PlotControlEvent::DisplayDecayTime(val) => {
                    self.display_decay = decay_time_to_factor(*val);
                }
                PlotControlEvent::MemoryDecayTime(val) => {
                    self.memory_decay = decay_time_to_factor(*val);
                }
            }
            event.consume();
        }
    }
}

#[derive(Default)]
struct Control {
    memory_over: bool,
    memory_press: bool,
}

#[derive(Clone, PartialEq, Debug)]
enum MemoryHoverEvent {
    OverChange(bool),
    PressChange(bool),
}

impl Widget for Control {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_element(state, "control");
        entity.set_flex_direction(state, FlexDirection::Column);
        let checkbox = Row::new().build(state, entity, |builder| builder.class("check"));
        Checkbox::new(true)
            .on_checked(Event::new(PlotControlEvent::Stabilize(true)).propagate(Propagation::All))
            .on_unchecked(
                Event::new(PlotControlEvent::Stabilize(false)).propagate(Propagation::All),
            )
            .build(state, checkbox, |builder| builder);
        Label::new("Vakauta").build(state, checkbox, |builder| builder);
        Label::new("Näytön vaimenemisaika").build(state, entity, |builder| builder);
        Slider::new()
            .with_min(0.)
            .with_max(2.)
            .with_initial_value(0.2)
            .on_changing(move |val| {
                Event::new(PlotControlEvent::DisplayDecayTime(val))
                    .propagate(Propagation::All)
                    .target(entity)
            })
            .build(state, entity, |builder| builder);
        Label::new("Muistin vaimenemisaika").build(state, entity, |builder| builder);
        Slider::new()
            .with_min(0.)
            .with_max(2.)
            .with_initial_value(0.8)
            .on_over(Event::new(MemoryHoverEvent::OverChange(true)).direct(entity))
            .on_out(Event::new(MemoryHoverEvent::OverChange(false)).direct(entity))
            .on_press(Event::new(MemoryHoverEvent::PressChange(true)).direct(entity))
            .on_release(Event::new(MemoryHoverEvent::PressChange(false)).direct(entity))
            .on_changing(move |val| {
                Event::new(PlotControlEvent::MemoryDecayTime(val))
                    .propagate(Propagation::All)
                    .target(entity)
            })
            .build(state, entity, |builder| builder);
        entity
    }

    fn on_event(&mut self, state: &mut State, _entity: Entity, event: &mut Event) {
        if let Some(ev) = event.message.downcast() {
            match ev {
                MemoryHoverEvent::OverChange(status) => self.memory_over = *status,
                MemoryHoverEvent::PressChange(status) => self.memory_press = *status,
            }
            state.insert_event(
                Event::new(PlotControlEvent::ShowMemory(
                    self.memory_over || self.memory_press,
                ))
                .propagate(Propagation::All),
            );
        }
    }
}
