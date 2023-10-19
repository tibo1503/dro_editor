use crate::worker::model::structs::DRODataManager;
use super::super::side_panel::SelectedData;

use egui::*;

pub trait Editor {
    fn view(&mut self, dro: &mut DRODataManager, selected_data: &mut SelectedData, ui: &mut Ui);

    fn get_title(&self) -> String;
}