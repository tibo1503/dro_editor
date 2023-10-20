use crate::worker::model::structs::{
    DRODataManager, 
    area::{self, AreaRefering}
};

use egui::*;

#[derive(Default)]
pub struct SelectedData {
    pub area: Option<AreaRefering>
}

impl SelectedData { 
    pub fn get_area_name(&self) -> Option<String> {
        if let Option::Some(area_refer) = &self.area {
            area_refer.upgrade().as_ref().map(|area| area.borrow().name.clone())
        } else {
            Option::None
        }
    }
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

            let new_area = dro.areas.add_by_name(&name);
            if new_area.is_some() {
                selected_data.area = new_area;
            }
        }
        ui.separator();

        ui.vertical(|ui| {ScrollArea::vertical().show(ui, |ui| {
            let area_list = dro.areas.get_name_list();
            for item in area_list {
                let is_selected = Option::Some(item.clone()) == selected_data.get_area_name();
                if ui.selectable_label(is_selected, &item).clicked() {
                    selected_data.area = dro.areas.get_by_name(&item);
                }
            }
        })});
    }
}