use std::cell::{RefMut, RefCell};
use std::rc::{Rc, Weak};

use std::path::Path;

#[derive(Debug, Clone)]
pub struct Background {
    data: Rc<Rc<RefCell<BackgroundData>>>
}

impl Background {
    pub fn new() -> Self {
        Background {
            data: Rc::new(Rc::new(RefCell::new(
                BackgroundData {
                    ..BackgroundData::default()
                }
            )))
        }
    }

    pub fn get_mut_data(&mut self) -> Rc<RefCell<BackgroundData>> {
        Rc::clone(&*self.data)
    }

    pub fn get_data_clone(&self) -> BackgroundData {
        (**self.data).borrow().clone()
    }
}

pub struct BackgroundRef {
    data: Weak<Rc<RefCell<BackgroundData>>>
}

impl From<&Background> for BackgroundRef {
    fn from(bg: &Background) -> BackgroundRef {
        BackgroundRef { 
            data: Rc::downgrade(&bg.data) 
        }
    }
}

impl BackgroundRef {
    pub fn get_mut_data(&mut self) -> Option<Rc<RefCell<BackgroundData>>> {
        let bg_ref = self.data.upgrade();
        if let Option::Some(bg) = bg_ref {
            Option::Some(Rc::clone(&bg))
        } else {
            Option::None
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct BackgroundData {
    pub name: String,
    pub file_path: Option<Box<Path>>,
}

#[cfg(test)]
mod background_test {
    use super::{BackgroundData, Background, BackgroundRef};

    #[test]
    fn background_update() {
        let mut a = Background::new();

        let a_ref_1 = BackgroundRef::from(&a);
        let a_ref_2 = BackgroundRef::from(&a);

        let mut x = a.get_mut_data();

        //std::rc::Rc::get_mut(&x).int = 7;

        println!("{:#?}", x);
        println!("{:#?}", x);
    }
}