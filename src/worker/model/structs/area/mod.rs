mod data;
pub use data::*;

use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct AreaManager {
    data: Vec<StrongAreaRefering>
}

impl AreaManager {
    pub fn new() -> AreaManager {
        AreaManager { data: Vec::new() }
    }

    pub fn add_by_name(&mut self, name: &String) -> Option<AreaRefering> {        
        let mut area = Area::default();
        area.name = name.clone();

        if self.get_by_name(name).is_some() {
            return Option::None
        }

        let area_rc = Rc::new(RefCell::new(area));
        
        let weak = Rc::downgrade(&area_rc);
        self.data.push(area_rc);
        Option::Some(weak)
    }

    fn get_pos_by_name(&self, name: &String) -> Option<usize> {
        self
        .data
        .iter()
        .position(|y| y
            .borrow()
            .name == *name
        )
    }

    fn remove_by_index(&mut self, index: usize) {
        if index <= self.data.len() {
            self.data.remove(index);
        }
    }

    pub fn remove_by_name(&mut self, name: &String) -> Result<(),()> {
        if let Option::Some(index) = self.get_pos_by_name(name) {
            self.remove_by_index(index);
            return Result::Ok(())
        } else {
            Result::Err(())
        }
    }

    pub fn get_by_name(&self, name: &String) -> Option<AreaRefering> {
        self.get_pos_by_name(name).map(|index| Rc::downgrade(&self.data[index]))
    }

    pub fn get_name_list(&self) -> Vec<String> {
        self.data.iter().map(|x| x.borrow().name.clone()).collect()
    }
}

#[cfg(test)]
mod tests_area_manager {
    use super::AreaManager;
    use std::rc::{Rc};

    #[test]
    fn new() {
        AreaManager::new();
    }

    #[test]
    fn add() {
        let mut manager = AreaManager::new();

        // Adding A and B
        let a_name = "A".to_string();
        let a = manager.add_by_name(&"A".to_string()).unwrap();
        
        let b_name = "B".to_string();
        let b = manager.add_by_name(&"B".to_string()).unwrap();

        // Verify getter
        assert!(Rc::ptr_eq(
            &a.upgrade().unwrap(), 
            &manager.get_by_name(&a_name).unwrap().upgrade().unwrap()
        ));
        assert!(Rc::ptr_eq(
            &b.upgrade().unwrap(), 
            &manager.get_by_name(&b_name).unwrap().upgrade().unwrap()
        ));

        // Remove A
        assert_eq!(Result::Ok(()), manager.remove_by_name(&a_name));
        
        // Verify it doesn't exist
        assert!(a.upgrade().is_none());
    }
}