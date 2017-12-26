use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use runtime::value::Value;

pub type Link<'a> = Rc<RefCell<FnMut(Value<'a>, bool) + 'a>>;

#[derive(Clone)]
pub struct LinkWatcher<'a> {
    watchers: HashMap<String, HashMap<String, Link<'a>>>,
}

impl<'a> LinkWatcher<'a> {
    pub fn new() -> Self {
        LinkWatcher {
            watchers: HashMap::new(),
        }
    }

    pub fn add_watcher(&mut self, target: String, uuid: String, link: Link<'a>) {
        if self.watchers.contains_key(&target) {
            self.watchers.get_mut(&target).as_mut().unwrap().insert(uuid, link);
            return;
        }

        let mut newmap = HashMap::new();
        newmap.insert(uuid, link);
        self.watchers.insert(target, newmap);
    }

    pub fn resolve_links(&mut self, target: String, value: &Value<'a>, end: bool) {
        match self.watchers.get_mut(&target) {
            Some(map) => {
                debug!("Found {} watcher for {:} with value: {:?}", map.len(), target, value);
                for (_, watcher) in map {
                    (&mut *watcher.borrow_mut())(value.clone(), end);
                }
            },
            None => {
                debug!("Tried to resolve link to '{:}' but no one linked there", target);
            },
        }
    }
}
