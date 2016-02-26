use link::header::*;
use std::cmp;
use link::crc::calc_crc;

#[derive(PartialEq, Debug)]
pub enum ParseError {
    BadLength(u8),
    BadHeaderCRC,
}

pub trait ParseHandler {

    // called when a complete frame is received
    fn on_frame(&mut self, header: &Header, slice: &[u8]);

    // called when bad data is received
    fn on_error(&mut self, error: ParseError);
}

enum ParserState {
    WaitSync1,
    WaitSync2,
    WaitForHeader(usize),       // the number of bytes received
    WaitForBody(usize, usize)   // the number of bytes received and the number remaining
}

const HEADER_SIZE : usize = 10;
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

    fn read_u16(slice: &[u8]) -> u16 {
        ((slice[1] as u16) << 8) | (slice[0] as u16)
    }

    // decode all the bytes in the slice
    pub fn decode<T: ParseHandler>(&mut self, slice: &[u8], handler: &mut T) {

        let mut total_consumed : usize = 0;

        while  {
            let subslice = &slice[total_consumed ..];
            let num_consumed = self.decode_one(subslice, handler);
            total_consumed += num_consumed;
            num_consumed != 0
        } {}

    }

    // returns the number of bytes consumed from the slice. might mutate the state.
    fn decode_one<T: ParseHandler>(&mut self, slice: &[u8], handler: &mut T) -> usize {
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
                self.header[0] = SYNC1;
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
                self.header[1] = SYNC2;
                self.state = ParserState::WaitForHeader(2);
                1
            }
            _ =>  {
                self.state = ParserState::WaitSync1;
                1
            }
        }
    }

    fn decode_header<T: ParseHandler>(&mut self, slice: &[u8], received: usize, handler: &mut T) -> usize {

        // consume the minimum of the length of the slice or the remainder
        let remaining = HEADER_SIZE - received;
        let consumed = cmp::min(slice.len(), remaining);

        {
            // copy this amount into header buffer
            // TODO - clean this up in 1.7
            let src = &slice[0 .. consumed];
            let dest = &mut self.header[received ..];
            assert!(src.len() == dest.len());
            for i in 0..consumed {
                dest[i] = src[i];
            }
        }

        let new_received = received + consumed;

        if new_received < HEADER_SIZE {
            self.state = ParserState::WaitForHeader(new_received);
            return consumed;
        }

        // we have a full header in the buffer. Time to analyze and state transition
        let len = self.header[2];

        if len < 5 {
                handler.on_error(ParseError::BadLength(len));
                self.state = ParserState::WaitSync1;
                return consumed;
        };

        let ctrl = self.header[3];
        let dest = Parser::read_u16(&self.header[4..6]);
        let src = Parser::read_u16(&self.header[6..8]);
        let crc = Parser::read_u16(&self.header[8..10]);

        let expected_crc = calc_crc(&self.header[0 .. 8]);

        if crc != expected_crc {
            handler.on_error(ParseError::BadHeaderCRC);
            self.state = ParserState::WaitSync1;
            return consumed;
        }

        let header = Header {
            ctrl: Ctrl::from(ctrl),
            dest: dest,
            src: src,
        };

        handler.on_frame(&header, &self.body[0..0]);

        self.state = ParserState::WaitSync1;

        consumed
    }

    fn decode_body<T: ParseHandler>(&mut self, slice: &[u8], handler: &mut T) -> usize {
        0
    }
}

enum Expect {
    Error(ParseError),
    Frame(Header, usize)
}

struct MockHandler {
    expects: Vec<Expect>
}

impl MockHandler {
    pub fn new() -> MockHandler {
        MockHandler {
            expects: Vec::new()
        }
    }
}

impl ParseHandler for MockHandler {

    fn on_frame(&mut self, header: &Header, slice: &[u8]) {
        match self.expects.pop() {
            Some(Expect::Frame(hdr, size)) => assert_eq!(hdr, *header),
            _ => assert!(false, "frame was not expected")
        }
    }

    fn on_error(&mut self, error: ParseError) {
        match self.expects.pop() {
            Some(Expect::Error(err)) => assert_eq!(err, error),
            _ => assert!(false, "error was not expected"),
        }
    }
}

#[test]
fn header_parse_catches_bad_length() {
    // CRC is the 0x21E9 at the end (little endian)
    let frame: [u8; 10] = [0x05, 0x64, 0x04, 0xC0, 0x01, 0x00, 0x00, 0x04, 0xE9, 0x21];

    let mut parser = Parser::new();
    let mut handler = MockHandler::new();

    handler.expects.push(Expect::Error(ParseError::BadLength(4)));

    parser.decode(&frame[..], &mut handler);

    assert!(handler.expects.is_empty());
}

#[test]
fn header_parse_catches_bad_crc() {
    // CRC is the 0x21E9 at the end (little endian)
    let frame: [u8; 10] = [0x05, 0x64, 0x05, 0xC0, 0x01, 0x00, 0x00, 0x04, 0xE9, 0x20];

    let mut parser = Parser::new();
    let mut handler = MockHandler::new();

    handler.expects.push(Expect::Error(ParseError::BadHeaderCRC));

    parser.decode(&frame[..], &mut handler);

    assert!(handler.expects.is_empty());
}

#[test]
fn returns_frame_for_length_of_five() {
    // CRC is the 0x21E9 at the end (little endian)
    let frame: [u8; 10] = [0x05, 0x64, 0x05, 0xC0, 0x01, 0x00, 0x00, 0x04, 0xE9, 0x21];

    let mut parser = Parser::new();
    let mut handler = MockHandler::new();

    handler.expects.push(
        Expect::Frame(
            Header::from(Ctrl::from(0xC0), 1, 1024),
            0
        )
    );

    parser.decode(&frame[..], &mut handler);

    assert!(handler.expects.is_empty());
}
