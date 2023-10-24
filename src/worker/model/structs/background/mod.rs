pub mod data;
pub use data::*;

pub struct BackgroundManager {
    data: Vec<Background>
}

impl BackgroundManager {
    pub fn new() -> Self {
        Self {
            data: Vec::new()
        }
    }

    pub fn add(&mut self, bg: Background) {
        self.data.push(bg)
    }
    
    pub fn add_by_name(&mut self, name: String) {
        if !self.get_names().contains(&"".to_string()) {
            let mut bg = Background::new();
            (*bg.get_mut_data()).borrow_mut().name = name;
            self.data.push(bg);
        }
    }

    pub fn get_by_name(&mut self, name: &String) -> Option<BackgroundRef> {
        for bg in &mut self.data {
            if bg.get_mut_data().borrow().name == *name {
                return Option::Some(BackgroundRef::from(&*bg));
            }
        }

        Option::None
    }

    pub fn get_clone_vec(&self) -> Vec<BackgroundRef> {
        self.data.iter().map(BackgroundRef::from).collect()
    }

    pub fn get_names(&self) -> Vec<String> {
        self.data.iter().map(|x| x.get_data_clone().name.clone()).collect()
    }
}