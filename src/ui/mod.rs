use std::sync::Arc;

use winit::window::{Fullscreen, Window};

pub struct UI {
    window: Arc<Window>,
    egui_state: egui_winit::State,
    egui_full_output: Option<egui::FullOutput>,
}

impl UI {
    pub fn new(window: Arc<Window>) -> Self {
        let egui_ctx = egui::Context::default();
        let viewport_id = egui_ctx.viewport_id();

        let egui_state = egui_winit::State::new(
            egui_ctx, 
            viewport_id,
            &window,
            Some(window.scale_factor() as f32),
            None
        );

        Self {
            window,
            egui_state,
            egui_full_output: None,
        }
    }

    pub fn take_full_output(&mut self) -> Option<egui::FullOutput> {
        self.egui_full_output.take()
    }

    pub fn ctx(&self) -> &egui::Context {
        self.egui_state.egui_ctx()
    }

    pub fn egui_state(&self) -> &egui_winit::State {
        &self.egui_state
    }

    pub fn egui_state_mut(&mut self) -> &mut egui_winit::State {
        &mut self.egui_state
    }

    pub fn update(&mut self) {
        let raw_input = self.egui_state.take_egui_input(&self.window);
        let ctx = self.egui_state.egui_ctx();

        let egui_full_output = ctx.run(raw_input, |egui_ctx| {
            egui::TopBottomPanel::top("menu_bar").show(egui_ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |_ui| {
                    });

                    ui.menu_button("Window", |ui| {
                        let is_fullscreen = self.window.fullscreen().is_some();

                        let fullscreen_text = if is_fullscreen {
                            "Exit Fullscreen"
                        } else {
                            "Enter Fullscreen"
                        };

                        if ui.button(fullscreen_text).clicked() {
                            self.window.set_fullscreen(
                                if is_fullscreen {
                                    None 
                                } else {
                                    Some(Fullscreen::Borderless(None))
                                }
                            );
                        }
                    });

                    ui.menu_button("View", |ui| {
                        if ui.button("Model Settings").clicked() {
                        }
                    });
                });
            });
        });

        self.egui_full_output = Some(egui_full_output);
    }
}
