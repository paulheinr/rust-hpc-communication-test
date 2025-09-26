use std::fmt::Debug;

pub trait EventTrait: Debug {
    fn type_(&self) -> &'static str;
    fn time(&self) -> u32;
}

#[derive(Debug)]
pub struct SimpleEvent {
    pub time: u32,
    pub type_: &'static str,
}

impl EventTrait for SimpleEvent {
    fn type_(&self) -> &'static str {
        self.type_
    }
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

impl EventTrait for ComplexEvent {
    fn type_(&self) -> &'static str {
        self.type_
    }
    fn time(&self) -> u32 {
        self.time
    }
}

// Minimal subscriber trait
pub trait EventsSubscriber {
    fn receive_event(&mut self, event: &dyn EventTrait);
}

pub struct PrintSubscriber;

impl EventsSubscriber for PrintSubscriber {
    fn receive_event(&mut self, event: &dyn EventTrait) {
        println!("Received event {:?}", event);
    }
}

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
    pub fn publish_event<E: EventTrait>(&mut self, event: &E) {
        for handler in self.handlers.iter_mut() {
            handler.receive_event(event);
        }
    }
}

fn main() {
    let mut publisher = EventsPublisher::new();
    publisher.add_subscriber(Box::new(PrintSubscriber));

    let event = SimpleEvent {
        time: 42,
        type_: "TestEvent",
    };
    publisher.publish_event(&event);

    let event = ComplexEvent {
        time: 42,
        type_: "TestEvent",
        data: "Hello world!".to_string(),
    };
    publisher.publish_event(&event);
}
