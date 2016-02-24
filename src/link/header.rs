

enum LinkFunction
{
  PriResetLinkStates,
  PriTestLinkStates,
  PriConfirmedUserData,
  PriUnconfirmedUser,
  PriRequestLinkStates,
  SecAck,
  SecNack,
  SecLinkStatus,
  SecNotSupported,
  Unknown(u8),
}

struct Header {
    func:      LinkFunction,
    master:    bool,
	fcb:       bool,
	fcvdfc:    bool,
	dest:      u16,
	src:       u16,
}
