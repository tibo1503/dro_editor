use crate::worker::model::structs::DRODataManager;
use super::super::side_panel::SelectedData;
use super::super::widget::field_manager::*;

use egui::*;

use super::editor_trait::*;

// -=Room editor=-
pub struct RoomEditor<'a> {
    field_storage: FieldStateStore<'a>
}

impl RoomEditor<'_> {
    pub fn new() -> Self {
        Self {
            field_storage: FieldStateStore::new()
        }
    }
}

impl Editor for RoomEditor<'_> {
    fn view(&mut self, dro: &mut DRODataManager, selected_data: &mut SelectedData, ui: &mut Ui) {
        ui.vertical(|ui| {ScrollArea::vertical().show(ui, |ui| {
        // Verify if Area is available
            if let Option::Some(area_rc) = &selected_data.area {
                if let Option::Some(area_ref_cell) = area_rc.upgrade() {
                    let mut area = area_ref_cell.borrow().clone();
                
                    let mut field_collect = FieldCollect::new(
                        self
                            .field_storage
                            .get_search_value("FIELD_COLLECT", &"".to_string())
                    );
                
                    // Room info
                    let cat = "Room info";
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
                    field_collect.add_cat_optional_field(cat, &mut field);
                
                    // Background
                    let cat = "Background";
                    let mut field = OptionalBoolRefField::new(    
                        "cbg_allowed".to_string(),
                        &mut area.cbg_allowed,
                        false
                    );
                    field_collect.add_cat_optional_field(cat, &mut field);
                
                    let mut field = OptionalBoolRefField::new(    
                        "bglock".to_string(),
                        &mut area.bglock,
                        false
                    );
                    field_collect.add_cat_optional_field(cat, &mut field);
                
                    //
                    let mut field = OptionalBoolRefField::new(
                        "has_lights".to_string(),
                        &mut area.has_lights,
                        false
                    );
                    field_collect.add_cat_optional_field(cat, &mut field);
                
                    // Area
                    let cat = "Area";
                    //
                    let mut field = OptionalBoolRefField::new(    
                        "change_reachability_allowed".to_string(),
                        &mut area.change_reachability_allowed,
                        false
                    );
                    field_collect.add_cat_optional_field(cat, &mut field);
                
                    //
                            
                    //
                    let mut field = OptionalBoolRefField::new(    
                        "locking_allowed".to_string(),
                        &mut area.locking_allowed,
                        false
                    );
                    field_collect.add_cat_optional_field(cat, &mut field);
                
                    //
                    let mut field = OptionalBoolRefField::new(    
                        "private_area".to_string(),
                        &mut area.private_area,
                        false
                    );
                    field_collect.add_cat_optional_field(cat, &mut field);
                            
                    //
                    let mut field = OptionalBoolRefField::new(    
                        "rp_getarea_allowed".to_string(),
                        &mut area.rp_getarea_allowed,
                        false
                    );
                    field_collect.add_cat_optional_field(cat, &mut field);
                
                    let mut field = OptionalBoolRefField::new(    
                        "rp_getareas_allowed".to_string(),
                        &mut area.rp_getareas_allowed,
                        false
                    );
                    field_collect.add_cat_optional_field(cat, &mut field);
                        
                    //
                    let mut field = OptionalBoolRefField::new(    
                        "lobby_area".to_string(),
                        &mut area.lobby_area,
                        false
                    );
                    field_collect.add_cat_optional_field(cat, &mut field);
                            
                    // Character
                    let cat = "Character";
                
                    // Message
                    let cat = "Message";
                    let mut field = OptionalBoolRefField::new(    
                        "iniswap_allowed".to_string(),
                        &mut area.iniswap_allowed,
                        false
                    );
                    field_collect.add_cat_optional_field(cat, &mut field);
                
                    let mut field = OptionalBoolRefField::new(    
                        "global_allowed".to_string(),
                        &mut area.global_allowed,
                        false
                    );
                    field_collect.add_cat_optional_field(cat, &mut field);
                
                    // AFK
                    let cat = "AFK";
                    let mut field = OptionalUnsignedRefField::new(
                        "afk_delay".to_string(),
                        &mut area.afk_delay,
                        0,
                        0,
                        360
                    );
                    field_collect.add_cat_optional_field(cat, &mut field);
                
                    let mut field = OptionalAreaRefField::new(
                        "afk_sendto".to_string(),
                        &mut area.afk_sendto,
                        dro
                    );
                    field_collect.add_cat_optional_field(cat, &mut field);
                            
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
                
                    *area_ref_cell.borrow_mut() = area;
                };
            }
        })});
    }

    fn get_title(&self) -> String {
        "".to_string()
    }
}