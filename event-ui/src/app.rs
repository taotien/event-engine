use std::collections::{HashMap, VecDeque};

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
    events: HashMap<UsfEvent, bool>,

    #[serde(skip)]
    promise: VecDeque<Promise<ehttp::Result<Scraper>>>,
    #[serde(skip)]
    expanded: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            events: HashMap::new(),
            promise: VecDeque::new(),
            expanded: true,
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
                ui.label("(state is automaticallly saved)");
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                let now = Utc::now();
                ui.label(format!("Copyright {}", now.format("%Y-%m-%d")));
                crate::powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });

        SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("sources");
            ui.separator();
            // ui.label("(currently only supports USF events)");

            if ui.button("Fetch Events").clicked() {
                *self = Default::default();
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
            if ui.button("Export selected").clicked() {}
        });

        CentralPanel::default().show(ctx, |ui| {
            let mut expand_toggle = self.expanded;
            let mut scroll_top = false;
            let mut scroll_bottom = false;

            ui.horizontal(|ui| {
                expand_toggle = ui.toggle_value(&mut self.expanded, "Expand all").clicked();
                scroll_top = ui.button("Scroll to top").clicked();
                scroll_bottom = ui.button("Scroll to bottom").clicked();
            });

            ui.separator();

            self.promise.retain(|promise| {
                if let Some(result) = promise.ready() {
                    match result {
                        Ok(resource) => {
                            let Scraper { text, .. } = resource;
                            if let Some(text) = &text {
                                self.events.extend(
                                    event_scraper::scrape(text)
                                        .unwrap()
                                        .into_iter()
                                        .map(|e| (e, false)),
                                );
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
                    true
                }
            });

            if !self.promise.is_empty() {
                ui.spinner();
            }
            ScrollArea::both().auto_shrink(false).show(ui, |ui| {
                let mut events_sorted: Vec<_> = self.events.iter_mut().collect();
                events_sorted.sort_by(|(l, _), (r, _)| l.time_start.cmp(&r.time_start));
                if scroll_top {
                    ui.scroll_to_cursor(Some(Align::Min));
                }
                for (event, selected) in events_sorted.iter_mut() {
                    let UsfEvent {
                        name,
                        time_start,
                        time_end,
                        location,
                        source,
                    } = event;

                    CollapsingHeader::new(name)
                        .id_source(event)
                        .open(if expand_toggle {
                            Some(self.expanded)
                        } else {
                            None
                        })
                        .show(ui, |ui| {
                            ui.label(format!("Start: {}", time_start));
                            ui.label(format!("End: {}", time_end));
                            ui.label(location.clone().unwrap_or("".into()));
                            ui.hyperlink_to("link", source);
                            ui.checkbox(selected, "");
                        });
                }
                if scroll_bottom {
                    ui.scroll_to_cursor(Some(Align::Max));
                }
            });
        });
    }
}
