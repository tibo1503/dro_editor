use crate::worker::model::structs::{
    DRODataManager, 
    area::AreaRefering
};

use egui::*;
use std::collections::HashMap;

// Field collect
pub struct FieldCollect<'a> {
    fields: Vec<&'a mut dyn Field>,
    optional_fields: Vec<&'a mut dyn OptionalField>,
    search_words: &'a mut String
}

impl<'a> FieldCollect<'a> {
    pub fn new(search: &'a mut String) -> Self {
        Self {
            fields: Vec::new(),
            optional_fields: Vec::new(),
            search_words: search
        }
    }

    pub fn add_field(&mut self, field: &'a mut dyn Field) {
        self.fields.push(field);
    }

    pub fn add_optional_field(&mut self, field: &'a mut dyn OptionalField) {
        self.optional_fields.push(field);
    }

    pub fn disp(&mut self, ui: &mut Ui) {
        egui::Grid::new("some_unique_id").show(ui, |ui| {
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
        });

        // Adding optional fields
        ui.menu_button("+".to_string(), |ui| {
            ui.text_edit_singleline(self.search_words);
            for ref mut item in &mut self.optional_fields {
                if !item.disp_val() && item.get_field_name().to_lowercase().contains(&self.search_words.to_lowercase()) {
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
pub trait Field {
    fn disp(&mut self, ui: &mut Ui);
    fn get_field_name(&self) -> String;
}

pub struct StringField<'a> {
    data: &'a mut String,
    field_name: String
}

impl<'a> StringField<'a> {
    pub fn new(field_name: String, data: &'a mut String) -> StringField{
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

pub trait OptionalField {
    fn disp(&mut self, ui: &mut Ui);
    fn disp_val(&self) -> bool;
    fn get_field_name(&self) -> String;
    fn disp_state(&mut self, statue: bool);
}

// -- Optional
// Bool
pub struct OptionalBoolRefField<'a> {
    data: &'a mut Option<bool>,
    default_value: bool,
    field_name: String
}

impl<'a> OptionalBoolRefField<'a> {
    pub fn new(field_name: String, data: &'a mut Option<bool>, default_value: bool, ) -> Self {
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
pub struct OptionalStringRefField<'a> {
    data: &'a mut Option<String>,
    default_value: String,
    field_name: String
}

impl<'a> OptionalStringRefField<'a> {
    pub fn new(field_name: String, data: &'a mut Option<String>, default_value: String) -> Self {
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

// U32
pub struct OptionalUnsignedRefField<'a> {
    data: &'a mut Option<u32>,
    default_value: u32,
    min: u32,
    max: u32,

    field_name: String
}

impl<'a> OptionalUnsignedRefField<'a> {
    pub fn new(field_name: String, data: &'a mut Option<u32>, default_value: u32, min: u32, max: u32) -> Self {
        Self {
            data,
            default_value,
            field_name,
            min,
            max
        }
    }
}

impl OptionalField for OptionalUnsignedRefField<'_> {
    fn disp(&mut self, ui: &mut Ui) {
        if let Option::Some(ref mut data) = &mut self.data {
            ui.add(egui::Slider::new(data, self.min..=self.max));
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

// Any Area
pub struct OptionalAreaRefField<'a> {
    data: &'a mut Option<AreaRefering>,
    dro: &'a DRODataManager,

    field_name: String
}

impl<'a> OptionalAreaRefField<'a> {
    pub fn new(field_name: String, data: &'a mut Option<AreaRefering>, dro: &'a DRODataManager) -> Self {
        Self {
            data,
            field_name,
            dro
        }
    }
}

impl OptionalField for OptionalAreaRefField<'_> {
    fn disp(&mut self, ui: &mut Ui) {
        let mut menu_name = "Select Area".to_string();
        let current_selected_area_name: Option<String> = if let Option::Some(x) = self.data.as_mut() {
            if let Option::Some(x) = x.upgrade() {
                let name = x.borrow().name.clone();
                menu_name = name.clone();
                Option::Some(name)
            } else {
                Option::None
        }
        } else {
            Option::None
        };
        ui.menu_button(menu_name, |ui| {
            for name in self.dro.areas.get_name_list() {
                let selected = current_selected_area_name == Option::Some(name.clone());
                if ui.selectable_label(selected, &name).clicked() {
                    *self.data = self.dro.areas.get_by_name(&name);
                }
            }
        });
    }
    
    fn disp_val(&self) -> bool {
        self.data.is_some()
    }

    fn get_field_name(&self) -> String {
        self.field_name.clone()
    }
    
    fn disp_state(&mut self, state: bool) {
        if state {
            *self.data = Option::Some(std::rc::Weak::new());
        } else {
            *self.data = Option::None;
        }
    }
}

// -=State field store=-
pub struct FieldStateStore<'a> {
    search: HashMap<&'a str, String>,
}

impl<'a> FieldStateStore<'a> {
    pub fn new() -> Self {
        Self {
            search: HashMap::new(),
        }
    }
}

impl<'a> FieldStateStore<'a> {
    pub fn get_search_value(&mut self, key: &'a str, default: &String) -> &mut String {
        self.search.entry(key).or_insert(default.clone())
    }
}