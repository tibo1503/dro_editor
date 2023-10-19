use std::io::ErrorKind;
pub mod background;
use background::Background;

pub mod character;
use character::Character;

pub mod area;
use area::*;

use std::sync::Arc;
use std::collections::HashMap;

pub struct DRODataManager {
    areas: AreaManager,
    character: HashMap<String, Character>,
    background: HashMap<String, Background>,
}

impl DRODataManager {
    fn new() -> DRODataManager {
        DRODataManager {
            areas: AreaManager::new(),
            character: HashMap::new(),
            background: HashMap::new(),
        }
    }    
}

#[cfg(test)]
mod dro_manager_test {
    use super::DRODataManager;

    #[test]
    fn new() {
        DRODataManager::new();
    }

    #[test]
    fn getter() {
        use super::area::AreaSelector;
        use std::rc::Rc;

        let mut ddm = DRODataManager::new();

        let area_a = ddm.areas.add_by_name(&"A".to_string());
        let area_b = ddm.areas.add_by_name(&"B".to_string());
    
        let area_a_ref = area_a.upgrade().unwrap();
        
        area_a_ref.borrow_mut().name = "aled".to_string();

        area_a_ref.borrow_mut().reachable_areas = Option::Some(AreaSelector::set_restricted());
        if let Option::Some(AreaSelector::restricted(x)) = &mut area_a_ref.borrow_mut().reachable_areas {
            assert!(x.add(area_b).is_ok());
        } else {
            panic!()
        };

        if let Option::Some(AreaSelector::restricted(x)) = &mut area_a_ref.borrow_mut().reachable_areas {
            x.iter().any(|x| Rc::ptr_eq(&x.upgrade().unwrap(), &area_a_ref));
        } else {
            panic!()
        };
    }
}