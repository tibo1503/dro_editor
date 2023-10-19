use egui::*;

use super::editor_trait::*;

pub struct RoomEditor {
    test: String,
    aled: Vec<Box<dyn RoomEditorRow>>
}

impl RoomEditor {
    pub fn new() -> Self {
        Self {
            test: "ALED".to_string(),
            aled: Vec::new(),
        }
    }
}

impl Editor for RoomEditor {
    fn view(&mut self, ui: &mut Ui) {
        egui::Grid::new("some_unique_id").show(ui, |ui| {
            ui.add(egui::TextEdit::singleline(&mut self.test).hint_text("Write something here"));
            if ui.button("aled").clicked() {
                //self.aled.push(self.test.clone());
                self.aled.push(Box::new(BoolRow {
                    field_name: "caca".to_string(),
                    data: false,
                    remove: false,
                }));
            };
            if ui.button("woula").clicked() {
                //self.aled.push(self.test.clone());
                self.aled.push(Box::new(StringRow {
                    field_name: "caca".to_string(),
                    data: self.test.clone(),
                    remove: false,
                }));
            };
            ui.end_row();

            ui.label("Field");
            ui.label("Action what");
            ui.end_row();

            let mut remove_index: Option<usize> = Option::None;
            for (index, row_val) in self.aled.iter_mut().enumerate() {
                row_val.disp(ui);

                if row_val.remove_required() {
                    remove_index = Option::Some(index);
                }

                //ui.horizontal(|ui| {
                //    if ui.button("❌").clicked() {
                //        remove_index = Option::Some(index);
                //    };
                //    ui.label(line_str.clone());
                //});
                //ui.add(egui::TextEdit::singleline(line_str).hint_text("Write something here"));

                //ui.end_row();
            }
            remove_index.map(|x| self.aled.remove(x));

            /*
            ui.label(self.test.as_str());
            ui.label("First row, second column");
            ui.end_row();
        
            ui.label("Second row, first column");
            ui.label("Second row, second column");
            //ui.label("Second row, third column");
            ui.end_row();
        
            ui.horizontal(|ui| { ui.label("Same"); ui.label("cell"); });
            ui.label(self.test.as_str());
            ui.end_row();
            */
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
                //remove_index = Option::Some(index);
                self.remove = true;
            };
            ui.label(self.field_name.clone());
        });
        ui.checkbox(&mut self.data, "");

        ui.end_row();
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
                //remove_index = Option::Some(index);
                self.remove = true;
            };
            ui.label(self.field_name.clone());
        });
        ui.add(egui::TextEdit::singleline(&mut self.data).hint_text("Need a value"));

        ui.end_row();
    }

    fn remove_required(&self) -> bool {
        self.remove
    }
}