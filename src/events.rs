pub enum Event {
    KeyDown {
        code: KeyCode,
        modifier: KeyModifier,
    },
    KeyUp,
    MouseDown,
    MouseUp,
}
