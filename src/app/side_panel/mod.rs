use egui::*;

pub trait SidePanel {
    fn disp(&mut self, ui: &mut Ui);
}

pub struct RoomSidePanel {
    selected_editor: usize
}

impl Default for RoomSidePanel {
    fn default() -> Self {
        Self {
            selected_editor: 0 
        }
    }
}

impl SidePanel for RoomSidePanel {
    fn disp(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {ScrollArea::vertical().show(ui, |ui| {
            for item in 1..=50 {
                ui.selectable_value(&mut self.selected_editor, item, format!("Item N°{}", item));
            }
        })});
    }
}