use egui::{CentralPanel, Hyperlink, ScrollArea, SidePanel, TopBottomPanel};

pub struct App {
    events: Vec<Event>,
}

struct Event {}

impl Default for App {
    fn default() -> Self {
        Self { events: Vec::new() }
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::app_key).unwrap_or_default();
        // }

        Default::default()
    }
}

impl eframe::App for App {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.heading("event engine");

                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                crate::powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });

        SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("sources");
            ui.separator();
            ui.label("(currently only supports USF events)");

            let usf_link = Hyperlink::from_label_and_url(
                "USF events calendar",
                "https://www.usfca.edu/life-usf/events",
            )
            .open_in_new_tab(true);
            ui.add(usf_link);

            if ui.button("grab events").clicked() {}
        });

        SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("filters");
            ui.separator();
            ui.separator();
            ui.heading("export");
            ui.separator();
        });

        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.label("howdy world");
            });
        });
    }
}
