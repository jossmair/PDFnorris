use super::*;

#[derive(Default)]
pub(super) struct State {
    pub active: bool,
    pub top_visible: bool,
    pub zoom_visible: bool,
    zoom_dragging: bool,
}

impl App {
    pub(super) fn set_fullscreen(&mut self, ctx: &egui::Context, active: bool) {
        if active == self.fullscreen.active {
            return;
        }
        if active {
            if self.editor.is_none() || !self.set_tool(ctx, Tool::Hand) {
                return;
            }
            self.workspace = Workspace::Read;
        }
        self.fullscreen = State {
            active,
            ..Default::default()
        };
        ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(false));
        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(active));
        ctx.request_repaint();
    }

    pub(super) fn fullscreen_keys(&mut self, ctx: &egui::Context) {
        if self.work.discard_dialog || self.printing.open || self.recognition.is_some() {
            return;
        }
        if self.fullscreen.active && ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::Escape))
        {
            self.set_fullscreen(ctx, false);
        } else if ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::F11)) {
            self.set_fullscreen(ctx, !self.fullscreen.active);
        }
    }

    pub(super) fn fullscreen_controls(&mut self, ctx: &egui::Context) {
        let screen = ctx.content_rect();
        let pointer = ctx.pointer_hover_pos();
        self.fullscreen.top_visible = pointer.is_some_and(|p| {
            p.y <= screen.top()
                + if self.fullscreen.top_visible {
                    58.
                } else {
                    10.
                }
        });
        self.fullscreen.zoom_visible = self.fullscreen.zoom_dragging
            || pointer.is_some_and(|p| {
                p.x <= screen.left()
                    + if self.fullscreen.zoom_visible {
                        86.
                    } else {
                        10.
                    }
                    && p.y > screen.top() + 60.
            });

        // Floating controls never reserve space: revealing them must not change
        // fit-to-width, the document's scale or the current reading position.
        if self.fullscreen.top_visible {
            let width = (self.documents.len().max(1) as f32 * 186. + 12.)
                .min((screen.width() - 128.).max(180.))
                .min(944.);
            egui::Area::new(egui::Id::new("fullscreen-tabs"))
                .order(egui::Order::Foreground)
                .fixed_pos(Pos2::new(screen.center().x - width / 2., screen.top() + 8.))
                .show(ctx, |ui| {
                    floating_frame().show(ui, |ui| {
                        ui.set_width(width - 12.);
                        ui.set_min_height(34.);
                        let mut activate = None;
                        egui::ScrollArea::horizontal()
                            .id_salt("fullscreen-document-tabs")
                            .scroll_bar_visibility(
                                egui::scroll_area::ScrollBarVisibility::AlwaysHidden,
                            )
                            .show(ui, |ui| {
                                ui.spacing_mut().item_spacing.x = 6.;
                                ui.horizontal(|ui| {
                                    for index in 0..self.documents.len() {
                                        let label = self.document_label(index);
                                        let active = index == self.active_document;
                                        let response = ui
                                            .add_sized(
                                                Vec2::new(180., 34.),
                                                egui::Button::new(
                                                    egui::RichText::new(&label)
                                                        .color(if active {
                                                            icons::INK
                                                        } else {
                                                            Color32::from_rgb(91, 103, 120)
                                                        })
                                                        .size(12.),
                                                )
                                                .selected(active)
                                                .fill(if active {
                                                    Color32::from_rgb(255, 237, 219)
                                                } else {
                                                    Color32::TRANSPARENT
                                                })
                                                .stroke(Stroke::NONE)
                                                .corner_radius(8.)
                                                .truncate(),
                                            )
                                            .on_hover_text(&label);
                                        if active {
                                            ui.painter().line_segment(
                                                [
                                                    Pos2::new(
                                                        response.rect.left() + 14.,
                                                        response.rect.bottom() - 2.,
                                                    ),
                                                    Pos2::new(
                                                        response.rect.right() - 14.,
                                                        response.rect.bottom() - 2.,
                                                    ),
                                                ],
                                                Stroke::new(2., icons::ACCENT),
                                            );
                                        }
                                        if self.document_dirty(index) {
                                            ui.painter().circle_filled(
                                                response.rect.right_top() + Vec2::new(-7., 7.),
                                                2.5,
                                                Color32::from_rgb(231, 67, 79),
                                            );
                                        }
                                        if response.clicked() {
                                            activate = Some(index);
                                        }
                                    }
                                });
                            });
                        if let Some(index) = activate {
                            self.activate_document(index);
                            self.set_tool(ctx, Tool::Hand);
                            self.workspace = Workspace::Read;
                        }
                    });
                });
        }

        // Always available, including when both retractable controls are hidden.
        egui::Area::new(egui::Id::new("fullscreen-exit"))
            .order(egui::Order::Foreground)
            .fixed_pos(Pos2::new(screen.right() - 46., screen.top() + 6.))
            .show(ctx, |ui| {
                egui::Frame::new()
                    .fill(Color32::from_rgb(247, 249, 253))
                    .corner_radius(10.)
                    .inner_margin(2.)
                    .show(ui, |ui| {
                        if icons::button(
                            ui,
                            Icon::Close,
                            "Quitter le plein écran · Échap",
                            false,
                            true,
                            32.,
                        )
                        .on_hover_text("Quitter le plein écran · Échap")
                        .clicked()
                        {
                            self.set_fullscreen(ctx, false);
                        }
                    });
            });

        if self.fullscreen.zoom_visible {
            egui::Area::new(egui::Id::new("fullscreen-zoom"))
                .order(egui::Order::Foreground)
                .fixed_pos(Pos2::new(
                    screen.left() + 8.,
                    (screen.center().y - 150.).max(screen.top() + 64.),
                ))
                .show(ctx, |ui| {
                    egui::Frame::new()
                        .fill(Color32::from_rgb(247, 249, 253))
                        .stroke(Stroke::new(1., Color32::from_rgb(215, 222, 231)))
                        .corner_radius(12.)
                        .inner_margin(10.)
                        .show(ui, |ui| {
                            ui.set_width(46.);
                            ui.vertical_centered(|ui| {
                                ui.label(format!("{:.0} %", self.zoom * 100.));
                                let mut zoom = self.zoom;
                                let response = ui.add_sized(
                                    Vec2::new(28., 220.),
                                    egui::Slider::new(&mut zoom, 0.2..=4.)
                                        .logarithmic(true)
                                        .vertical()
                                        .show_value(false),
                                );
                                response.widget_info(|| {
                                    egui::WidgetInfo::slider(
                                        ui.is_enabled(),
                                        f64::from(zoom),
                                        "Zoom du PDF",
                                    )
                                });
                                self.fullscreen.zoom_dragging = response.dragged();
                                if response.changed() {
                                    self.zoom_document(zoom);
                                }
                            });
                        });
                });
        }
    }
}

fn floating_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(Color32::from_rgb(250, 251, 253))
        .stroke(Stroke::new(1., Color32::from_rgb(221, 226, 233)))
        .corner_radius(14.)
        .shadow(egui::epaint::Shadow {
            offset: [0, 3],
            blur: 14,
            spread: 0,
            color: Color32::from_black_alpha(22),
        })
        .inner_margin(6.)
}
