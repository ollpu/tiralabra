use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::vec::Vec;
use std::{error::Error, result::Result};
use tuix::*;

mod test_signal;
use test_signal::TestSignal;

use tiralabra::ring_buffer;

static THEME: &'static str = include_str!("theme.css");
fn main() -> Result<(), Box<dyn Error>> {
    let (publish_handle, consume_handle) = ring_buffer::with_capacity(8 * N);
    match setup_audio(publish_handle) {
        Ok(_) => {},
        Err(e) => eprintln!("Mikrofonin avaaminen ei onnistunut: {:?}\nVoit silti käyttää testisignaalia!", e),
    }
    let plot = Plot::new(consume_handle);
    let app = Application::new(move |state, window| {
        state.add_theme(style::themes::DEFAULT_THEME);
        state.add_theme(THEME);
        window.set_layout_type(state, LayoutType::Row);
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
fn setup_audio(publish_handle: ring_buffer::Producer<f32>) -> Result<(), Box<dyn Error>> {
    let host = cpal::default_host();
    let device = host.default_input_device().ok_or("Äänilaitetta ei löydetty")?;
    eprintln!("Käytetään äänilaitetta: \"{}\"", device.name()?);
    let config = device.default_input_config()?;
    let sample_format = config.sample_format();
    let config: cpal::StreamConfig = config.into();
    let plot_ingest = PlotIngest::new(config.channels as usize, publish_handle);
    match sample_format {
        cpal::SampleFormat::F32 => run_audio::<f32>(device, config, plot_ingest)?,
        cpal::SampleFormat::I16 => run_audio::<i16>(device, config, plot_ingest)?,
        cpal::SampleFormat::U16 => run_audio::<u16>(device, config, plot_ingest)?,
    }
    Ok(())
}
fn run_audio<T: cpal::Sample>(
    device: cpal::Device,
    config: cpal::StreamConfig,
    mut plot_ingest: PlotIngest,
) -> Result<(), Box<dyn Error>> {
    let audio_cb = move |data: &[T], _: &cpal::InputCallbackInfo| {
        plot_ingest.process(data);
    };
    std::thread::spawn(move || {
        let input_stream = device
            .build_input_stream(&config, audio_cb, err_fn)
            .unwrap();
        input_stream.play().unwrap();
        std::thread::park();
    });
    Ok(())
}

#[derive(Clone, PartialEq, Debug)]
enum PlotControlEvent {
    Stabilize(bool),
    ShowMemory(bool),
    DisplayDecayTime(f32),
    MemoryDecayTime(f32),
    Source(AudioSource),
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum AudioSource {
    Microphone,
    Test,
}

/// Hard-coded to read pieces of size 44100/30 for now.
const N: usize = 2 * 735;
const M: usize = 2 * 360;
const SCROLL_AMOUNT: i32 = 100;

struct PlotIngest {
    channels: usize,
    publish_handle: ring_buffer::Producer<f32>,
    buffer: Vec<f32>,
}

impl PlotIngest {
    fn new(channels: usize, publish_handle: ring_buffer::Producer<f32>) -> PlotIngest {
        PlotIngest {
            channels,
            publish_handle,
            buffer: Vec::with_capacity(N),
        }
    }
    fn process<T: cpal::Sample>(&mut self, data: &[T]) {
        for frame in data.chunks(self.channels) {
            if self.buffer.len() < N {
                let val = frame.iter().map(|v| v.to_f32()).sum::<f32>() / self.channels as f32;
                self.buffer.push(val);
            }
            if self.buffer.len() == N {
                match self.publish_handle.push(&self.buffer) {
                    Ok(()) => (), // success
                    Err(_) => {
                        // fail, report error for debugging
                        println!("ring buffer full");
                    }
                }
                self.buffer.clear();
            }
        }
    }
}

use tiralabra::correlation_match::CorrelationMatch;
struct Plot {
    consume_handle: ring_buffer::Consumer<f32>,
    test_signal_generator: TestSignal,
    correlation_matcher: CorrelationMatch,
    buffer: [f32; N],
    offset: usize,
    last_displayed: [f32; M],
    memory: [f32; M],
    weight: [f32; M],
    stabilize_enabled: bool,
    show_memory: bool,
    display_decay: f32,
    memory_decay: f32,
    audio_source: AudioSource,
    scroll_amount: i32,
}

fn decay_time_to_factor(time: f32) -> f32 {
    // arbitrary constant that gives a useful range
    1. - (-1. / 6. / time).exp()
}

impl Plot {
    pub fn new(consume_handle: ring_buffer::Consumer<f32>) -> Self {
        let mut weight = [0.; M];
        for (i, w) in weight.iter_mut().enumerate() {
            *w = 1.
                + (2. * std::f32::consts::PI * (i as isize - (M / 2) as isize) as f32 / M as f32)
                    .cos()
        }
        Plot {
            consume_handle,
            test_signal_generator: TestSignal::new(),
            correlation_matcher: CorrelationMatch::new(N),
            buffer: [0.; N],
            offset: 0,
            last_displayed: [0.; M],
            memory: [0.; M],
            weight,
            stabilize_enabled: true,
            show_memory: false,
            display_decay: decay_time_to_factor(0.2),
            memory_decay: decay_time_to_factor(0.8),
            audio_source: AudioSource::Microphone,
            scroll_amount: 0,
        }
    }
}

use femtovg::{renderer::OpenGl, Canvas, Color, Paint, Path};

impl Widget for Plot {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        //state.style.insert_element(entity, "element");
        let animation = AnimationState::new()
            .with_duration(std::time::Duration::from_secs(30000000))
            .with_keyframe((0.0, Stretch(0.0)))
            .with_keyframe((0.0, Stretch(5.)));
        let animation = state.style.border_width.insert_animation(animation);
        state.style.border_width.play_animation(entity, animation);
        entity
    }
    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas<OpenGl>) {
        state.insert_event(Event::new(WindowEvent::Redraw).direct(Entity::root()));
        let BoundingBox { x, y, h, w } = state.data.get_bounds(entity);

        // Handle scroll
        let mut scroll = self.scroll_amount / 6;
        if scroll == 0 && self.scroll_amount != 0 {
            scroll = self.scroll_amount;
        }
        self.scroll_amount -= scroll;
        if scroll > 0 {
            let scroll = scroll as usize;
            self.last_displayed.rotate_right(scroll);
            self.memory.rotate_right(scroll);
            self.buffer.rotate_right(scroll);
            for v in self.buffer[..scroll].iter_mut() { *v = 0.; }
            self.last_displayed[..scroll].copy_from_slice(&self.buffer[self.offset..][..scroll]);
            self.memory[..scroll].copy_from_slice(&self.buffer[self.offset..][..scroll]);
        } else if scroll < 0 {
            let scroll = -scroll as usize;
            self.last_displayed.rotate_left(scroll);
            self.memory.rotate_left(scroll);
            self.buffer.rotate_left(scroll);
            for v in self.buffer[N - scroll..].iter_mut() { *v = 0.; }
            self.last_displayed[M - scroll..].copy_from_slice(
                &self.buffer[self.offset + M - scroll..][..scroll]
            );
            self.memory[M - scroll..].copy_from_slice(
                &self.buffer[self.offset + M - scroll..][..scroll]
            );
        }

        let mut test_once = true;
        while {
            match self.audio_source {
                AudioSource::Microphone => self.consume_handle.pop_full(&mut self.buffer).is_ok(),
                AudioSource::Test => {
                    if test_once {
                        self.consume_handle.discard_all();
                        test_once = false;
                        self.test_signal_generator.get(&mut self.buffer)
                    } else {
                        false
                    }
                }
            }
        } {
            if self.stabilize_enabled {
                self.offset =
                    self.correlation_matcher
                        .compute(&self.buffer, &self.memory, &self.weight)
                        as usize;
            }
            let factor = self.memory_decay;
            for (i, tr) in self.memory.iter_mut().enumerate() {
                *tr = factor * self.buffer[i + self.offset] + (1. - factor) * *tr;
            }
        }
        // Draw indicator self.offset indicator
        canvas.clear_rect((x + 0.4 * w) as u32, (y + h - 40.) as u32, (0.2 * w) as u32, 15, Color::rgb(70, 70, 70));
        let pos = self.offset as f32 / N as f32;
        let span = M as f32 / N as f32;
        canvas.clear_rect((x + (0.4 + 0.2 * pos) * w) as u32, (y + h - 40.) as u32, (0.2 * span * w) as u32, 15, Color::rgb(90, 90, 90));
        // Smooth once per displayed frame. Memory is smoothed once per piece of input data instead.
        let factor = if self.stabilize_enabled { self.display_decay } else { 1. };
        for (i, tr) in self.last_displayed.iter_mut().enumerate() {
            *tr = factor * self.buffer[i + self.offset] + (1. - factor) * *tr;
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
                PlotControlEvent::Source(source) => {
                    self.audio_source = *source;
                }
            }
            event.consume();
        }
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseScroll(_, change) => {
                    if *change > 0. {
                        self.scroll_amount += SCROLL_AMOUNT;
                    } else {
                        self.scroll_amount -= SCROLL_AMOUNT;
                    }
                }
                _ => {}
            }
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
        entity.set_layout_type(state, LayoutType::Column);
        let (_, _, dropdown) = Dropdown::new("Äänilähde").build(state, entity, |b| {
            b
                .set_height(Pixels(30.0))
                .set_width(Stretch(1.0))
        });
        let options = List::new().build(state, dropdown, |b| b);
        CheckButton::new(true)
            .on_checked(Event::new(PlotControlEvent::Source(AudioSource::Microphone)).propagate(Propagation::All))
            .build(state, options, |b| {
            b
                .set_text("Mikrofoni")
                .set_height(Pixels(30.0))
                .set_child_left(Pixels(5.0))
        });
        CheckButton::new(false)
            .on_checked(Event::new(PlotControlEvent::Source(AudioSource::Test)).propagate(Propagation::All))
            .build(state, options, |b| {
            b
                .set_text("Testisignaali")
                .set_height(Pixels(30.0))
                .set_child_left(Pixels(5.0))
        });
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
