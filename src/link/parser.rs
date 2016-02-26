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

const HEADER_SIZE : usize = 8;
const MAX_BODY_SIZE : usize = 282;

pub struct Parser {
    state: ParserState,
    header: [u8; HEADER_SIZE], // beginning after the 0x0564
    body: [u8; MAX_BODY_SIZE], // after the header including the CRCs
}

const SYNC1 : u8 = 0x05;
const SYNC2 : u8 = 0x64;

impl Parser {

    pub fn new () -> Parser {
        Parser {
            state: ParserState::WaitSync1,
            header: [0; HEADER_SIZE],
            body: [0; MAX_BODY_SIZE]
        }
    }

    // decode all the bytes in the slice
    pub fn decode(&mut self, slice: &[u8], handler: &mut ParseHandler) {

        let mut total_consumed : usize = 0;

        while  {
            let subslice = &slice[total_consumed ..];
            let num_consumed = self.decode_one(subslice, handler);
            total_consumed += num_consumed;
            num_consumed != 0
        } {}

        //while(num_consumed)
        //while self.decode_one(slice, handler) {}
    }

    // returns the number of bytes consumed from the slice. might mutate the state.
    fn decode_one(&mut self, slice: &[u8], handler: &mut ParseHandler) -> usize {
        match self.state {
            ParserState::WaitSync1 => self.decode_wait_sync1(slice),
            ParserState::WaitSync2 => self.decode_wait_sync2(slice),
            ParserState::WaitForHeader{received} => 0,
            ParserState::WaitForBody{ref header, received, length} => 0,
        }
    }

    // skip over values until you find SYNC1
    fn decode_wait_sync1(&mut self, slice: &[u8]) -> usize {
        for value in slice {
            if *value == SYNC1 {
                self.state = ParserState::WaitSync2;
                return 1;
            }
        }
        slice.len()
    }

    // skip over values until you find SYNC2
    fn decode_wait_sync2(&mut self, slice: &[u8]) -> usize {
        match slice.first() {
            None => 0,
            Some(&SYNC2) => {
                self.state = ParserState::WaitForHeader{received: 0};
                1
            }
            _ =>  {
                self.state = ParserState::WaitSync1;
                1
            }
        }
    }
}
