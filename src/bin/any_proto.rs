use prost::Message;
use prost_types::Any;
use rust_hpc_communication_test::proto::events::{Event, LoginEvent, LogoutEvent};

fn main() {
    let event = create_login_event("test", 1);
    handle_event(event);
}

fn create_login_event(user_id: &str, timestamp: i64) -> Event {
    let login = LoginEvent {
        user_id: user_id.to_string(),
        timestamp,
    };

    let mut buf = Vec::new();
    login.encode(&mut buf).unwrap();

    let result = Any::from_msg(&LoginEvent {
        user_id: user_id.to_string(),
        timestamp,
    }).unwrap();

    Event {
        payload: Some(result),
    }
}

fn handle_event(event: Event) {
    match event.payload.as_ref().unwrap().type_url.as_str() {
        "/events.LoginEvent" => {
                let login = event.payload.unwrap().to_msg::<LoginEvent>().unwrap();
                println!("Login: {} at {}", login.user_id, login.timestamp);
            }
        "/events.LogoutEvent" => {
                let logout = event.payload.unwrap().to_msg::<LogoutEvent>().unwrap();
        println!("Logout: {} at {}", logout.user_id, logout.timestamp);
            }
        _ => {
            println!("Unknown event type: {}", event.payload.as_ref().unwrap().type_url);
        }
    }
}