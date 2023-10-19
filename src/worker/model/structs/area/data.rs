use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::sync::Arc;

use super::super::background::Background;

use super::super::character::Character;

pub type AreaRefering = Weak<RefCell<Area>>;
pub type StrongAreaRefering = Rc<RefCell<Area>>;

#[derive(Debug, Clone)]
pub struct AreaWeakList {
    data: Vec<AreaRefering>
}

impl AreaWeakList {
    fn new() -> Self {
        Self {
            data: Vec::new()
        }
    }

    pub fn clear_removed_area(&mut self) {
        self.data.retain(|weak_area| {
            weak_area.upgrade().is_some()
        });
    }

    pub fn add(&mut self, area: AreaRefering) -> Result<(), AreaError> {
        if let Option::Some(x) = area.upgrade() {
            let name = x.borrow().name.clone();
            
            if self.data.iter().any(|y| y.upgrade().unwrap().borrow().name == name) {
                return Result::Err(AreaError::AlreadyExist);
            }

            self.data.push(area);

            Result::Ok(())
        } else {
            Result::Err(AreaError::AreaNotAvailable)
        }
    }

    fn get_pos_by_name(&self, name: &String) -> Option<usize> {
        self
            .data
            .iter()
            .position(|y| y
                .upgrade()
                .unwrap()
                .borrow()
                .name == *name
            )
    }

    pub fn get_by_name(&self, name: &String) -> Option<AreaRefering> {
        self
            .get_pos_by_name(name)
            .map(|index| 
                self
                .data
                .get(index)
                .unwrap()
                .clone()
            )
    }

    fn remove(&mut self, index: usize) {        
        if index <= self.data.len() {
            self.data.remove(index);
        }
    }

    pub fn remove_by_name(&mut self, name: &String) {
        if let Option::Some(index) = self.get_pos_by_name(name) {
            self.remove(index)   
        }
    }

    pub fn iter(&self) -> core::slice::Iter<'_, AreaRefering> {
        self.data.iter()
    }
}

#[derive(Debug, Clone)]
pub enum AreaSelector {
    restricted(AreaWeakList),
    all
}

#[derive(Debug)]
pub enum AreaError {
    IsDefaultDefined,
    IsNotRestrictedArea,
    AlreadyExist,
    AreaNotAvailable,
}

impl AreaSelector {
    pub fn set_restricted() -> AreaSelector {
        AreaSelector::restricted(AreaWeakList::new())
    }
}

#[derive(Debug, Clone)]
pub enum ASOrReachable {
    reachable,
    area_selector(AreaSelector),
}

impl ASOrReachable {}

// Area data container
#[derive(Debug, Default, Clone)]
pub struct Area {
    // Room info
    pub name: String, // Necessary
    pub default_description: Option<String>,

    // Background
    background: Option<Weak<Rc<Background>>>, //Necessary
    pub cbg_allowed: Option<bool>,
    pub bglock: Option<bool>,

    background_tod: Option<Weak<Rc<Background>>>,
    pub has_lights: Option<bool>,

    // Area
    pub reachable_areas: Option<AreaSelector>,
    pub change_reachability_allowed: Option<bool>,

    pub visible_areas: Option<ASOrReachable>,
    pub scream_range: Option<ASOrReachable>,

    pub locking_allowed: Option<bool>,

    pub private_area: Option<bool>,

    pub rp_getarea_allowed: Option<bool>,
    pub rp_getareas_allowed: Option<bool>,

    pub lobby_area: Option<bool>,

    // Character
    restricted_chars: Option<Vec<Arc<Character>>>,

    // Message
    pub iniswap_allowed: Option<bool>,
    pub global_allowed: Option<bool>,

    // AFK
    afk_sendto: Option<AreaRefering>,
    pub afk_delay: Option<u32>,

    // Other
    pub song_switch_allowed: Option<bool>,
    pub rollp_allowed: Option<bool>,
    pub bullet: Option<bool>,
    pub gm_iclock_allowed: Option<bool>,
}

impl Area {
    pub fn is_valid_to_format(&self) -> bool {
        if let Option::Some(bg) = &self.background {
            return bg.upgrade().is_some();
        }

        false
    }
}

#[cfg(test)]
mod area_tests {
    #[test]
    fn area_sync() {
        use crate::worker::model::structs::area::Area;
        use crate::worker::model::structs::area::data::AreaSelector;

        use std::rc::Rc;
        use std::cell::RefCell;

        // Create area
        let bedroom = Rc::new(RefCell::new(Area {
            name: String::from("Iris bedroom"),
            reachable_areas: Option::Some(AreaSelector::set_restricted()),
            ..Area::default()
        }));

        let workshop = Rc::new(RefCell::new(Area {
            name: String::from("Iris workshop aka 404"),
            ..Area::default()
        }));

        // Push a bedroom access to workshop
        if let Option::Some(AreaSelector::restricted(ref mut y)) = bedroom.borrow_mut().reachable_areas {
            y.add(Rc::downgrade(&workshop)).unwrap();
        }

        // Get all of the accessible area
        if let Option::Some(AreaSelector::restricted(ref mut x)) = bedroom.borrow_mut().reachable_areas {
            for y in x.iter() {
                if let Option::Some(z) = y.upgrade() {
                    if let Result::Ok(aled) = z.try_borrow_mut() {
                        println!("{:#?}", aled);
                    }
                }
            }
        }

        // Debug output
        println!("{:#?}", workshop.borrow_mut().reachable_areas);
        println!("{:#?}", bedroom.borrow_mut().reachable_areas);
    }
}


// TODO: Remove that sh*t
/*
impl Default for Area {
    fn default() -> Self {
        Area {
            name: String::from(""),
            background: Option::None,
            afk_delay: Option::None,
            afk_sendto: Option::None,
            background_tod: Option::None,
            bglock: Option::None,
            bullet: Option::None,
            cbg_allowed: Option::None,
            change_reachability_allowed: Option::None,
            default_description: Option::None,
            gm_iclock_allowed: Option::None,
            iniswap_allowed: Option::None,
            global_allowed: Option::None,
            has_lights: Option::None,
            lobby_area: Option::None,
            locking_allowed: Option::None,
            private_area: Option::None,
            reachable_areas: Option::None,
            restricted_chars: Option::None,
            rollp_allowed: Option::None,
            rp_getarea_allowed: Option::None,
            rp_getareas_allowed: Option::None,
            scream_range: Option::None,
            song_switch_allowed: Option::None,
            visible_areas: Option::None,
        }
    }
}
*/
