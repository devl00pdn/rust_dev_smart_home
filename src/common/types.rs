use std::cell::RefCell;
use std::rc::Rc;

pub type SmartPointer<T> = Rc<RefCell<T>>;
