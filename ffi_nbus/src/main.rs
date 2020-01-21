#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod nbus_listener;

extern crate pretty_env_logger;
#[macro_use] extern crate log;
use std::ffi::{CStr, CString};

unsafe extern "C" fn call_cb(_nbus : *mut nbus_listener::nbus_s, event: *const i8, json_message: *const i8) -> () {
    let event_type = CStr::from_ptr(event).to_str().unwrap_or_default();
    let json_message = CStr::from_ptr(json_message).to_str().unwrap_or_default();
    info!("Event {} received: {}", event_type, json_message);
}

fn main() {

    pretty_env_logger::init();

    unsafe {
        debug!("Initializing nbus");
        nbus_listener::NbusClient::init();
        debug!("Sending message");
        nbus_listener::NbusClient::start_listener(call_cb);
        debug!("Sending message");
        nbus_listener::NbusClient::nbus_event_send(String::from("opa.hal.name"), String::from("{\"hello\" : \"hello\" }"));
    }
 
    info!("Hello, world!");
    loop{}
}
