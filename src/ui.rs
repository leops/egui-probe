use egui::{CornerRadius, Frame, Margin, Stroke, epaint::Shadow};

use crate::{EguiProbe, num::non_negative};

impl<C> EguiProbe<C> for Stroke {
    fn probe(&mut self, ui: &mut egui::Ui, _ctx: &mut C, _style: &crate::Style) -> egui::Response {
        ui.weak("Stroke")
    }

    #[inline(always)]
    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &mut C,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut C, &mut dyn EguiProbe<C>),
    ) {
        f("color", ui, ctx, &mut self.color);
        f("width", ui, ctx, &mut non_negative(&mut self.width));
    }
}

impl<C> EguiProbe<C> for Margin {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _ctx: &mut C, _style: &crate::Style) -> egui::Response {
        ui.weak("Margin")
    }

    #[inline(always)]
    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &mut C,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut C, &mut dyn EguiProbe<C>),
    ) {
        f("top", ui, ctx, &mut non_negative(&mut self.top));
        f("left", ui, ctx, &mut non_negative(&mut self.left));
        f("bottom", ui, ctx, &mut non_negative(&mut self.bottom));
        f("right", ui, ctx, &mut non_negative(&mut self.right));
    }
}

impl<C> EguiProbe<C> for CornerRadius {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _ctx: &mut C, _style: &crate::Style) -> egui::Response {
        ui.weak("Rounding")
    }

    #[inline(always)]
    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &mut C,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut C, &mut dyn EguiProbe<C>),
    ) {
        f("nw", ui, ctx, &mut non_negative(&mut self.nw));
        f("ne", ui, ctx, &mut non_negative(&mut self.ne));
        f("sw", ui, ctx, &mut non_negative(&mut self.sw));
        f("se", ui, ctx, &mut non_negative(&mut self.se));
    }
}

impl<C> EguiProbe<C> for Shadow {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _ctx: &mut C, _style: &crate::Style) -> egui::Response {
        ui.weak("Shadow")
    }

    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &mut C,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut C, &mut dyn EguiProbe<C>),
    ) {
        f("offset", ui, ctx, &mut self.offset);
        f("blur", ui, ctx, &mut non_negative(&mut self.blur));
        f("spread", ui, ctx, &mut non_negative(&mut self.spread));
        f("color", ui, ctx, &mut self.color);
    }
}

impl<C> EguiProbe<C> for Frame {
    #[inline(always)]
    fn probe(&mut self, ui: &mut egui::Ui, _ctx: &mut C, _style: &crate::Style) -> egui::Response {
        ui.weak("Frame")
    }

    #[inline(always)]
    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &mut C,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut C, &mut dyn EguiProbe<C>),
    ) {
        f("inner_margin", ui, ctx, &mut self.inner_margin);
        f("outer_margin", ui, ctx, &mut self.outer_margin);
        f("rounding", ui, ctx, &mut self.corner_radius);
        f("shadow", ui, ctx, &mut self.shadow);
        f("fill", ui, ctx, &mut self.fill);
        f("stroke", ui, ctx, &mut self.stroke);
    }
}
