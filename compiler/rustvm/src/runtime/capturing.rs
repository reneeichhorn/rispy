use std::collections::HashMap;
use runtime::value::Value;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Capturer<'a> {
    captured_values: Rc<RefCell<HashMap<String, Vec<Value<'a>>>>>,
}

impl<'a> Capturer<'a> {
    pub fn new() -> Self {
        Capturer {
            captured_values: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn capture(&mut self, target: &String, value: &Value<'a>, ends: bool) -> Option<Value<'a>> {
        // Insert into vector.
        if self.captured_values.borrow_mut().contains_key(target) {
            self.captured_values.borrow_mut().get_mut(target).as_mut().unwrap().push(value.clone());
        } else {
            self.captured_values.borrow_mut().insert(target.clone(), vec![value.clone()]);
        }
        debug!("Captured a value for '{:}': {:#?}", target, value);

        // Stop capturing on stream end,
        if ends {
            let cloned = target.clone();
            let cloned_self = self.captured_values.borrow().clone();

            self.captured_values.borrow_mut().remove(target);
            Some(Value::Stream(Rc::new(RefCell::new(move | mut emit: Box<FnMut(Value<'a>, bool)> | {
                debug!("Captured values for '{:}': {:#?}", cloned, cloned_self);
                for (index, captured_value) in cloned_self.get(&cloned).unwrap().iter().enumerate() {
                    emit(captured_value.clone(), index >= (cloned_self.get(&cloned).unwrap().len() - 1));
                }
            }))))
        } else {
            None
        }
    }
}
