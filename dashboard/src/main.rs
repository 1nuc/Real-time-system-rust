use eframe::egui;
use egui_plot::{Plot, Line, PlotPoints};
struct App{
    counter: i32,
}
impl Default for App{
    fn default()->Self{
        Self{
            counter:0,
        }
    }
}
impl eframe::App for App{

    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame){
        egui::CentralPanel::default().show(ctx, |i|{
            let plot=Plot::new("first plot");
            plot.show(i, |plot_ui|{
                plot_ui.line(Line::new("my plot",PlotPoints::from_ys_f32(&[2.0,432.0,43.0,5.0,5.0,5.0,3.0,3.0])));
            });
            ui_counter(i, &mut self.counter);
        });
    }
}
fn ui_counter(ui: &mut egui::Ui, counter: &mut i32) {
    // Put the buttons and label on the same row:
    ui.horizontal(|ui| {
        if ui.button("−").clicked() {
            *counter -= 1;
        }
        ui.label(counter.to_string());
        if ui.button("+").clicked() {
            *counter += 1;
        }
    });
}
fn main() {
    let options=eframe::NativeOptions::default();
    eframe::run_native("my app", options, Box::new(|_| Ok(Box::new(App::default())))).unwrap();
}
