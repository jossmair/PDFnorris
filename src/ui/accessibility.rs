use super::*;

pub(super) fn load_preferences(path: &std::path::Path) -> Option<InterfaceSize> {
    use std::io::Read;
    let mut bytes = Vec::new();
    std::fs::File::open(path)
        .ok()?
        .take(4097)
        .read_to_end(&mut bytes)
        .ok()?;
    if bytes.len() > 4096 {
        return None;
    }
    serde_json::from_slice(&bytes).ok()
}

#[derive(Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(super) enum InterfaceSize {
    #[default]
    Normal,
    Large,
    ExtraLarge,
}

impl InterfaceSize {
    pub fn factor(self) -> f32 {
        match self {
            Self::Normal => 1.,
            Self::Large => 1.25,
            Self::ExtraLarge => 1.5,
        }
    }
    fn label(self) -> &'static str {
        match self {
            Self::Normal => "Normale",
            Self::Large => "Grande",
            Self::ExtraLarge => "Très grande",
        }
    }
}
impl App {
    pub(super) fn interface_size_controls(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.horizontal(|ui| {
            let (rect, _) = ui.allocate_exact_size(Vec2::splat(22.), egui::Sense::hover());
            icons::draw(ui.painter(), Icon::Text, rect, icons::ACCENT);
            ui.label(
                egui::RichText::new("Confort de lecture")
                    .strong()
                    .color(icons::INK),
            );
        });
        ui.add_space(4.);
        let mut size = self.interface_size;
        ui.horizontal_wrapped(|ui| {
            for value in [
                InterfaceSize::Normal,
                InterfaceSize::Large,
                InterfaceSize::ExtraLarge,
            ] {
                let selected = size == value;
                let button =
                    egui::Button::new(egui::RichText::new(value.label()).color(if selected {
                        Color32::WHITE
                    } else {
                        icons::INK
                    }))
                    .selected(selected)
                    .fill(if selected {
                        icons::ACCENT
                    } else {
                        Color32::from_rgb(235, 239, 245)
                    })
                    .corner_radius(8.)
                    .min_size(Vec2::new(84., 30.));
                let response = ui.add(button);
                if response.has_focus() {
                    response.scroll_to_me_animation(None, egui::style::ScrollAnimation::none());
                }
                if response.clicked() {
                    size = value;
                }
            }
        });
        if size != self.interface_size {
            self.interface_size = size;
            ui.ctx().set_zoom_factor(size.factor());
            if let Some(path) = &self.interface_preferences {
                let saved = path
                    .parent()
                    .ok_or("Dossier des préférences introuvable.".to_string())
                    .and_then(|p| std::fs::create_dir_all(p).map_err(|e| e.to_string()))
                    .and_then(|_| serde_json::to_vec(&size).map_err(|e| e.to_string()))
                    .and_then(|bytes| pdfnorris::files::atomic_write(path, &bytes));
                if saved.is_err() {
                    self.message = Some("La taille est appliquée pour cette session, mais sa mémorisation a échoué. Vérifiez l’accès au dossier de préférences.".into());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn malformed_or_oversized_preferences_do_not_change_the_default() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("interface.json");
        assert!(load_preferences(&path).is_none());
        for value in [b"\"Unknown\"".to_vec(), vec![b' '; 8192]] {
            std::fs::write(&path, value).unwrap();
            assert!(load_preferences(&path).is_none());
        }
        std::fs::write(&path, b"\"ExtraLarge\"").unwrap();
        assert!(load_preferences(&path) == Some(InterfaceSize::ExtraLarge));
    }
}
