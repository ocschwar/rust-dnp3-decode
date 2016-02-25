use std::fmt;

pub enum Function
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

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Function::PriResetLinkStates => write!(f, "PriResetLinkStates"),
            &Function::PriTestLinkStates => write!(f, "PriTestLinkStates"),
            &Function::PriConfirmedUserData => write!(f, "PriConfirmedUserData"),
            &Function::PriUnconfirmedUserData => write!(f, "PriUnconfirmedUserData"),
            &Function::PriRequestLinkStates => write!(f, "PriRequestLinkStates"),
            &Function::SecAck => write!(f, "SecAck"),
            &Function::SecNack => write!(f, "SecNack"),
            &Function::SecLinkStatus => write!(f, "SecLinkStatus"),
            &Function::SecNotSupported => write!(f, "SecNotSupported"),
            &Function::Unknown(x) => write!(f, "Unknown({})", x),
        }
    }
}
