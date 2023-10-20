use crate::worker::model::structs::DRODataManager;
use super::super::side_panel::SelectedData;

use egui::*;

use super::editor_trait::*;

// -=Room editor=-
pub struct RoomEditor {
    test: String,
    aled: Vec<Box<dyn RoomEditorRow>>
}

impl RoomEditor {
    pub fn new() -> Self {
        Self {
            test: "".to_string(),
            aled: Vec::new(),
        }
    }
}

impl Editor for RoomEditor {
    fn view(&mut self, dro: &mut DRODataManager, selected_data: &mut SelectedData, ui: &mut Ui) {
        egui::Grid::new("some_unique_id").show(ui, |ui| {
            ui.menu_button("Add field", |ui| {
                if ui.button("Bool row").clicked() {
                    self.aled.push(Box::new(BoolRow {
                        field_name: "field name".to_string(),
                        data: false,
                        remove: false,
                    }));
                };
                if ui.button("String row").clicked() {
                    self.aled.push(Box::new(StringRow {
                        field_name: "field name".to_string(),
                        data: self.test.clone(),
                        remove: false,
                    }));
                };
            });
            ui.end_row();

            ui.label("Field");
            ui.label("value");
            ui.end_row();

            let mut remove_index: Option<usize> = Option::None;
            for (index, row_val) in self.aled.iter_mut().enumerate() {
                row_val.disp(ui);

                if row_val.remove_required() {
                    remove_index = Option::Some(index);
                }
                ui.end_row();
            }
            remove_index.map(|x| self.aled.remove(x));

            // Verify if Area is available
            if let Option::Some(area_rc) = &selected_data.area {
                if let Option::Some(area_ref_cell) = area_rc.upgrade() {
                    let area = area_ref_cell.borrow_mut();
                    std::cell::RefMut::map(area, |area| {
                        ui.horizontal(|ui| {
                            ui.label(&area.name);
                        });
                        ui.add(egui::TextEdit::singleline(&mut area.name).hint_text("Need a value"));
                        area
                    });
                };
            }
        });
    }

    fn get_title(&self) -> String {
        "".to_string()
    }
}


// -=Rows=-
trait RoomEditorRow {
    fn disp(&mut self, ui: &mut Ui);
    fn remove_required(&self) -> bool;
}

// Bool
struct BoolRow {
    field_name: String,
    data: bool,
    remove: bool
}

impl RoomEditorRow for BoolRow {
    fn disp(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("❌").clicked() {
                self.remove = true;
            };
            ui.label(&self.field_name);
        });
        ui.checkbox(&mut self.data, "");
    }

    fn remove_required(&self) -> bool {
        self.remove
    }
}

// String
struct StringRow {
    field_name: String,
    data: String,
    remove: bool
}

impl RoomEditorRow for StringRow {
    fn disp(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("❌").clicked() {
                self.remove = true;
            };
            ui.label(&self.field_name);
        });
        ui.add(egui::TextEdit::singleline(&mut self.data).hint_text("Need a value"));
    }

    fn remove_required(&self) -> bool {
        self.remove
    }
}