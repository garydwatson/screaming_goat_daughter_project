use eframe::egui;
use rodio::{Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

#[derive(Default)]
struct App {
    text: String,
    sound_path: String,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.label("Button and Sound Program");

            if ui.button("Hello World").clicked() {
                self.text.push_str("\nHello World");
            }

            if ui.button("Play Sound").clicked() {
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                let sink = rodio::Sink::try_new(&stream_handle).unwrap();

                let file = File::open(&self.sound_path).unwrap();
                let source = Decoder::new(BufReader::new(file)).unwrap();

                sink.append(source);
                sink.sleep_until_end();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(&self.text);
        });
    }
}

fn main() {
    let app = App {
        text: String::new(),
        sound_path: "funny_screaming_goat.wav".to_owned(),
    };
    ////eframe::run_native(Box::new(app), eframe::NativeOptions::default());
    let native_options = eframe::NativeOptions::default();
    //eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(app))).unwrap();
}
