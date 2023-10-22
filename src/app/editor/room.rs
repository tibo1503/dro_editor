use crate::worker::model::structs::{DRODataManager, area::StrongAreaRefering};
use super::super::side_panel::SelectedData;

use egui::*;

use super::editor_trait::*;

// -=Room editor=-
pub struct RoomEditor {
    test: String,
}

impl RoomEditor {
    pub fn new() -> Self {
        Self {
            test: "".to_string(),
        }
    }
}

impl Editor for RoomEditor {
    fn view(&mut self, dro: &mut DRODataManager, selected_data: &mut SelectedData, ui: &mut Ui) {
        egui::Grid::new("some_unique_id").show(ui, |ui| {
            // Verify if Area is available
            if let Option::Some(area_rc) = &selected_data.area {
                if let Option::Some(area_ref_cell) = area_rc.upgrade() {
                    // Display all the field
                    std::cell::RefMut::map(area_ref_cell.borrow_mut(), |area| {
                        let mut field_collect = FieldCollect::new();

                        // Room info
                        let mut field = StringField::new(
                            "Area".to_string(), 
                            &mut area.name
                        );
                        field_collect.add_field(&mut field);
                        let mut field = OptionalStringRefField::new(
                            "default_description".to_string(), 
                            &mut area.default_description, 
                            "".to_string()
                        );
                        field_collect.add_optional_field(&mut field);

                        // Background
                        let mut field = OptionalBoolRefField::new(    
                            "cbg_allowed".to_string(),
                            &mut area.cbg_allowed,
                            false
                        );
                        field_collect.add_optional_field(&mut field);

                        let mut field = OptionalBoolRefField::new(    
                            "bglock".to_string(),
                            &mut area.bglock,
                            false
                        );
                        field_collect.add_optional_field(&mut field);



                        let mut field = OptionalBoolRefField::new(
                            "has_lights".to_string(),
                            &mut area.has_lights,
                            false
                        );
                        field_collect.add_optional_field(&mut field);

                        // Area
                        //
                        let mut field = OptionalBoolRefField::new(    
                            "change_reachability_allowed".to_string(),
                            &mut area.change_reachability_allowed,
                            false
                        );
                        field_collect.add_optional_field(&mut field);

                        //
                        
                        //
                        let mut field = OptionalBoolRefField::new(    
                            "locking_allowed".to_string(),
                            &mut area.locking_allowed,
                            false
                        );
                        field_collect.add_optional_field(&mut field);

                        //
                        let mut field = OptionalBoolRefField::new(    
                            "private_area".to_string(),
                            &mut area.private_area,
                            false
                        );
                        field_collect.add_optional_field(&mut field);
                        
                        //
                        let mut field = OptionalBoolRefField::new(    
                            "rp_getarea_allowed".to_string(),
                            &mut area.rp_getarea_allowed,
                            false
                        );
                        field_collect.add_optional_field(&mut field);

                        let mut field = OptionalBoolRefField::new(    
                            "rp_getareas_allowed".to_string(),
                            &mut area.rp_getareas_allowed,
                            false
                        );
                        field_collect.add_optional_field(&mut field);
                        //
                        let mut field = OptionalBoolRefField::new(    
                            "lobby_area".to_string(),
                            &mut area.lobby_area,
                            false
                        );
                        field_collect.add_optional_field(&mut field);
                        
                        // Character

                        // Message
                        let mut field = OptionalBoolRefField::new(    
                            "iniswap_allowed".to_string(),
                            &mut area.iniswap_allowed,
                            false
                        );
                        field_collect.add_optional_field(&mut field);

                        let mut field = OptionalBoolRefField::new(    
                            "global_allowed".to_string(),
                            &mut area.global_allowed,
                            false
                        );
                        field_collect.add_optional_field(&mut field);

                        // AFK
                        
                        // Other
                        let mut field = OptionalBoolRefField::new(    
                            "song_switch_allowed".to_string(),
                            &mut area.song_switch_allowed,
                            false
                        );
                        field_collect.add_optional_field(&mut field);
                        
                        let mut field = OptionalBoolRefField::new(    
                            "rollp_allowed".to_string(),
                            &mut area.rollp_allowed,
                            false
                        );
                        field_collect.add_optional_field(&mut field);

                        let mut field = OptionalBoolRefField::new(    
                            "bullet".to_string(),
                            &mut area.bullet,
                            false
                        );
                        field_collect.add_optional_field(&mut field);

                        let mut field = OptionalBoolRefField::new(    
                            "gm_iclock_allowed".to_string(),
                            &mut area.gm_iclock_allowed,
                            false
                        );
                        field_collect.add_optional_field(&mut field);

                        // End field collect
                        field_collect.disp(ui);

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

// -=Fields=-
// Field collect
struct FieldCollect<'a> {
    fields: Vec<&'a mut dyn Field>,
    optional_fields: Vec<&'a mut dyn OptionalField>
}

impl<'a> FieldCollect<'a> {
    fn new() -> Self {
        Self {
            fields: Vec::new(),
            optional_fields: Vec::new()
        }
    }

    fn add_field(&mut self, field: &'a mut dyn Field) {
        self.fields.push(field);
    }

    fn add_optional_field(&mut self, field: &'a mut dyn OptionalField) {
        self.optional_fields.push(field);
    }

    fn disp(&mut self, ui: &mut Ui) {
        ui.label("Field");
        ui.label("value");
        ui.end_row();

        // Default fields
        for ref mut item in &mut self.fields {
            ui.label(item.get_field_name());
            item.disp(ui);
            ui.end_row();
        }

        // Added fields
        for ref mut item in &mut self.optional_fields {
            if item.disp_val() {
                ui.horizontal(|ui| {
                    let mut remove = false;
                    if ui.button("-").clicked() {
                        remove = true;
                    }
                    if remove {
                        item.disp_state(false);
                    }
                    ui.label(item.get_field_name());
                });
                item.disp(ui);
                ui.end_row()
            }
        }

        // Adding optional fields
        ui.menu_button("+".to_string(), |ui| {
            for ref mut item in &mut self.optional_fields {
                if !item.disp_val() {
                    let mut add = false;
                    if ui.button(item.get_field_name()).clicked() {
                        add = true
                    }
                    if add {
                        item.disp_state(true);
                    }
                }
            }
        });
        ui.end_row();
    }
}

// Fields
trait Field {
    fn disp(&mut self, ui: &mut Ui);
    fn get_field_name(&self) -> String;
}

struct StringField<'a> {
    data: &'a mut String,
    field_name: String
}

impl<'a> StringField<'a> {
    fn new(field_name: String, data: &'a mut String) -> StringField{
        StringField {
            data,
            field_name
        }
    }
}

impl Field for StringField<'_> {
    fn disp(&mut self, ui: &mut Ui) {
        ui.text_edit_singleline(self.data);
    }

    fn get_field_name(&self) -> String {
        self.field_name.clone()
    }
}

trait OptionalField {
    fn disp(&mut self, ui: &mut Ui);
    fn disp_val(&self) -> bool;
    fn get_field_name(&self) -> String;
    fn disp_state(&mut self, statue: bool);
}

// -- Optional
// Bool
struct OptionalBoolRefField<'a> {
    data: &'a mut Option<bool>,
    default_value: bool,
    field_name: String
}

impl<'a> OptionalBoolRefField<'a> {
    fn new(field_name: String, data: &'a mut Option<bool>, default_value: bool, ) -> Self {
        Self {
            data,
            default_value,
            field_name
        }
    }
}

impl OptionalField for OptionalBoolRefField<'_> {
    fn disp(&mut self, ui: &mut Ui) {
        if let Option::Some(ref mut data) = &mut self.data {
            ui.checkbox(data, "");
        }
    }
    
    fn disp_val(&self) -> bool {
        self.data.is_some()
    }

    fn get_field_name(&self) -> String {
        self.field_name.clone()
    }
    
    fn disp_state(&mut self, state: bool) {
        if state {
            *self.data = Option::Some(self.default_value);
        } else {
            *self.data = Option::None;
        }
    }
}

// String
struct OptionalStringRefField<'a> {
    data: &'a mut Option<String>,
    default_value: String,
    field_name: String
}

impl<'a> OptionalStringRefField<'a> {
    fn new(field_name: String, data: &'a mut Option<String>, default_value: String) -> Self {
        Self {
            data,
            default_value,
            field_name
        }
    }
}

impl OptionalField for OptionalStringRefField<'_> {
    fn disp(&mut self, ui: &mut Ui) {
        if let Option::Some(ref mut data) = &mut self.data {
            ui.text_edit_singleline(data);
        }
    }
    
    fn disp_val(&self) -> bool {
        self.data.is_some()
    }

    fn get_field_name(&self) -> String {
        self.field_name.clone()
    }
    
    fn disp_state(&mut self, state: bool) {
        if state {
            *self.data = Option::Some(self.default_value.clone());
        } else {
            *self.data = Option::None;
        }
    }
}