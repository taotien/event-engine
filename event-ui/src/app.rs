use std::collections::{HashSet, VecDeque};

use chrono::prelude::Utc;
use egui::{
    Align, CentralPanel, CollapsingHeader, Hyperlink, Layout, ScrollArea, SidePanel, TopBottomPanel,
};
use poll_promise::Promise;
use serde::{Deserialize, Serialize};

use event_scraper::{UsfEvent, USFCA_EVENTS_URL};

use crate::Scraper;

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct App {
    events: HashSet<UsfEvent>,

    #[serde(skip)]
    promise: VecDeque<Promise<ehttp::Result<Scraper>>>,
    #[serde(skip)]
    collapsed: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            events: HashSet::new(),
            promise: VecDeque::new(),
            collapsed: true,
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

            if ui.button("Fetch Events").clicked() {
                for page in 0..=6 {
                    let ctx = ctx.clone();
                    let (sender, promise) = Promise::new();
                    let request =
                        ehttp::Request::get(format!("{}?page={}", USFCA_EVENTS_URL, page));
                    ehttp::fetch(request, move |response| {
                        ctx.request_repaint();
                        let resource =
                            response.map(|response| Scraper::from_response(&ctx, response));
                        sender.send(resource);
                    });
                    self.promise.push_back(promise);
                }
            }

            let usf_link = Hyperlink::from_label_and_url(
                "source: USF Events Calendar",
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
            self.promise.retain(|promise| {
                if let Some(result) = promise.ready() {
                    match result {
                        Ok(resource) => {
                            let Scraper { text, .. } = resource;
                            if let Some(text) = &text {
                                self.events.extend(event_scraper::scrape(text).unwrap());
                            }
                        }
                        Err(e) => {
                            ui.colored_label(
                                ui.visuals().error_fg_color,
                                if e.is_empty() { "Error!" } else { e },
                            );
                        }
                    }
                    false
                } else {
                    ui.spinner();
                    true
                }
            });

            let expand_toggle = ui.toggle_value(&mut self.collapsed, "Expand all");

            ui.separator();

            ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                // self.events.sort_by(|l, r| l.time.cmp(&r.time));
                let mut events_sorted: Vec<_> = self.events.iter().collect();
                events_sorted.sort_by(|l, r| l.time.cmp(&r.time));
                for event in events_sorted {
                    CollapsingHeader::new(event.name.clone())
                        .id_source(event)
                        .open(if expand_toggle.clicked() {
                            Some(self.collapsed)
                        } else {
                            None
                        })
                        .show(ui, |ui| {
                            ui.label(event.time.clone());
                            ui.label(event.location.clone().unwrap_or("".into()));
                            ui.hyperlink_to("link", event.source.clone());
                        });
                }
            });
        });
    }
}
