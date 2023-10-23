mod editor;
use editor::{
    editor_trait::Editor,
    room::RoomEditor
};

mod side_panel;

mod widget;

use crate::worker::model::structs::{
    DRODataManager
};

use egui::*;

pub struct MyApp {
    side_panels: Vec<Box<dyn side_panel::SidePanel>>,
    selected_area: side_panel::SelectedData,

    editor: Vec<Box<dyn Editor>>,
    
    dro_struct: DRODataManager, 
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            side_panels: vec![Box::new(side_panel::RoomSidePanel::default())],
            selected_area: side_panel::SelectedData::default(),
            
            editor: vec![Box::new(RoomEditor::new())],

            dro_struct: DRODataManager::new(),
        }
    }
}

impl eframe::App for MyApp {
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
            self.side_panels[0].disp(&mut self.dro_struct, &mut self.selected_area, ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.right_panel(ui);
        });
    }
}

impl MyApp {
    fn right_panel(&mut self, ui: &mut Ui) {
        //self.hello_world(ui);

        if self.editor.is_empty() {
            self.editor.push(Box::new(RoomEditor::new()));
        }

        self.editor[0].view(&mut self.dro_struct, &mut self.selected_area, ui);
    }

    // TODO: Remove code sample
    //fn hello_world(&mut self, ui: &mut Ui) {
    //    ui.vertical(|ui| {
    //        ui.heading("My egui Application");
    //        ui.horizontal(|ui| {
    //            let name_label = ui.label("Your name: ");
    //            ui.text_edit_singleline(&mut self.name)
    //                .labelled_by(name_label.id);
    //        });
    //        ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
    //        if ui.button("Click each year").clicked() {
    //            self.age += 1;
    //        }
    //        ui.label(format!("Hello '{}', age {}", self.name, self.age));
    //
    //        ui.image(egui::include_image!(
    //            "../../assets/edit_Picasso.png"
    //        ));
    //    });
    //}
}