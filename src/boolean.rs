use egui::StrokeKind;

use crate::{BooleanStyle, EguiProbe, Style, option_probe_with};

pub struct ToggleSwitch<'a, T>(pub &'a mut T);

impl<C> EguiProbe<C> for bool {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _ctx: &mut C, style: &Style) -> egui::Response {
        match style.boolean {
            BooleanStyle::Checkbox => ui.add(egui::Checkbox::without_text(self)),
            BooleanStyle::ToggleSwitch => toggle_switch(self, ui),
        }
    }
}

impl<C> EguiProbe<C> for ToggleSwitch<'_, bool> {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _ctx: &mut C, _style: &Style) -> egui::Response {
        toggle_switch(self.0, ui)
    }
}

impl<C> EguiProbe<C> for ToggleSwitch<'_, Option<bool>> {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _ctx: &mut C, style: &Style) -> egui::Response {
        option_probe_with(
            self.0,
            ui,
            style,
            || false,
            |value, ui, _style| toggle_switch(value, ui),
        )
    }
}

/// Shows a toggle switch.
/// <https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/toggle_switch.rs>
pub fn toggle_switch(on: &mut bool, ui: &mut egui::Ui) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, true, *on, ""));

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter().rect(
            rect,
            radius,
            visuals.bg_fill,
            visuals.bg_stroke,
            StrokeKind::Inside,
        );
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    response
}
