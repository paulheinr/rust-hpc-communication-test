use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
// ======================
// Events (open set)
// ======================

pub trait Event: Debug + Any {
    fn time(&self) -> u32;
}

// Your events can be anywhere (including external crates)
#[derive(Debug)]
pub struct SimpleEvent {
    pub time: u32,
    pub type_: &'static str, // keep if you like, not required for routing
}
impl Event for SimpleEvent {
    fn time(&self) -> u32 {
        self.time
    }
}

#[derive(Debug)]
pub struct ComplexEvent {
    pub time: u32,
    pub type_: &'static str,
    pub data: String,
}
impl Event for ComplexEvent {
    fn time(&self) -> u32 {
        self.time
    }
}

// ======================
//
// Subscribers
//
// ======================

pub trait EventsSubscriber {
    fn receive_event(&mut self, event: &dyn Event);
}

// A generic, type-erased router that lets you register per-type handlers.
// External users can build one and only register what they care about.
pub struct RoutingSubscriber {
    // one or more handlers per concrete event type
    handlers: HashMap<TypeId, Vec<Box<dyn FnMut(&dyn Any)>>>,
    // optional fallback for unknown events
    fallback: Option<Box<dyn FnMut(&dyn Event)>>,
}

impl RoutingSubscriber {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            fallback: None,
        }
    }

    // Register a handler for a specific event type.
    pub fn on<E: Event + 'static>(mut self, mut f: impl FnMut(&E) + 'static) -> Self {
        self.handlers
            .entry(TypeId::of::<E>())
            .or_default()
            .push(Box::new(move |any: &dyn Any| {
                if let Some(e) = any.downcast_ref::<E>() {
                    f(e);
                }
            }));
        self
    }

    // Optional fallback for events without a registered handler.
    pub fn on_any(mut self, mut f: impl FnMut(&dyn Event) + 'static) -> Self {
        self.fallback = Some(Box::new(move |e| f(e)));
        self
    }
}

impl EventsSubscriber for RoutingSubscriber {
    fn receive_event(&mut self, event: &dyn Event) {
        let any: &dyn Any = event as &dyn Any;

        if let Some(list) = self.handlers.get_mut(&any.type_id()) {
            for h in list.iter_mut() {
                (h)(any);
            }
        } else if let Some(fallback) = self.fallback.as_mut() {
            (fallback)(event);
        }
    }
}

// A simple example subscriber using the router under the hood
pub struct PrintSubscriber(RoutingSubscriber);

impl PrintSubscriber {
    pub fn new() -> Self {
        // Only declares interest in SimpleEvent and ComplexEvent;
        // more can be added later without touching core traits.
        let router = RoutingSubscriber::new()
            .on::<SimpleEvent>(|e| {
                println!("(Print) SimpleEvent @{} type={}", e.time(), e.type_);
            })
            .on::<ComplexEvent>(|e| {
                println!("(Print) ComplexEvent @{} data={}", e.time(), e.data);
            })
            .on_any(|e| println!("(Print) Unhandled event: {:?}", e));
        Self(router)
    }
}

impl EventsSubscriber for PrintSubscriber {
    fn receive_event(&mut self, event: &dyn Event) {
        self.0.receive_event(event)
    }
}

// ======================
// Publisher (unchanged public shape)
// ======================

pub struct EventsPublisher {
    handlers: Vec<Box<dyn EventsSubscriber>>,
}

impl EventsPublisher {
    pub fn new() -> Self {
        EventsPublisher {
            handlers: Vec::new(),
        }
    }
    pub fn add_subscriber(&mut self, handler: Box<dyn EventsSubscriber>) {
        self.handlers.push(handler);
    }
    pub fn publish_event(&mut self, event: &dyn Event) {
        for handler in self.handlers.iter_mut() {
            handler.receive_event(event);
        }
    }
}

// ======================
// Demo
// ======================

fn main() {
    let mut publisher = EventsPublisher::new();
    publisher.add_subscriber(Box::new(PrintSubscriber::new()));

    let se = SimpleEvent {
        time: 42,
        type_: "TestEvent",
    };
    publisher.publish_event(&se);

    let ce = ComplexEvent {
        time: 43,
        type_: "TestEvent",
        data: "Hello world!".into(),
    };
    publisher.publish_event(&ce);

    // ---- External crate example (pseudo) ----
    // #[derive(Debug)]
    // pub struct ExternalEvent { pub time: u32, pub payload: u64 }
    // impl Event for ExternalEvent { fn time(&self) -> u32 { self.time } }
    //
    // let mut custom = RoutingSubscriber::new()
    //     .on::<ExternalEvent>(|e| println!("external payload={}", e.payload));
    // publisher.add_subscriber(Box::new(custom));
    // publisher.publish_event(&ExternalEvent { time: 100, payload: 7 });
}
