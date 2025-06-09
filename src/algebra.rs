use egui::{Pos2, Rect, Vec2};

use crate::EguiProbe;

impl<C> EguiProbe<C> for Vec2 {
    fn probe(&mut self, ui: &mut egui::Ui, _ctx: &mut C, _style: &crate::Style) -> egui::Response {
        let mut changed = false;

        let mut r = ui
            .horizontal(|ui| {
                changed |= ui.add(egui::DragValue::new(&mut self.x)).changed();
                changed |= ui.add(egui::DragValue::new(&mut self.y)).changed();
            })
            .response;

        if changed {
            r.mark_changed();
        }

        r
    }
}

impl<C> EguiProbe<C> for Pos2 {
    fn probe(&mut self, ui: &mut egui::Ui, _ctx: &mut C, _style: &crate::Style) -> egui::Response {
        let mut changed = false;

        let mut r = ui
            .horizontal(|ui| {
                changed |= ui.add(egui::DragValue::new(&mut self.x)).changed();
                changed |= ui.add(egui::DragValue::new(&mut self.y)).changed();
            })
            .response;

        if changed {
            r.mark_changed();
        }

        r
    }
}

impl<C> EguiProbe<C> for Rect {
    fn probe(&mut self, ui: &mut egui::Ui, ctx: &mut C, style: &crate::Style) -> egui::Response {
        let mut changed = false;

        let mut r = ui
            .horizontal(|ui| {
                let top = ui.label("top");
                changed |= self
                    .min
                    .y
                    .probe(ui, ctx, style)
                    .labelled_by(top.id)
                    .changed();
                let left = ui.label("left");
                changed |= self
                    .min
                    .x
                    .probe(ui, ctx, style)
                    .labelled_by(left.id)
                    .changed();
                let bottom = ui.label("bottom");
                changed |= self
                    .max
                    .y
                    .probe(ui, ctx, style)
                    .labelled_by(bottom.id)
                    .changed();
                let right = ui.label("right");
                changed |= self
                    .max
                    .x
                    .probe(ui, ctx, style)
                    .labelled_by(right.id)
                    .changed();
            })
            .response;

        if changed {
            r.mark_changed();
        }

        r
    }
}
