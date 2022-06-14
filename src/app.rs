use crate::math::{check_expression, evaluate_expression, replace_constants};
use eframe::egui;
use eframe::egui::Visuals;
use rug::Float;

pub struct App {
    expression: String,
    result: Float,
}

impl Default for App {
    fn default() -> Self {
        Self {
            expression: String::new(),
            result: Float::new(1),
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());
        Self::default()
    }

    fn calculate(&mut self) {
        let expression = replace_constants(self.expression.clone());
        if check_expression(&expression) {
            self.result = evaluate_expression(&expression);
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::warn_if_debug_build(ui);

            ui.horizontal(|ui| {
                ui.label("Expression");

                if ui.text_edit_singleline(&mut self.expression).lost_focus()
                    && ui.input().key_pressed(egui::Key::Enter)
                {
                    self.calculate();
                }
            });

            ui.horizontal(|ui| {
                if ui.button("Result").clicked() {
                    self.calculate();
                }

                ui.label(self.result.to_string_radix(10, Some(10)));
            })
        });
    }
}
