use std::fmt;

pub enum LinkFunction
{
  PriResetLinkStates,
  PriTestLinkStates,
  PriConfirmedUserData,
  PriUnconfirmedUserData,
  PriRequestLinkStates,
  SecAck,
  SecNack,
  SecLinkStatus,
  SecNotSupported,
  Unknown(u8),
}

impl fmt::Display for LinkFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &LinkFunction::PriResetLinkStates => write!(f, "PriResetLinkStates"),
            &LinkFunction::PriTestLinkStates => write!(f, "PriTestLinkStates"),
            &LinkFunction::PriConfirmedUserData => write!(f, "PriConfirmedUserData"),
            &LinkFunction::PriUnconfirmedUserData => write!(f, "PriUnconfirmedUserData"),
            &LinkFunction::PriRequestLinkStates => write!(f, "PriRequestLinkStates"),
            &LinkFunction::SecAck => write!(f, "SecAck"),
            &LinkFunction::SecNack => write!(f, "SecNack"),
            &LinkFunction::SecLinkStatus => write!(f, "SecLinkStatus"),
            &LinkFunction::SecNotSupported => write!(f, "SecNotSupported"),
            &LinkFunction::Unknown(x) => write!(f, "Unknown({})", x),
        }
    }
}

pub struct LinkHeader {
    pub func:      LinkFunction,
    pub master:    bool,
	pub fcb:       bool,
	pub fcvdfc:    bool,
	pub dest:      u16,
	pub src:       u16,
}

impl fmt::Display for LinkHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "func: {} master: {} fcb: {} fcv: {} dest: {} src: {}", self.func, self.master, self.fcb, self.fcvdfc, self.dest, self.src)
    }
}
