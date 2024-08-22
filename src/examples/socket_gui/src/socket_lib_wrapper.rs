use std::ffi::{c_char, c_void, CString};

extern "C" {
    fn create_socket(addr: *const c_char) -> *mut c_void;
    fn destroy_socket(socket: *mut c_void);
    fn turn_on(socket: *mut c_void) -> bool;
    fn turn_off(socket: *mut c_void) -> bool;
    fn power_consumption_wt(socket: *mut c_void) -> f32;
    fn current_state(socket: *mut c_void) -> bool;
}

pub struct SocketLibWrapper {
    socket: *mut c_void,
}

impl SocketLibWrapper {
    pub fn new(addr: String) -> Result<SocketLibWrapper, String>
    {
        unsafe {
            let addr_sc = CString::new(addr).unwrap();
            let s = create_socket(addr_sc.as_ptr());
            if !s.is_null() {
                return Ok(Self { socket: s });
            }
        }
        Err("Socket has not crated".to_string())
    }

    pub fn turn_on(&self) -> bool {
        unsafe { turn_on(self.socket) }
    }

    pub fn turn_off(&self) -> bool {
        unsafe { turn_off(self.socket) }
    }

    pub fn power_consumption_wt(&self) -> f32 {
        unsafe { power_consumption_wt(self.socket) }
    }

    pub fn current_state(&self) -> bool {
        unsafe { current_state(self.socket) }
    }
}

impl Drop for SocketLibWrapper {
    fn drop(&mut self) {
        unsafe { destroy_socket(self.socket); }
    }
}
