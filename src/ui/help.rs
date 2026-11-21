use super::*;

impl App {
    pub(super) fn help_dialog(&mut self, ctx: &egui::Context) {
        if !self.work.help_open {
            if self.work.help_was_open {
                if let Some(id) = self.work.help_return_focus.take() {
                    ctx.memory_mut(|m| m.request_focus(id));
                }
                self.work.help_was_open = false;
            }
            return;
        }
        let entering = !self.work.help_was_open;
        if entering {
            self.work.help_return_focus = ctx.memory(|m| m.focused());
            self.work.help_was_open = true;
        }
        let modal = egui::Modal::new(egui::Id::new("help"))
            .frame(
                egui::Frame::new()
                    .fill(Color32::WHITE)
                    .corner_radius(16.)
                    .inner_margin(16.),
            )
            .show(ctx, |ui| {
                let compact =
                    ctx.content_rect().height() < 420. || ctx.content_rect().width() < 600.;
                ui.set_width((ctx.content_rect().width() - 64.).clamp(120., 640.));
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                egui::Frame::new()
                    .fill(Color32::WHITE)
                    .corner_radius(12.)
                    .inner_margin(if compact { 4. } else { 16. })
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let (rect, _) =
                                ui.allocate_exact_size(Vec2::splat(40.), egui::Sense::hover());
                            icons::draw(ui.painter(), Icon::Brand, rect, Color32::WHITE);
                            ui.vertical(|ui| {
                                ui.label(
                                    egui::RichText::new("PDFnorris")
                                        .size(23.)
                                        .strong()
                                        .color(icons::INK),
                                );
                                if !compact {
                                    ui.label(
                                        egui::RichText::new("Les bons gestes, à portée de main.")
                                            .color(Color32::from_rgb(96, 109, 127)),
                                    );
                                }
                            });
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    let close = icons::button(
                                        ui,
                                        Icon::Close,
                                        "Fermer l’aide",
                                        false,
                                        true,
                                        32.,
                                    );
                                    if entering {
                                        close.request_focus();
                                    }
                                    if close.clicked() {
                                        self.work.help_open = false;
                                    }
                                },
                            );
                        });
                    });
                ui.add_space(8.);
                if compact {
                    let height = (ctx.content_rect().height() - 112.).max(24.);
                    egui::ScrollArea::vertical()
                        .id_salt("help-compact-body")
                        .max_height(height)
                        .show(ui, |ui| {
                            self.help_preferences(ui);
                            Self::help_details(ui);
                        });
                } else {
                    egui::ScrollArea::vertical()
                        .id_salt("help-details")
                        .max_height(((ctx.content_rect().height() - 64.).min(600.) - 240.).max(1.))
                        .show(ui, Self::help_details);
                    self.help_preferences(ui);
                }
            });
        if modal.should_close() {
            self.work.help_open = false;
        }
    }
    fn help_details(ui: &mut egui::Ui) {
        for (title, items) in [
            (
                "MODIFIER VOTRE DOCUMENT",
                vec![
                    (
                        Icon::Edit,
                        "Corriger un texte",
                        "Cliquez dans le texte. Double-cliquez une image pour reconnaître son texte.",
                    ),
                    (
                        Icon::Text,
                        "Ajouter du texte",
                        "Glissez pour dessiner le cadre, puis écrivez.",
                    ),
                    (
                        Icon::Select,
                        "Sélectionner et déplacer",
                        "Glissez un rectangle autour des objets, puis déplacez la sélection.",
                    ),
                    (
                        Icon::Drawing,
                        "Dessiner et créer un tableau",
                        "Choisissez une forme ; sa couleur et son épaisseur se règlent à droite.",
                    ),
                ],
            ),
            (
                "LIRE ET ORGANISER",
                vec![
                    (
                        Icon::Hand,
                        "Déplacer la vue",
                        "Avec la main, glissez la page pour parcourir le PDF.",
                    ),
                    (
                        Icon::Highlight,
                        "Surligner et commenter",
                        "Les deux outils sont disponibles en Lecture.",
                    ),
                    (
                        Icon::Page,
                        "Réorganiser les pages",
                        "Glissez les miniatures. Pour un autre PDF, survolez son onglet puis déposez entre ses miniatures.",
                    ),
                    (
                        Icon::Fullscreen,
                        "Lire en plein écran",
                        "F11 pour entrer ; Échap pour quitter. Tab révèle les onglets et le zoom au clavier ; ils apparaissent aussi au survol.",
                    ),
                ],
            ),
        ] {
            ui.add_space(8.);
            ui.label(
                egui::RichText::new(title)
                    .size(11.)
                    .strong()
                    .color(icons::INK),
            );
            ui.add_space(5.);
            for (icon, name, description) in items {
                egui::Frame::new()
                    .fill(Color32::WHITE)
                    .corner_radius(9.)
                    .stroke(Stroke::new(1., Color32::from_rgb(226, 231, 238)))
                    .inner_margin(10.)
                    .show(ui, |ui| {
                        ui.horizontal_top(|ui| {
                            let (rect, _) =
                                ui.allocate_exact_size(Vec2::splat(30.), egui::Sense::hover());
                            ui.painter().rect_filled(
                                rect.expand(3.),
                                9.,
                                Color32::from_rgb(255, 238, 219),
                            );
                            icons::draw(ui.painter(), icon, rect.shrink(2.), icons::ACCENT);
                            ui.vertical(|ui| {
                                ui.set_width((ui.available_width() - 4.).max(40.));
                                ui.label(egui::RichText::new(name).strong().color(icons::INK));
                                ui.label(description);
                            });
                        });
                    });
                ui.add_space(5.);
            }
        }
        ui.add_space(8.);
        ui.horizontal_wrapped(|ui| {
            for (icon, key, label) in [
                (Icon::Open, "Ctrl+O", "Ouvrir"),
                (Icon::Save, "Ctrl+S", "Enregistrer"),
                (Icon::Search, "Ctrl+F", "Rechercher"),
                (Icon::Undo, "Ctrl+Z", "Annuler"),
                (Icon::Redo, "Ctrl+Y", "Rétablir"),
            ] {
                ui.allocate_ui(Vec2::new(190., 30.), |ui| {
                    ui.horizontal(|ui| {
                        let (rect, _) =
                            ui.allocate_exact_size(Vec2::splat(19.), egui::Sense::hover());
                        icons::draw(ui.painter(), icon, rect, icons::ACCENT);
                        ui.label(egui::RichText::new(label).size(12.).color(icons::INK));
                        egui::Frame::new()
                            .fill(Color32::from_rgb(234, 239, 246))
                            .corner_radius(5.)
                            .inner_margin(egui::Margin::symmetric(7, 3))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new(key)
                                        .monospace()
                                        .size(11.)
                                        .color(icons::INK),
                                );
                            });
                        ui.add_space(8.);
                    });
                });
            }
        });
    }
    fn help_preferences(&mut self, ui: &mut egui::Ui) {
        ui.add_space(8.);
        self.interface_size_controls(ui);
        if !self.work.journal.entries.is_empty() {
            ui.add_space(10.);
            let (rect, response) = ui.allocate_exact_size(
                Vec2::new(ui.available_width().min(260.), 44.),
                egui::Sense::click(),
            );
            response.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::Button,
                    ui.is_enabled(),
                    "Reprendre un document",
                )
            });
            if response.has_focus() {
                response.scroll_to_me_animation(None, egui::style::ScrollAnimation::none());
            }
            let fill = if response.hovered() || response.has_focus() {
                Color32::from_rgb(255, 237, 215)
            } else {
                Color32::from_rgb(255, 247, 237)
            };
            ui.painter().rect_filled(rect, 10., fill);
            ui.painter().rect_stroke(
                rect,
                10.,
                Stroke::new(1., Color32::from_rgb(237, 185, 139)),
                egui::StrokeKind::Inside,
            );
            icons::draw(
                ui.painter(),
                Icon::Undo,
                Rect::from_min_size(rect.min + Vec2::new(12., 10.), Vec2::splat(24.)),
                icons::ACCENT,
            );
            ui.painter().text(
                rect.min + Vec2::new(46., 22.),
                egui::Align2::LEFT_CENTER,
                "Reprendre un document",
                egui::FontId::proportional(13.),
                icons::INK,
            );
            icons::draw(
                ui.painter(),
                Icon::Next,
                Rect::from_min_size(
                    Pos2::new(rect.right() - 28., rect.top() + 13.),
                    Vec2::splat(18.),
                ),
                icons::ACCENT,
            );
            if response.clicked() {
                self.work.help_open = false;
                self.work.show_recovery = true;
            }
        }
    }
}
