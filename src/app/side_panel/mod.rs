use crate::worker::model::structs::{
    DRODataManager, 
    area
};

use egui::*;

#[derive(Default)]
pub struct SelectedData {
    pub area: String
}

pub trait SidePanel {
    fn disp(&mut self, dro: &mut DRODataManager, selected_data: &mut SelectedData, ui: &mut Ui);
}

#[derive(Default)]
pub struct RoomSidePanel {}

impl SidePanel for RoomSidePanel {
    fn disp(&mut self, dro: &mut DRODataManager, selected_data: &mut SelectedData, ui: &mut Ui) {
        if ui.button("add").clicked() {
            let name = "".to_string();
            dro.areas.add_by_name(&name);
            selected_data.area = name;
        }
        ui.vertical(|ui| {ScrollArea::vertical().show(ui, |ui| {
            let area_list = dro.areas.get_name_list();
            for item in area_list {
                ui.selectable_value(&mut selected_data.area, item.clone(), item);
            }
        })});
    }
}