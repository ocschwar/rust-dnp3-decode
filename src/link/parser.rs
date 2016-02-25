use link::header::Header;

pub enum ParseError {
    BadLength(u8),
    BadHeaderCRC { expected: u16, received: u16 },
}

pub trait ParseHandler {

    // called when a complete frame is received
    fn on_frame(&self, header: &Header, slice: &[u8]);

    // called when bad data is received
    fn on_error(&self, error: &ParseError);
}

enum ParserState {
    WaitSync1,
    WaitSync2,
    WaitForHeader{received : usize},
    WaitForBody{header: Header, received : usize, length: usize }
}

const SYNC1 : u8 = 0x05;
const SYNC2 : u8 = 0x64;
const HEADER_SIZE : usize = 8;
const MAX_BODY_SIZE : usize = 282;

pub struct Parser {
    state: ParserState,
    header: [u8; HEADER_SIZE], // beginning after the 0x0564
    body: [u8; MAX_BODY_SIZE], // after the header including the CRCs
}

impl Parser {

    pub fn new () -> Parser {
        Parser {
            state: ParserState::WaitSync1,
            header: [0; HEADER_SIZE],
            body: [0; MAX_BODY_SIZE]
        }
    }

    pub fn decode(&self, slice: &mut [u8], handler: &mut ParseHandler) {
        while self.decode_one(slice, handler) {}
    }

    fn decode_one(&self, slice: &mut [u8], handler: &mut ParseHandler) -> bool {
        match self.state {
            ParserState::WaitSync1 => self.decode_wait_sync1(slice),
            ParserState::WaitSync2 => false,
            ParserState::WaitForHeader{received} => false,
            ParserState::WaitForBody{ref header, received, length} => false,
        }
    }

    // skip over values until you find SYNC1
    fn decode_wait_sync1(&self, slice: &mut [u8]) -> bool {
        false
    }
}
