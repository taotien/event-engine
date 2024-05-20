use egui::{
    Align, CentralPanel, Hyperlink, Layout, ScrollArea, SidePanel, Spinner, TopBottomPanel,
};
use chrono::prelude::Utc;
use serde::{Deserialize, Serialize};

use event_scraper::{Event, USFCA_EVENTS};

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct App {
    events: Vec<Event>,

    #[serde(skip)]
    grabbing: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            events: Vec::new(),
            grabbing: false,
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
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
            ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                let now = Utc::now();
                ui.label(format!("Copyright {}", now.format("%Y-%m-%d")));
                crate::powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
                if cfg!(debug_assertions) {
                    if ui.button("reset").clicked() {
                        *self = Default::default();
                    }
                }
            });
        });

        SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("sources");
            ui.separator();
            // ui.label("(currently only supports USF events)");

            if ui.button("USF Events").clicked() {
                self.grabbing = true;
                // let client = Client::new();

                // for postfix in event_scraper::pages(&client).await? {
                //     let page = format!("{}{}", USFCA_EVENTS, postfix);
                //     let events = event_scraper::scrape(&client, page.parse()?).await?;

                //     println!("{:#?}", events);
                // }
            }

            let usf_link = Hyperlink::from_label_and_url(
                "source: USF Events Calendar (new tab)",
                "https://www.usfca.edu/life-usf/events",
            )
            .open_in_new_tab(true);
            ui.add(usf_link);
        });

        SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("filters");
            ui.separator();
            ui.separator();
            ui.heading("export");
            ui.separator();
        });

        CentralPanel::default().show(ctx, |ui| {
            if self.grabbing {
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    //     ui.horizontal(|ui| {
                    ui.label("loading events");
                    ui.add(Spinner::new());
                    //     });
                });
            }
            ScrollArea::vertical().show(ui, |ui| {
                for event in &mut self.events {
                    ui.group(|ui| {
                        ui.label(event.name.clone());
                        // ui.checkbox(&mut event.export, "")
                    });
                }
            });
        });
    }
}
