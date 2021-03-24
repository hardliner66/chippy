#![allow(dead_code)]

mod app;

fn main() {
    let app = app::TemplateApp::default();
    eframe::run_native(Box::new(app));
}
