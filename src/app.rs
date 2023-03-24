use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub(crate) struct GuardianSkillReportApp {
    need_username: bool,
    valid_username: bool,
    label: String,
    value: i32,
    // user: crate::user::UserInfoCard,
}

impl Default for GuardianSkillReportApp {
    fn default() -> Self {
        Self {
            need_username: true,
            valid_username: false,
            label: "ExampleName#1234".to_string(),
            value: 32,
        }
    }
}

impl GuardianSkillReportApp {
    pub(crate) fn new(creation_context: &eframe::CreationContext) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = creation_context.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for GuardianSkillReportApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Bungie User").clicked() {
                        self.valid_username = false;
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Currently Viewing: ");
                ui.label(&self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0..=10).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });

        if true {
            self.need_username = !self.valid_username;
            egui::Window::new("Window").open(&mut self.need_username).show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Bungie Username: ");
                    ui.text_edit_singleline(&mut self.label);
                });
                if ui.button("Enter").clicked() {
                    self.valid_username = true;
                }
            });
        }
    }
}