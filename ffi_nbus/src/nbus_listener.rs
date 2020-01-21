#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate pretty_env_logger;
use std::ffi::{CStr, CString};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread;

static mut nbus_client:NbusClient = NbusClient {
    client : core::mem::MaybeUninit::<nbus_t>::uninit()
};

pub struct NbusClient {
    pub client: core::mem::MaybeUninit::<nbus_t> 
}

unsafe impl Send for NbusClient {}

impl NbusClient {

    pub unsafe fn init(){
        nbus_init(nbus_client.client.as_mut_ptr());
    }

    pub unsafe fn nbus_event_send(event:String, message: String) {
        debug!("Preparing structures");
        let event = CString::new(event).unwrap();
        let message = CString::new(message).unwrap();
        debug!("calling nbus; {:?}", *nbus_client.client.as_mut_ptr());
        let nbus_response = nbus_event_send(nbus_client.client.as_mut_ptr(), event.as_ptr() as *mut i8, message.as_ptr() as *mut i8);
        if nbus_response != 0{
            error!("Error {} while sending command to NBUS", nbus_response);
        }
        info!("Message sent");
    }

    pub unsafe fn start_listener(callback: unsafe extern "C" fn(_nbus : *mut nbus_s, event: *const i8, json_message: *const i8)){
        thread::spawn(move || {
            let mut events = vec!(CString::new("opa.*").unwrap().into_raw());
            info!("Listener started");
            let nbus_response = nbus_event_listen(nbus_client.client.as_mut_ptr(), events.as_mut_ptr(), events.len() as i32, Some(callback));
            if nbus_response != 0{
                error!("Error {} while connecting to NBUS", nbus_response);
            }
            loop{thread::sleep(Duration::from_secs(1));}
        });

    }
}

