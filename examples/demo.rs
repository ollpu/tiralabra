use tuix::*;
use std::{result::Result, error::Error};
use std::vec::Vec;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::convert::TryInto;

fn main() -> Result<(), Box<dyn Error>> {
    let host = cpal::default_host();
    let device = host.default_input_device().unwrap();
    eprintln!("Käytetään äänilaitetta: \"{}\"", device.name()?);
    let mut config: cpal::StreamConfig = device.default_input_config()?.into();
    config.channels = 1;
    let (mut plot, mut plot_ingest) = Plot::new_and_ingestor(config.sample_rate.0);
    let audio_cb = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        plot_ingest.process(data);
    };
    let input_stream = device.build_input_stream(&config, audio_cb, err_fn)?;
    input_stream.play()?;
    let mut app = Application::new(move |state, window| {
        //state.insert_theme(THEME);
        plot.build(state, window.entity(), |builder| builder.set_flex_grow(1.0));
        window.set_title("Tiralabra demo").set_inner_size(800, 600);
    });
    app.run();
    Ok(())
}
fn err_fn(err: cpal::StreamError) {
    eprintln!("Virhe äänilaitteen kanssa: {}", err);
}

/// Hard-coded to read pieces of size 44100/60 for now.
const N: usize = 735;

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

struct Plot {
    consume_handle: triple_buffer::Output<[f32; N]>,
}

impl Plot {
    pub fn new_and_ingestor(_sample_rate: u32) -> (Self, PlotIngest) {
        let buffer = triple_buffer::TripleBuffer::new([0.; N]);
        let (buf_in, buf_out) = buffer.split();
        (
            Plot { consume_handle: buf_out },
            PlotIngest {
                publish_handle: buf_in,
                buffer: Vec::with_capacity(N),
                clock: 0,
            }
        )
    }
}

use femtovg::{
    renderer::OpenGl, Path, Paint, Color, Canvas
};

impl Widget for Plot {
    type Ret = Entity;
    fn on_build(&mut self, _state: &mut State, entity: Entity) -> Self::Ret {
        //state.style.insert_element(entity, "element");
        entity
    }
    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas<OpenGl>) {
        state.insert_event(Event::new(WindowEvent::Redraw));
        entity.set_text(state, "");
        let mut path = Path::new();
        let buf = self.consume_handle.read();
        let mut points = buf.iter().enumerate().map(|(i, v)| {
            (i as f32, 200.-v*200.)
        });
        let (x, y) = points.next().unwrap();
        path.move_to(x, y);
        for (x, y) in points {
            path.line_to(x, y);
        }
        canvas.stroke_path(&mut path, Paint::color(Color::rgb(255, 255, 0)));
    }
}
