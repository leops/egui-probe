use crate::{
    EguiProbe,
    collections::{DeleteMe, EguiProbeFrozen},
    option::option_probe_with,
};

impl<T, C> EguiProbe<C> for Vec<T>
where
    T: EguiProbe<C> + Default,
{
    fn probe(&mut self, ui: &mut egui::Ui, _ctx: &mut C, style: &crate::Style) -> egui::Response {
        ui.horizontal(|ui| {
            ui.weak(format!("[{}]", self.len()));
            let r = ui.small_button(style.add_button_text());
            if r.clicked() {
                self.push(T::default());
            }
        })
        .response
    }

    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &mut C,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut C, &mut dyn EguiProbe<C>),
    ) {
        let mut idx = 0;
        self.retain_mut(|value| {
            let mut item = DeleteMe {
                value,
                delete: false,
            };
            f(&format!("[{idx}]"), ui, ctx, &mut item);
            idx += 1;
            !item.delete
        });
    }
}

impl<T, C> EguiProbe<C> for EguiProbeFrozen<'_, Vec<T>>
where
    T: EguiProbe<C>,
{
    fn probe(&mut self, ui: &mut egui::Ui, _ctx: &mut C, _style: &crate::Style) -> egui::Response {
        ui.weak(format!("[{}]", self.value.len()))
    }

    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &mut C,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut C, &mut dyn EguiProbe<C>),
    ) {
        for (i, value) in self.value.iter_mut().enumerate() {
            f(&format!("[{i}]"), ui, ctx, value);
        }
    }
}

impl<T, C> EguiProbe<C> for EguiProbeFrozen<'_, Option<Vec<T>>>
where
    T: EguiProbe<C>,
{
    fn probe(&mut self, ui: &mut egui::Ui, _ctx: &mut C, style: &crate::Style) -> egui::Response {
        option_probe_with(self.value, ui, style, Vec::new, |value, ui, _style| {
            ui.weak(format!("[{}]", value.len()))
        })
    }

    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &mut C,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut C, &mut dyn EguiProbe<C>),
    ) {
        if let Some(vec) = self.value {
            for (i, value) in vec.iter_mut().enumerate() {
                f(&format!("[{i}]"), ui, ctx, value);
            }
        }
    }
}
