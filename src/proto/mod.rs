use crate::proto::events::{Event, LoginEvent, LogoutEvent};
use prost::Name;

pub mod events {
    include!(concat!(env!("OUT_DIR"), "/events.rs"));
}

impl Name for LoginEvent {
    const NAME: &'static str = "LoginEvent";
    const PACKAGE: &'static str = "events";
}

impl Name for LogoutEvent {
    const NAME: &'static str = "LogoutEvent";
    const PACKAGE: &'static str = "events";
}

impl Name for Event {
    const NAME: &'static str = "Event";
    const PACKAGE: &'static str = "events";
}
