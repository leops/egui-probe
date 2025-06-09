use crate::{EguiProbe, Style, option::option_probe_with};

impl<C> EguiProbe<C> for String {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _ctx: &mut C, _: &Style) -> egui::Response {
        ui.add(egui::TextEdit::singleline(self))
    }
}

impl<C> EguiProbe<C> for &str {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _ctx: &mut C, _: &Style) -> egui::Response {
        ui.add(egui::TextEdit::singleline(self))
    }
}

/// Wrapper for string-like types to show multiline text field.
pub struct EguiProbeMultiline<'a, T> {
    pub string: &'a mut T,
}

impl<C> EguiProbe<C> for EguiProbeMultiline<'_, String> {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _ctx: &mut C, _: &Style) -> egui::Response {
        ui.add(egui::TextEdit::multiline(self.string))
    }
}

impl<C> EguiProbe<C> for EguiProbeMultiline<'_, &str> {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _ctx: &mut C, _: &Style) -> egui::Response {
        ui.add(egui::TextEdit::multiline(self.string))
    }
}

impl<C> EguiProbe<C> for EguiProbeMultiline<'_, Option<String>> {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, ctx: &mut C, style: &Style) -> egui::Response {
        option_probe_with(
            self.string,
            ui,
            ctx,
            style,
            |_| String::new(),
            |string, ui, _, _| ui.add(egui::TextEdit::multiline(string)),
        )
    }
}

impl<C> EguiProbe<C> for EguiProbeMultiline<'_, Option<&str>> {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, ctx: &mut C, style: &Style) -> egui::Response {
        option_probe_with(
            self.string,
            ui,
            ctx,
            style,
            |_| "",
            |string, ui, _, _| ui.add(egui::TextEdit::multiline(string)),
        )
    }
}
