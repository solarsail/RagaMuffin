pub trait Widget {
    fn handle_event(&self, e: &Event) -> bool {
        true
    }

    fn draw(&self, encoder: &mut Encoder) {}
}
