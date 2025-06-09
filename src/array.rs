use crate::EguiProbe;

impl<T, C, const N: usize> EguiProbe<C> for [T; N]
where
    T: EguiProbe<C>,
{
    fn probe(&mut self, ui: &mut egui::Ui, _ctx: &mut C, _style: &crate::Style) -> egui::Response {
        ui.weak(format!("[{N}]"))
    }

    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &mut C,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut C, &mut dyn EguiProbe<C>),
    ) {
        for (i, value) in self.iter_mut().enumerate() {
            f(&format!("[{i}]"), ui, ctx, value);
        }
    }
}

impl<T, C> EguiProbe<C> for [T]
where
    T: EguiProbe<C>,
{
    fn probe(&mut self, ui: &mut egui::Ui, _ctx: &mut C, _style: &crate::Style) -> egui::Response {
        ui.weak(format!("[{}]", self.len()))
    }

    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &mut C,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut C, &mut dyn EguiProbe<C>),
    ) {
        for (i, value) in self.iter_mut().enumerate() {
            f(&format!("[{i}]"), ui, ctx, value);
        }
    }
}
