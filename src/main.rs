use eframe::{egui, wgpu::naga::ScalarKind::Bool};
use std::time::SystemTime;

fn main() -> eframe::Result<()> {
    let icon = eframe::icon_data::from_png_bytes(include_bytes!("icon.png"))
        .expect("icon.png is geen geldige PNG");
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([400.0, 300.0])
            .with_resizable(true)
            .with_transparent(false)
            .with_icon(icon),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Rust Calculator App",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            cc.egui_ctx.global_style_mut(|style| {
                style.spacing.item_spacing = egui::vec2(8.0, 8.0);
                style.spacing.button_padding = egui::vec2(12.0, 8.0);
                style.text_styles.insert(
                    egui::TextStyle::Heading,
                    egui::FontId::new(28.0, egui::FontFamily::Proportional),
                );
                style.text_styles.insert(
                    egui::TextStyle::Button,
                    egui::FontId::new(20.0, egui::FontFamily::Proportional),
                );
            });
            Ok(Box::new(MijnApp::default()))
        }),
    )
}

#[derive(Default)]
struct MijnApp {
    notitie: Notitie,
}
struct Notitie {
    text: String,
    time: SystemTime,
    uploaded: bool,
    draft: String,
}

impl Default for Notitie {
    fn default() -> Self {
        Self {
            text: String::new(),
            time: SystemTime::now(),
            uploaded: false,
            draft: String::new(),
        }
    }
}

impl Notitie {
    fn bijwerken(&mut self) {
        self.time = SystemTime::now();
    }
    fn text_update(&mut self, t: String) {
        self.text = t;
    }
}

impl eframe::App for MijnApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Welkom in de Notitie app!");
            ui.label("Gemaakt door Tim Bolmers");
            ui.separator();
            if ui.button("Pas tijd aan").clicked() {
                self.notitie.bijwerken();
            }
            let tijd: chrono::DateTime<chrono::Local> = self.notitie.time.into();
            ui.label(format!("Laatst gewijzigd: {}", tijd.format("%H:%M:%S")));
            ui.text_edit_singleline(&mut self.notitie.draft);
            if ui.button("Upload...").clicked() {
                self.notitie.text_update(self.notitie.draft.clone());
                self.notitie.uploaded = true;
                self.notitie.bijwerken(); 
            }
            ui.label(format!("Geschreven Tekst: {}", self.notitie.text));
            
        });
    }
}