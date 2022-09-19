use std::fmt;

pub mod server;

pub struct SmartSocket {
    on: bool,
    load: u32,
}

impl SmartSocket {
    pub fn new() -> Self {
        Self { on: false, load: 0 }
    }

    pub fn turn_on(&mut self) {
        self.on = true;
    }

    pub fn turn_off(&mut self) {
        self.on = false;
        self.load = 0;
    }

    pub fn set_load(&mut self, load: u32) {
        self.load = load;
    }
}

impl Default for SmartSocket {
    fn default() -> Self {
        SmartSocket::new()
    }
}

impl fmt::Display for SmartSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.on {
            write!(f, "[socket] state: on. load: {}", self.load)
        } else {
            write!(f, "[socket] state: off")
        }
    }
}
