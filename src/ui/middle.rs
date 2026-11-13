//! Dedicated native middle-button gesture, independent of widget click/drag capture.
use super::*;
#[derive(Default)]
pub(super) struct Input {
    pub down: bool,
    pub position: Option<Pos2>,
    pub delta: Vec2,
}
impl Input {
    pub fn feed(&mut self, input: &mut egui::RawInput) {
        self.delta = Vec2::ZERO;
        input.events.retain(|event| {
            match event {
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Middle,
                    pressed,
                    ..
                } => {
                    self.position = Some(*pos);
                    self.down = *pressed;
                    false // The central button belongs to view navigation, never editing widgets.
                }
                egui::Event::PointerMoved(pos) => {
                    if self.down
                        && let Some(previous) = self.position
                    {
                        self.delta += *pos - previous;
                    }
                    self.position = Some(*pos);
                    true
                }
                _ => true,
            }
        });
        if !input.focused {
            self.down = false;
            self.delta = Vec2::ZERO;
        }
    }
}
