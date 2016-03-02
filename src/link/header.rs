use link::function::Function;

const MASK_DIR : u8 = 0x80;
const MASK_PRM : u8 = 0x40;
const MASK_FCB : u8 = 0x20;
const MASK_FCV : u8 = 0x10;
const MASK_FUNC : u8 = 0x0F;
const MASK_FUNC_OR_PRM : u8 = MASK_PRM | MASK_FUNC;

#[derive(Clone, PartialEq, Debug)]
pub struct Ctrl {
    pub func:      Function,
    pub master:    bool,
	pub fcb:       bool,
	pub fcvdfc:    bool,
}

impl Ctrl {
    pub fn from(byte: u8) -> Ctrl {
        Ctrl {
            func : Function::from(byte & MASK_FUNC_OR_PRM),
            master: (byte & MASK_DIR) != 0,
            fcb: (byte & MASK_FCB) != 0,
            fcvdfc: (byte & MASK_FCV) != 0
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Header {
    pub ctrl:      Ctrl,
	pub dest:      u16,
	pub src:       u16,
}

impl Header {
    pub fn from(ctrl: Ctrl, dest: u16, src: u16) -> Header {
        Header {
            ctrl: ctrl,
            dest: dest,
            src: src,
        }
    }

    pub fn default() -> Header {
        Header {
            ctrl: Ctrl::from(0),
            dest: 0,
            src: 0,
        }
    }
}
