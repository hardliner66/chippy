use chippy::Emulator;
use eframe::{egui, epi};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct TemplateApp {
    emu: Emulator,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let mut emu = Emulator::default();
        emu.load(include_bytes!("../roms/chip-8/TETRIS.bin"))
            .unwrap();
        emu.pause();
        Self { emu }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "egui template"
    }

    /// Called by the framework to load old app state (if any).
    #[cfg(feature = "persistence")]
    fn load(&mut self, storage: &dyn epi::Storage) {
        *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        ctx.request_repaint();
        let TemplateApp { emu } = self;

        emu.clock().unwrap();

        egui::SidePanel::left("side_panel", 200.0).show(ctx, |ui| {
            ui.heading("Side Panel");

            if ui.button("Pause/Resume").clicked() {
                emu.toggle_pause();
            }
            ui.separator();
            if ui.button("Step").clicked() {
                emu.step().expect("Error goes brrrrrrrrrr.");
            }

            ui.separator();

            ui.collapsing("Registers", |ui| {
                egui::Grid::new("address_register").show(ui, |ui| {
                    ui.label("I");
                    ui.label(format!("{:#04X}", emu.read_address_register()));
                    ui.end_row();
                });
                egui::Grid::new("memory_locations").show(ui, |ui| {
                    for i in 0..=0xF_u8 {
                        ui.label(format!("V{:X}", i));
                        ui.label(format!("{:#04X}", emu.read_register(i.into())));
                        ui.end_row();
                    }
                });
            });
            ui.label(format!("IP: {:X}", emu.instruction_pointer()));
        });

        egui::TopPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Central Panel");
            egui::warn_if_debug_build(ui);

            ui.separator();

            egui::containers::ScrollArea::auto_sized().show(ui, |ui| {
                egui::Grid::new("memory").show(ui, |ui| {
                    for i in (0..4096).step_by(16) {
                        for n in 0..16 {
                            let value = format!("{:#04X}", emu.read_memory(i + n));
                            if ((i + n) == emu.instruction_pointer()) || 
                               ((i + n - 1) == emu.instruction_pointer())
                            {
                                ui.colored_label(egui::Color32::RED, value);
                            } else {
                                ui.label(value);
                            }
                        }
                        ui.end_row();
                    }
                });
            });

        });
    }
}
