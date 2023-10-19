mod editor;
use editor::{
    editor_trait::Editor,
    room::RoomEditor
};

use crate::worker::model::structs::{
    DRODataManager
};

use egui::*;

pub struct MyApp {
    dro_struct: DRODataManager,

    editor: Vec<Box<dyn Editor>>,
    selected_editor: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            dro_struct: DRODataManager::new(),

            editor: vec![Box::new(RoomEditor::new())],
            selected_editor: 0            
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.button("File");
                ui.label("Hello World!");
            })
         });
        
        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            self.room_list(ui, 300.0);

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

        self.editor[0].view(ui);

        if let Some(editor) = self.editor.get(self.selected_editor) {
            //editor.view(ui);
        }
    }

    fn room_list(&mut self, ui: &mut Ui, height: f32) {
        ui.vertical(|ui| {ScrollArea::vertical().min_scrolled_height(height).show(ui, |ui| {
            let mut truc = 5u32;
            for item in 1..=50u32 {
                ui.selectable_value(&mut truc, item, format!("Item N°{}", item));
            }
        })});
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