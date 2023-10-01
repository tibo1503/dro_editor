use egui::*;

use crate::editor::editor_trait::*;

pub struct RoomEditor {}

impl Editor for RoomEditor {
    fn view(&mut self, ui: &mut Ui) {
        egui::Grid::new("some_unique_id").show(ui, |ui| {
            ui.label("First row, first column");
            ui.label("First row, second column");
            ui.end_row();
        
            ui.label("Second row, first column");
            ui.label("Second row, second column");
            ui.label("Second row, third column");
            ui.end_row();
        
            ui.horizontal(|ui| { ui.label("Same"); ui.label("cell"); });
            ui.label("Third row, second column");
            ui.end_row();
        });
    }

    fn get_title(&self) -> String {
        "".to_string()
    }
}