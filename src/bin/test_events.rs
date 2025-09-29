use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Event: Any + Debug {
    fn as_any(&self) -> &dyn Any;
}

type OnEventFn = dyn Fn(&dyn Event) + 'static;

pub struct EventsManager {
    per_type: HashMap<TypeId, Vec<Rc<OnEventFn>>>,
    catch_all: Vec<Box<OnEventFn>>,
}

impl EventsManager {
    pub fn new(
        per_type: HashMap<TypeId, Vec<Rc<OnEventFn>>>,
        catch_all: Vec<Box<OnEventFn>>,
    ) -> Self {
        Self {
            per_type,
            catch_all,
        }
    }

    pub fn on<E, F>(&mut self, f: F)
    where
        E: Event,
        F: Fn(&dyn Event) + 'static,
    {
        let type_id = TypeId::of::<E>();
        let entry = self.per_type.entry(type_id).or_default();
        entry.push(Rc::new(move |ev: &dyn Event| {
            if let Some(e) = ev.as_any().downcast_ref::<E>() {
                f(e);
            }
        }));
    }

    pub fn on_any<F>(&mut self, f: F)
    where
        F: Fn(&dyn Event) + 'static,
    {
        self.catch_all.push(Box::new(f));
    }

    pub fn emit(&self, event: &dyn Event) {
        let tid = event.as_any().type_id();
        if let Some(list) = self.per_type.get(&tid).cloned() {
            for h in list {
                h(event);
            }
        }
        for h in &self.catch_all {
            h(event);
        }
    }
}

#[derive(Debug)]
struct SimpleEvent {
    msg: String,
}

impl Event for SimpleEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
struct AnotherEvent {
    code: i32,
}

impl Event for AnotherEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    let mut manager = EventsManager::new(HashMap::new(), Vec::new());
    let c = Rc::new(RefCell::new(0));
    let c2 = c.clone();

    manager.on::<SimpleEvent, _>(move |ev| {
        println!("SimpleEvent received: {:?}", ev);
        *c.borrow_mut() += 1;
        println!("c = {}", c.borrow().to_string());
    });
    manager.on::<AnotherEvent, _>(move |ev| {
        println!("AnotherEvent received: {:?}", ev);
        *c2.borrow_mut() += 1;
        println!("c = {}", c2.borrow().to_string());
    });

    let ff = |ev: &dyn Event| {};

    manager.on_any(ff);

    let event = SimpleEvent {
        msg: "Hello".to_string(),
    };
    manager.emit(&event);
    manager.emit(&event);
    manager.emit(&event);
    manager.emit(&event);
    manager.emit(&event);
}
