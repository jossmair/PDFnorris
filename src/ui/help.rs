use super::*;

impl App {
    pub(super) fn help_dialog(&mut self, ctx: &egui::Context) {
        if !self.work.help_open {
            return;
        }
        let modal = egui::Modal::new(egui::Id::new("help")).show(ctx, |ui| {
            ui.set_width((ctx.content_rect().width() - 40.).clamp(120., 680.));
            egui::Frame::new().fill(icons::RAIL).corner_radius(12.).inner_margin(16.).show(ui, |ui| {
                ui.horizontal(|ui| {
                    let (rect, _) = ui.allocate_exact_size(Vec2::splat(40.), egui::Sense::hover());
                    icons::draw(ui.painter(), Icon::Brand, rect, Color32::WHITE);
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("PDFnorris").size(23.).strong().color(Color32::WHITE));
                        ui.label(egui::RichText::new("Les bons gestes, à portée de main.").color(Color32::from_rgb(206, 215, 228)));
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if icons::button(ui, Icon::Close, "Fermer l’aide", false, true, 32.).clicked() { self.work.help_open = false; }
                    });
                });
            });
            ui.add_space(10.);
            egui::ScrollArea::vertical().id_salt("help-details")
                .max_height((ctx.content_rect().height() - 240.).max(40.))
                .show(ui, |ui| {
                    for (title, items) in [
                        ("MODIFIER VOTRE DOCUMENT", vec![
                            (Icon::Edit, "Corriger un texte", "Cliquez dans le texte. Double-cliquez une image pour reconnaître son texte."),
                            (Icon::Text, "Ajouter du texte", "Glissez pour dessiner le cadre, puis écrivez."),
                            (Icon::Select, "Sélectionner et déplacer", "Glissez un rectangle autour des objets, puis déplacez la sélection."),
                            (Icon::Drawing, "Dessiner et créer un tableau", "Choisissez une forme ; sa couleur et son épaisseur se règlent à droite.")]),
                        ("LIRE ET ORGANISER", vec![
                            (Icon::Hand, "Déplacer la vue", "Avec la main, glissez la page pour parcourir le PDF."),
                            (Icon::Highlight, "Surligner et commenter", "Les deux outils sont disponibles en Lecture."),
                            (Icon::Page, "Réorganiser les pages", "Glissez les miniatures. Pour un autre PDF, survolez son onglet puis déposez entre ses miniatures."),
                            (Icon::Fullscreen, "Lire en plein écran", "F11 pour entrer ; Échap pour quitter. Onglets en haut, zoom à gauche au survol.")]),
                    ] {
                        ui.add_space(8.);
                        ui.label(egui::RichText::new(title).size(11.).strong().color(icons::INK));
                        ui.add_space(5.);
                        for (icon, name, description) in items {
                            egui::Frame::new().fill(Color32::WHITE).corner_radius(9.)
                                .stroke(Stroke::new(1., Color32::from_rgb(226, 231, 238)))
                                .inner_margin(10.).show(ui, |ui| {
                                    ui.horizontal_top(|ui| {
                                        let (rect, _) = ui.allocate_exact_size(Vec2::splat(30.), egui::Sense::hover());
                                        ui.painter().rect_filled(rect.expand(3.), 9., Color32::from_rgb(255, 238, 219));
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
                        for (icon, key, label) in [(Icon::Open, "Ctrl+O", "Ouvrir"), (Icon::Save, "Ctrl+S", "Enregistrer"), (Icon::Search, "Ctrl+F", "Rechercher"), (Icon::Undo, "Ctrl+Z", "Annuler"), (Icon::Redo, "Ctrl+Y", "Rétablir")] {
                            ui.horizontal(|ui| {
                                let (rect, _) = ui.allocate_exact_size(Vec2::splat(19.), egui::Sense::hover());
                                icons::draw(ui.painter(), icon, rect, icons::ACCENT);
                                ui.label(egui::RichText::new(label).size(12.).color(icons::INK));
                                egui::Frame::new().fill(Color32::from_rgb(234, 239, 246))
                                    .corner_radius(5.).inner_margin(egui::Margin::symmetric(7, 3))
                                    .show(ui, |ui| { ui.label(egui::RichText::new(key).monospace().size(11.).color(icons::INK)); });
                                ui.add_space(8.);
                            });
                        }
                    });
                });
            ui.add_space(8.);
            self.interface_size_controls(ui);
            if !self.work.journal.entries.is_empty() && ui.add(egui::Button::new("Documents à reprendre →").fill(Color32::from_rgb(255, 239, 223)).corner_radius(8.).min_size(Vec2::new(0., 32.))).clicked() {
                self.work.help_open = false;
                self.work.show_recovery = true;
            }
        });
        if modal.should_close() {
            self.work.help_open = false;
        }
    }
}
