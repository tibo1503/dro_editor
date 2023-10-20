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
    pub fn get_name(&self) -> Option<String> {
        if let Option::Some(area_refer) = &self.area {
            if let Option::Some(area) = &area_refer.upgrade() {
                Option::Some(area.borrow().name.clone())
            } else {
                Option::None
            }
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
            selected_data.area = dro.areas.add_by_name(&name);
        }
        ui.separator();

        ui.vertical(|ui| {ScrollArea::vertical().show(ui, |ui| {
            let current_area = selected_data.get_name();
            let area_list = dro.areas.get_name_list();
            for item in area_list {
                //ui.selectable_value(&mut selected_data.area, item.clone(), item);
                let color = if Option::Some(item.clone()) == current_area {
                    Color32::RED
                } else {
                    Color32::WHITE
                };

                if ui.button(RichText::new(&item).color(color)).clicked() {
                    selected_data.area = dro.areas.get_by_name(&item);
                }
            }
        })});
    }
}