use crate::worker::model::structs::DRODataManager;
use super::super::side_panel::SelectedData;
use super::super::widget::field_manager::*;

use egui::*;

use super::editor_trait::*;

// -=Room editor=-
pub struct BackgroundEditor {
    field_storage: FieldStateStore
}

impl BackgroundEditor {
    pub fn new() -> Self {
        Self {
            field_storage: FieldStateStore::new()
        }
    }
}

impl Editor for BackgroundEditor {
    fn view(&mut self, dro: &mut DRODataManager, selected_data: &mut SelectedData, ui: &mut Ui) {
        ui.vertical(|ui| {ScrollArea::vertical().show(ui, |ui| {
            ui.label("Background");
        })});

        if let Option::Some(x) = &mut selected_data.background {
            let mut area = x.get_mut_data().unwrap().borrow().clone();
            ui.text_edit_singleline(&mut area.name);
            *x.get_mut_data().unwrap().borrow_mut() = area;
        }
    }
}