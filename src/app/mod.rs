mod editor;
use editor::{
    editor_trait::Editor,
    room::RoomEditor,
    background::BackgroundEditor
};

mod side_panel;

mod widget;

use crate::worker::model::structs::{
    DRODataManager
};

use std::collections::BTreeMap;

use egui::*;

pub struct DROEditor {
    side_panels: BTreeMap<String, Box<dyn side_panel::SidePanel>>,
    side_panel_choice: String,

    selected_area: side_panel::SelectedData,

    editor: BTreeMap<String, Box<dyn Editor>>,
    editor_choice: String,
    
    dro_struct: DRODataManager, 
}

impl Default for DROEditor {
    fn default() -> Self {
        let mut app = Self {
            side_panels: BTreeMap::new(),
            side_panel_choice: "".to_string(),

            selected_area: side_panel::SelectedData::default(),
            
            editor: BTreeMap::new(),
            editor_choice: "".to_string(),

            dro_struct: DRODataManager::new(),
        };

        // Side panels
        app.side_panels.insert("Area".to_string(), Box::new(side_panel::RoomSidePanel::default()));
        app.side_panels.insert("Background".to_string(), Box::new(side_panel::BackgroundSidePanel::default()));

        // Editor
        app.editor.insert("Area".to_string(), Box::new(RoomEditor::new()));
        app.editor.insert("Background".to_string(), Box::new(BackgroundEditor::new()));

        app
    }
}

impl eframe::App for DROEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Project", |ui| {
                    let _ = ui.button("Load");
                    let _ = ui.button("Close");
                    ui.menu_button("Previous", |ui| {
                        for projects in 1..10 {
                            let _ = ui.button(format!("Project N°{}", projects));
                        }
                    });
                });
                ui.menu_button("Settings", |ui| {
                    let _ = ui.button("Settings");
                })
            })
         });
        
        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            let mut name = self.side_panel_choice.clone();
            if name.as_str() == "" {
                name = "Choice".to_string();
            }
            ui.menu_button(name, |ui| {
                for name in self.side_panels.iter() {
                    ui.selectable_value(&mut self.side_panel_choice, name.0.clone(), name.0);
                }
            });
            
            ui.separator();
            if let Option::Some(side_panel) = self.side_panels.get_mut(&self.side_panel_choice) {
                side_panel.disp(&mut self.dro_struct, &mut self.selected_area, ui);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                for name in self.editor.iter() {
                    ui.selectable_value(&mut self.editor_choice, name.0.clone(), name.0);
                }
            });
            ui.separator();
    
            if let Option::Some(editor) = self.editor.get_mut(&self.editor_choice) {
                editor.view(&mut self.dro_struct, &mut self.selected_area, ui);
            }
        });
    }
}