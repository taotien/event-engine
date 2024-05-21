use std::{
    collections::{HashMap, HashSet, VecDeque},
    ffi::OsStr,
    fs::File,
    io::Write,
    path::Path,
};

use chrono::{prelude::Utc, Datelike, NaiveDate};
use egui::{
    Align, CentralPanel, CollapsingHeader, Hyperlink, ScrollArea, SidePanel, TopBottomPanel,
};
use egui_extras::DatePickerButton;
use egui_file::FileDialog;
use event_export::ParsedEvent;
use poll_promise::Promise;
use serde::{Deserialize, Serialize};

use event_scraper::{UsfEvent, USFCA_EVENTS_URL};

use crate::Scraper;

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct App {
    /// Stores the scraped events and whether they should be exported
    events: HashMap<UsfEvent, bool>,

    readme_open: bool,

    #[serde(skip)]
    keyword_box: String,
    keywords: HashSet<String>,
    after_date: NaiveDate,
    before_date: NaiveDate,

    #[serde(skip)]
    promise: VecDeque<Promise<ehttp::Result<Scraper>>>,
    #[serde(skip)]
    expanded: bool,

    #[serde(skip)]
    save_file_dialog: Option<FileDialog>,
}

impl Default for App {
    fn default() -> Self {
        let after_date = NaiveDate::default().with_year(2024).unwrap();
        let before_date = NaiveDate::default().with_year(2100).unwrap();
        Self {
            events: HashMap::new(),
            readme_open: true,
            keyword_box: String::new(),
            keywords: HashSet::new(),
            after_date,
            before_date,
            promise: VecDeque::new(),
            expanded: true,
            save_file_dialog: None,
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
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
        egui_extras::install_image_loaders(ctx);
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.heading("event engine");
                ui.label("(state is automaticallly saved)");
                ui.toggle_value(&mut self.readme_open, "README");
                if self.readme_open {
                    egui::Window::new("README").show(ctx, |ui| {
                        ui.label("Tao Tien - 20766272");

                        ui.label("\n");

                        ui.label("event engine is a program that fetches event listings from websites that list them, filters, and exports to your favorite calendar. Currently it only scrapes the USF Events Calendar. To get started, please run the program locally and click \"Fetch USF Events\"");

                        ui.label("\n");
                        
                        ui.label("Difficulties:");
                        ui.label("I didn't realize browser networking security would stop the web version of this program to not function, and it would be too difficult to set up a reliable proxy to get around the CORS limitations.");

                        ui.label("\n");
                        ui.label("Resources");
                        let docs_rs = Hyperlink::new("https://docs.rs").open_in_new_tab(true);
                        ui.add(docs_rs);
                    });
                }
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            crate::powered_by_egui_and_eframe(ui);
            egui::warn_if_debug_build(ui);
            let now = Utc::now();
            ui.label(format!("Copyright {}", now.format("%Y-%m-%d")));
        });

