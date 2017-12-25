use std::collections::HashMap;
use runtime::value::Value;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Capturer<'a> {
    captured_values: HashMap<String, Vec<Value<'a>>>,
}

impl<'a> Capturer<'a> {
    pub fn new() -> Self {
        Capturer {
            captured_values: HashMap::new(),
        }
    }

    pub fn capture(&'a mut self, target: &String, value: &Value<'a>, ends: bool) -> Option<Value<'a>> {
        // Insert into vector.
        if self.captured_values.contains_key(target) {
            self.captured_values.get_mut(target).as_mut().unwrap().push(value.clone());
        } else {
            self.captured_values.insert(target.clone(), vec![value.clone()]);
        }

        // Stop capturing on stream end,
        if ends {
            let cloned = target.clone();
            Some(Value::Stream(Rc::new(RefCell::new(move | mut emit: Box<FnMut(Value<'a>, bool)> | {
                for captured_value in self.captured_values.get(&cloned).unwrap() {
                    emit(captured_value.clone(), false);
                }
            }))))
        } else {
            None
        }
    }
}
