use crate::{EguiProbe, Style};

impl<T, C> EguiProbe<C> for Option<T>
where
    T: EguiProbe<C> + Default,
{
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, ctx: &mut C, style: &Style) -> egui::Response {
        option_probe_with(self, ui, style, T::default, |value, ui, style| {
            value.probe(ui, ctx, style)
        })
    }

    #[inline(always)]
    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &mut C,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut C, &mut dyn EguiProbe<C>),
    ) {
        if let Some(value) = self {
            value.iterate_inner(ui, ctx, f);
        }
    }
}

#[inline(always)]
pub fn option_probe_with<T>(
    value: &mut Option<T>,
    ui: &mut egui::Ui,
    style: &Style,
    default: impl FnOnce() -> T,
    probe: impl FnOnce(&mut T, &mut egui::Ui, &Style) -> egui::Response,
) -> egui::Response {
    let mut changed = false;
    let mut r = ui
        .horizontal(|ui| {
            let mut checked = value.is_some();

            if ui.selectable_label(!checked, "None").clicked() {
                checked = false;
            }
            if ui.selectable_label(checked, "Some").clicked() {
                checked = true;
            }

            match (checked, value.is_some()) {
                (true, false) => {
                    *value = Some(default());
                    changed = true;
                }
                (false, true) => {
                    *value = None;
                    changed = true;
                }
                _ => {}
            }

            if let Some(value) = value {
                changed |= probe(value, ui, style).changed();
            }
        })
        .response;

    if changed {
        r.mark_changed();
    }

    r
}
