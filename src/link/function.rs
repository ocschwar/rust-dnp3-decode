
#[derive(Clone, PartialEq, Debug)]
pub enum Function
{
  PriResetLinkStates,
  PriTestLinkStates,
  PriConfirmedUserData,
  PriUnconfirmedUserData,
  PriRequestLinkStatus,
  SecAck,
  SecNack,
  SecLinkStatus,
  SecNotSupported,
  Unknown(u8),
}

const PRI_RESET_LINK_STATES : u8 = 0x40;
const PRI_TEST_LINK_STATES : u8 = 0x42;
const PRI_CONFIRMED_USER_DATA : u8  = 0x43;
const PRI_UNCONFIRMED_USER_DATA : u8  = 0x44;
const PRI_REQUEST_LINK_STATUS : u8  = 0x49;
const SEC_ACK : u8  = 0x00;
const SEC_NACK : u8  = 0x01;
const SEC_LINK_STATUS : u8  = 0x0B;
const SEC_NOT_SUPPORTED : u8  = 0x0F;

impl Function {

    pub fn from(byte: u8) -> Function {
        match byte {
            PRI_RESET_LINK_STATES => Function::PriResetLinkStates,
            PRI_TEST_LINK_STATES => Function::PriTestLinkStates,
            PRI_CONFIRMED_USER_DATA => Function::PriConfirmedUserData,
            PRI_UNCONFIRMED_USER_DATA => Function::PriUnconfirmedUserData,
            PRI_REQUEST_LINK_STATUS => Function::PriRequestLinkStatus,
            SEC_ACK => Function::SecAck,
            SEC_NACK => Function::SecNack,
            SEC_LINK_STATUS => Function::SecLinkStatus,
            SEC_NOT_SUPPORTED => Function::SecNotSupported,
            _ => Function::Unknown(byte),
        }
    }

}
