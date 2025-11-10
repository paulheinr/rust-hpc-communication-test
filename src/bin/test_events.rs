use derive_builder::Builder;
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

struct App {
    events: EventsManager,
}

impl App {
    pub fn run(&self) {
        let event = SimpleEvent {
            msg: "Hello".to_string(),
        };

        self.events.emit(&event);
        self.events.emit(&event);
        self.events.emit(&event);
        self.events.emit(&event);
        self.events.emit(&event);
    }
}

struct AppBuilder {
    events: Box<dyn FnOnce(&mut EventsManager) + Send>,
}

impl AppBuilder {
    fn build(self) -> App {
        let mut events = EventsManager::new(HashMap::new(), Vec::new());
        (self.events)(&mut events);
        App { events }
    }

    pub fn new(events: Box<dyn FnOnce(&mut EventsManager) + Send>) -> Self {
        Self { events }
    }
}

#[derive(Default)]
struct Handler {
    c: Rc<RefCell<i32>>,
}

impl Handler {
    fn on_simple(&self, e: &dyn Event) {
        if e.as_any().downcast_ref::<SimpleEvent>().is_some() {
            *self.c.borrow_mut() += 1;
            println!("SimpleEvent: {:?} (c = {})", e, *self.c.borrow());
        }
    }
    fn on_another(&self, e: &dyn Event) {
        if e.as_any().downcast_ref::<AnotherEvent>().is_some() {
            *self.c.borrow_mut() += 1;
            println!("AnotherEvent: {:?} (c = {})", e, *self.c.borrow());
        }
    }
}

fn main() {
    let events1: Box<dyn FnOnce(&mut EventsManager) + Send> = Box::new(move |events| {
        let c = Rc::new(RefCell::new(0));
        let c2 = c.clone();

        events.on::<SimpleEvent, _>(move |ev| {
            println!("SimpleEvent received: {:?}", ev);
            *c.borrow_mut() += 1;
            println!("c = {}", *c.borrow());
        });
        events.on::<AnotherEvent, _>(move |ev| {
            println!("AnotherEvent received: {:?}", ev);
            *c2.borrow_mut() += 1;
            println!("c = {}", *c2.borrow());
        });

        let h = Rc::new(Handler::default());

        let h1 = h.clone();
        events.on::<SimpleEvent, _>(move |e| h1.on_simple(e));

        let h2 = h.clone();
        events.on::<AnotherEvent, _>(move |e| h2.on_another(e));
    });

    let app_builder = AppBuilder::new(events1);

    let app = app_builder.build();
    app.run();
}