        SidePanel::left("left_panel").show(ctx, |ui| {
            ui.image("file://event-ui/assets/dorito.png");
            ui.heading("sources");
            ui.separator();
            ui.label("(currently only works in desktop mode due to USF's CORS policies)");

            // asyncronously fetch from USF Events Calendar
            if ui.button("Fetch USF Events").clicked() {
                *self = Default::default();
                for page in 0..=6 {
                    let ctx = ctx.clone();
                    let (sender, promise) = Promise::new();
                    let request =
                        ehttp::Request::get(format!("{}?page={}", USFCA_EVENTS_URL, page));
                    // request.mode = ehttp::Mode::NoCors; // TODO why doesn't this work?
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

        let mut export_clicked = false;
        let mut select_shown = false;
        SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("filters");
            ui.separator();
            ui.label("Enter keywords: ");
            let tag_editor = ui.text_edit_singleline(&mut self.keyword_box);
            ui.label("Click keywords to remove");
            // text boxes lose focus on enter pressed
            if tag_editor.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                if !self.keyword_box.is_empty() {
                    self.keywords.insert(self.keyword_box.clone());
                }
                self.keyword_box.clear();
                tag_editor.request_focus();
            };

            let mut keywords_sorted: Vec<_> = self.keywords.clone().into_iter().collect();
            keywords_sorted.sort();
            for tag in keywords_sorted {
                if ui.button(tag.clone()).clicked() {
                    self.keywords.remove(&tag);
                }
            }
            if ui.button("Clear keywords").clicked() {
                self.keywords.clear();
            }
            ui.separator();
            ui.label("After");
            let after = DatePickerButton::new(&mut self.after_date).id_source("after");
            ui.add(after);
            ui.label("Before");
            let before = DatePickerButton::new(&mut self.before_date).id_source("before");
            ui.add(before);
            if ui.button("Clear dates").clicked() {
                self.after_date = NaiveDate::default().with_year(2024).unwrap();
                self.before_date = NaiveDate::default().with_year(2100).unwrap();
            }
            ui.separator();
            select_shown = ui.button("Select shown events").clicked();
            if ui.button("Clear filters").clicked() {
                let events = self.events.iter_mut();
                for (_, s) in events {
                    *s = false;
                }
                self.keywords.clear();
                self.after_date = NaiveDate::default().with_year(2024).unwrap();
                self.before_date = NaiveDate::default().with_year(2100).unwrap();
            }
            ui.separator();

            ui.heading("export");
            ui.separator();
            export_clicked = ui.button("Export to iCal").clicked();
        });

        CentralPanel::default().show(ctx, |ui| {
            let mut expand_toggle = self.expanded;
            let mut scroll_top_clicked = false;
            let mut scroll_bottom_clicked = false;

            ui.horizontal(|ui| {
                expand_toggle = ui.toggle_value(&mut self.expanded, "Expand all").clicked();
                scroll_top_clicked = ui.button("Scroll to top").clicked();
                scroll_bottom_clicked = ui.button("Scroll to bottom").clicked();
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
                            false
                        }
                        Err(e) => {
                            ui.colored_label(
                                ui.visuals().error_fg_color,
                                if e.is_empty() { "Error!" } else { e },
                            );
                            true
                        }
                    }
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
                if scroll_top_clicked {
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

                    let mut show = false;
                    for tag in self.keywords.clone().into_iter() {
                        if name.to_lowercase().contains(&tag) {
                            show = true;
                        }
                    }
                    if (self.keywords.is_empty() || show)
                        && (time_start > &self.after_date.into()
                            && time_end < &self.before_date.into())
                    {
                        if select_shown {
                            expand_toggle = true;
                            **selected = true;
                        }
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
                }
                if scroll_bottom_clicked {
                    ui.scroll_to_cursor(Some(Align::Max));
                }
            });
        });

        if export_clicked {
            let filter = Box::new({
                move |path: &Path| -> bool { path.extension() == Some(OsStr::new("ical")) }
            });
            let mut dialog = FileDialog::save_file(None).show_files_filter(filter);
            dialog.open();
            self.save_file_dialog = Some(dialog);
        }
        if let Some(dialog) = &mut self.save_file_dialog {
            if dialog.show(ctx).selected() {
                if let Some(path) = dialog.path() {
                    let selected: Vec<_> = self
                        .events
                        .iter()
                        .filter_map(|(e, s)| {
                            if *s {
                                Some(ParsedEvent::new(e.clone()))
                            } else {
                                None
                            }
                        })
                        .collect();

                    let ical_text = event_export::export(selected);
                    let mut path = path.to_path_buf();
                    if path.extension() != Some(OsStr::new("ical")) {
                        path.set_extension("ical");
                    }
                    let mut file = File::create(path).unwrap();
                    file.write_all(ical_text.as_bytes()).unwrap();
                }
            }
        }
    }
}
