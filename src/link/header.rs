use std::fmt;
use link::Function;

pub struct Header {
    pub func:      Function,
    pub master:    bool,
	pub fcb:       bool,
	pub fcvdfc:    bool,
	pub dest:      u16,
	pub src:       u16,
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "func: {} master: {} fcb: {} fcv: {} dest: {} src: {}", self.func, self.master, self.fcb, self.fcvdfc, self.dest, self.src)
    }
}
