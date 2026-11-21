use super::*;

impl App {
    pub(super) fn shortcuts(&mut self, ctx: &egui::Context) {
        if self.page_drag.is_some() {
            return;
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::F1)) {
            self.work.help_open = !self.work.help_open;
        }
        if self.work.discard_dialog
            || self.work.help_open
            || self.work.show_recovery
            || self.work.recovered_draft.is_some()
        {
            return;
        }
        if self.printing.open || self.recognition.is_some() {
            return;
        }
        let ctrl_shift = Modifiers {
            shift: true,
            ..Modifiers::CTRL
        };
        if self.workspace == Workspace::Pages
            && ctx.input_mut(|i| i.consume_key(ctrl_shift, Key::O))
        {
            self.pick_append_pdfs();
            return;
        }
        if ctx.input_mut(|i| i.consume_key(ctrl_shift, Key::S)) {
            self.save_keep_focus(ctx, true);
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::W)) {
            self.close_document(self.active_document);
            return;
        }
        let previous = ctx.input_mut(|i| i.consume_key(ctrl_shift, Key::Tab));
        let next = ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::Tab));
        if (previous || next) && self.documents.len() > 1 {
            let count = self.documents.len();
            self.activate_document(if previous {
                (self.active_document + count - 1) % count
            } else {
                (self.active_document + 1) % count
            });
            return;
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::O)) {
            self.pick_open();
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::S)) {
            self.save_keep_focus(ctx, false);
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::P)) {
            self.request_print();
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::F)) {
            self.request_search(ctx);
        }
        if self.search && ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::Escape)) {
            self.close_search();
            ctx.memory_mut(|m| m.surrender_focus(egui::Id::new("find")));
        }
        if self.block_clipboard_shortcuts(ctx) {
            return;
        }
        if self.inline.as_ref().is_some_and(|i| !i.committed) {
            if egui::Popup::is_any_open(ctx)
                || (ctx.egui_wants_keyboard_input()
                    && !ctx.memory(|m| m.has_focus(egui::Id::new("direct-text"))))
            {
                return;
            }
            for (key, style) in [(Key::B, 0), (Key::I, 1), (Key::U, 2)] {
                if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, key)) {
                    if let Some(s) = self.inline.as_ref().map(|i| i.selection.clone()) {
                        let f = s.active_format();
                        match style {
                            0 => self.font_variant(
                                ctx,
                                !f.bold.unwrap_or(s.bold),
                                f.italic.unwrap_or(s.italic),
                            ),
                            1 => self.font_variant(
                                ctx,
                                f.bold.unwrap_or(s.bold),
                                !f.italic.unwrap_or(s.italic),
                            ),
                            _ => self.apply_format(
                                ctx,
                                pdfnorris::rich::Format {
                                    underline: Some(!f.underline.unwrap_or(false)),
                                    ..Default::default()
                                },
                                None,
                            ),
                        }
                    }
                }
            }
            if ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::Escape)) {
                self.inline = None;
            } else if ctx.input_mut(|i| {
                i.consume_key(Modifiers::NONE, Key::Enter)
                    || i.consume_key(Modifiers::CTRL, Key::Enter)
            }) {
                self.commit_text_keep_focus(ctx);
            }
            return;
        }
        if ctx.egui_wants_keyboard_input() {
            return;
        }
        if self.workspace == Workspace::Pages {
            self.page_selection_shortcuts(ctx);
            let count = self.editor.as_ref().map_or(0, |e| e.info.pages.len());
            if ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::Enter)) {
                self.open_page(self.page);
                return;
            }
            for (key, direction) in [(Key::ArrowLeft, -1_isize), (Key::ArrowRight, 1)] {
                let dest = self
                    .page
                    .saturating_add_signed(direction)
                    .min(count.saturating_sub(1));
                if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, key)) {
                    self.move_current_page(dest);
                } else if ctx.input_mut(|i| i.consume_key(Modifiers::NONE, key)) {
                    self.page_selection.replace([dest]);
                    self.page = dest;
                    self.page_destination = dest + 1;
                    self.jump = Some(dest);
                }
            }
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::Z)) {
            self.undo_redo(false);
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::Y)) {
            self.undo_redo(true);
        }
        if self.mixed_shortcuts(ctx) {
            return;
        }
        if self.tool == Tool::Draw || !self.drawing.selected.is_empty() {
            self.drawing_shortcuts(ctx);
            return;
        }
        if self.workspace == Workspace::Pages {
            return;
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::PageDown)) {
            self.jump = Some(
                self.page.saturating_add(1).min(
                    self.editor
                        .as_ref()
                        .map_or(0, |e| e.info.pages.len().saturating_sub(1)),
                ),
            );
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::PageUp)) {
            self.jump = Some(self.page.saturating_sub(1));
        }
        if ctx.input_mut(|i| {
            let key = i.consume_key(Modifiers::CTRL, Key::C);
            let copy = i.events.iter().any(|e| matches!(e, egui::Event::Copy));
            if key || copy {
                i.events.retain(|e| !matches!(e, egui::Event::Copy));
            }
            key || copy
        }) && self.editor.as_ref().is_some_and(Editor::can_copy)
        {
            if let Some((p, a, b)) = self.text_selection {
                if let Some(t) = self.text.get(&p) {
                    ctx.copy_text(t.text_of(a.min(b)..a.max(b) + 1));
                }
            } else if let Some(s) = &self.selected {
                ctx.copy_text(s.draft.clone());
            }
        }
        if ctx.input(|i| i.key_pressed(Key::Delete)) {
            self.delete();
        }
        if ctx.input(|i| i.key_pressed(Key::F2)) {
            if let Some(s) = self.selected.clone() {
                self.begin_text(ctx, s);
            }
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::F6)) {
            self.select_next(ctx);
        }
        let step = if ctx.input(|i| i.modifiers.shift) {
            10.
        } else {
            1.
        };
        for (key, delta) in [
            (Key::ArrowLeft, Vec2::new(-step, 0.)),
            (Key::ArrowRight, Vec2::new(step, 0.)),
            (Key::ArrowUp, Vec2::new(0., -step)),
            (Key::ArrowDown, Vec2::new(0., step)),
        ] {
            if ctx.input(|i| i.key_pressed(key)) {
                self.move_selected(delta);
            }
        }
    }
}
