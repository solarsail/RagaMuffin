pub struct Clickable {
    area: Rect,
    is_down: bool,
}

impl Clickable {
    fn on_clicked(&self, f: F)
        where F: FnMut()
    {
        f(ith);
    }

    fn click(&self) {}
}

impl Widget for Clickable {
    fn handle_event(&mut self, e: &Event) -> bool {
        match e {
            Event::MouseDown => {
                self.is_down = true;
                false
            }
            Event::MouseUp => {
                if self.is_down {
                    self.is_down = false;
                    self.click();
                    false
                }
            }
            _ => true,
        }
    }
}
