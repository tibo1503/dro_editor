use crate::worker::model::structs::{
    DRODataManager, 
    area
};

use egui::*;

#[derive(Default)]
pub struct SelectedData {
    area: String
}

pub trait SidePanel {
    fn disp(&mut self, dro: &mut DRODataManager, selected_data: &mut SelectedData, ui: &mut Ui);
}

pub struct RoomSidePanel {
    selected_editor: usize
}

impl Default for RoomSidePanel {
    fn default() -> Self {
        Self {
            selected_editor: 0 
        }
    }
}

impl SidePanel for RoomSidePanel {
    fn disp(&mut self, dro: &mut DRODataManager, selected_data: &mut SelectedData, ui: &mut Ui) {
        if ui.button("add").clicked() {
            dro.areas.add_by_name(&"a".to_string());
        }
        ui.vertical(|ui| {ScrollArea::vertical().show(ui, |ui| {
            let area_list = dro.areas.get_name_list();
            for item in area_list {
                ui.selectable_value(&mut selected_data.area, item.clone(), item);
            }
        })});
    }
}