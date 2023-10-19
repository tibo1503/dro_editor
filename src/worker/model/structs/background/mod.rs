use std::cell::{RefMut, RefCell};
use std::rc::{Rc, Weak};

#[derive(Debug, Clone)]
pub struct Background {
    data: Rc<Rc<BackgroundData>>
}

impl Background {
    pub fn new() -> Self {
        Background {
            data: Rc::new(Rc::new(
                BackgroundData {
                    ..BackgroundData::default()
                }
            ))
        }
    }

    pub fn get_mut_data(&mut self) -> Option<Rc<BackgroundData>> {
        Option::Some(Rc::clone(&*self.data))
    }
}

pub struct BackgroundRef {
    data: Weak<Rc<BackgroundData>>
}

impl From<&Background> for BackgroundRef {
    fn from(bg: &Background) -> BackgroundRef {
        BackgroundRef { 
            data: Rc::downgrade(&bg.data) 
        }
    }
}

#[derive(Debug, Clone, Default)]
struct BackgroundData {
    int: i32    
}

#[cfg(test)]
mod background_test {
    use super::{BackgroundData, Background, BackgroundRef};

    #[test]
    fn background_update() {
        let mut a = Background::new();

        let a_ref_1 = BackgroundRef::from(&a);
        let a_ref_2 = BackgroundRef::from(&a);

        let mut x = a.get_mut_data().unwrap();

        //std::rc::Rc::get_mut(&x).int = 7;

        println!("{:#?}", x);
        println!("{:#?}", x);
    }
}