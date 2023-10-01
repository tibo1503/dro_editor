use egui::*;

pub trait Editor {
    fn view(&mut self, ui: &mut Ui);

    fn get_title(&self) -> String;
}