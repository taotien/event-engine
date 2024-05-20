#![warn(clippy::all)]

mod app;
pub use app::App;

pub struct Scraper {
    response: ehttp::Response,
    text: Option<String>,
}

impl Scraper {
    pub fn from_response(ctx: &egui::Context, response: ehttp::Response) -> Self {
        let text = response.text().map(|t| t.into());
        Self { response, text }
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
