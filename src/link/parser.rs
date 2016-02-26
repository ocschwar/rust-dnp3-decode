use link::header::Header;
use std::cmp;
use std::ptr;

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
    WaitForHeader(usize),       // the number of bytes received
    WaitForBody(usize, usize)   // the number of bytes received and the number remaining
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
            ParserState::WaitSync1 => self.wait_sync1(slice),
            ParserState::WaitSync2 => self.wait_sync2(slice),
            ParserState::WaitForHeader(received) => self.decode_header(slice, received, handler),
            ParserState::WaitForBody(received, remaining) => self.decode_body(slice, handler),
        }
    }

    // skip over values until you find SYNC1
    fn wait_sync1(&mut self, slice: &[u8]) -> usize {
        for value in slice {
            if *value == SYNC1 {
                self.state = ParserState::WaitSync2;
                return 1;
            }
        }
        slice.len()
    }

    // skip over values until you find SYNC2
    fn wait_sync2(&mut self, slice: &[u8]) -> usize {
        match slice.first() {
            None => 0,
            Some(&SYNC2) => {
                self.state = ParserState::WaitForHeader(0);
                1
            }
            _ =>  {
                self.state = ParserState::WaitSync1;
                1
            }
        }
    }

    fn decode_header(&mut self, slice: &[u8], received: usize, handler: &mut ParseHandler) -> usize {

        // consume the minimum of the length of the slice or the remainder
        let remaining = HEADER_SIZE - received;
        let consumed = cmp::min(slice.len(), remaining);

        // copy this amount into header buffer
        // TODO - clean this up in 1.7
        let src = &slice[0 .. consumed];
        let dest = &mut self.header[received ..];
        assert!(src.len() == dest.len());
        for i in 0..consumed {
            dest[i] = src[i];
        }

        let new_received = received + consumed;

        if new_received < HEADER_SIZE {
            self.state = ParserState::WaitForHeader(new_received);
        }
        else {
            // we have a full header. Time to analyze and state transition
            


            self.state = ParserState::WaitSync1;
        }

        consumed
    }

    fn decode_body(&mut self, slice: &[u8], handler: &mut ParseHandler) -> usize {
        0
    }
}
