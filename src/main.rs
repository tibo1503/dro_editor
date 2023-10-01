#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use editor::{
    editor_trait::Editor,
    room::RoomEditor
};
use eframe::egui;
use egui::*;

mod editor;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 300.0)),
        ..Default::default()
    };
    eframe::run_native(
        "DRO Dofe's Editor",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    name: String,
    age: u32,
    editor: Vec<Box<dyn Editor>>,
    selected_editor: usize
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            editor: vec![Box::new(RoomEditor{})],
            selected_editor: 0
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                self.room_list(ui, 300.0);
                self.right_panel(ui);
            });
        });
    }
}

impl MyApp {
    fn right_panel(&mut self, ui: &mut Ui) {
        //self.hello_world(ui);

        let mut editor = RoomEditor{};
        editor.view(ui);

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


    fn hello_world(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
    
            ui.image(egui::include_image!(
                "../assets/edit_Picasso.png"
            ));
        });
    }
}