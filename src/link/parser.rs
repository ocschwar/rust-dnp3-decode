use link::header::*;
use std::cmp;
use link::crc::calc_crc;

#[derive(PartialEq, Debug)]
pub enum ParseError {
    BadLength(u8),
    BadHeaderCRC,
    BadBodyCRC
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
    WaitForHeader(usize),           // the number of bytes received
    WaitForBody(usize, usize, u8)   // the number of bytes received, the number remaining, and the size of the payload user bytes
}

const HEADER_SIZE : usize = 10;
const MAX_BODY_SIZE : usize = 282;
const MAX_USER_DATA : usize = 250;

pub struct Parser {
    state: ParserState,
    header: Header,             // the last header value read
    header_arr: [u8; HEADER_SIZE],  // beginning after the 0x0564
    body_arr: [u8; MAX_BODY_SIZE],  // after the header including the CRCs
}

const SYNC1 : u8 = 0x05;
const SYNC2 : u8 = 0x64;
const DATA_BLOCK_SIZE: usize = 16;
const CRC_BYTES_PER_BLOCK : usize = 2;
const FULL_BLOCK_SIZE: usize = DATA_BLOCK_SIZE + CRC_BYTES_PER_BLOCK;

impl Parser {

    pub fn new () -> Parser {
        Parser {
            state: ParserState::WaitSync1,
            header: Header::default(),
            header_arr: [0; HEADER_SIZE],
            body_arr: [0; MAX_BODY_SIZE]
        }
    }

    fn read_u16(slice: &[u8]) -> u16 {
        ((slice[1] as u16) << 8) | (slice[0] as u16)
    }

    fn calc_body_size(num_payload_bytes: u8) -> usize {
        let num_full_blocks = (num_payload_bytes as usize) / DATA_BLOCK_SIZE;
        let remainder = (num_payload_bytes as usize) % DATA_BLOCK_SIZE;
        if remainder == 0 {
            num_full_blocks*FULL_BLOCK_SIZE
        }
        else {
            (num_full_blocks*FULL_BLOCK_SIZE) + remainder + CRC_BYTES_PER_BLOCK
        }
    }

    fn verify_body_crc(body: &[u8]) -> bool {

        let mut pos = 0;
        let mut remainder : usize = body.len();

        while  remainder > 0  {

            let block_size = cmp::min(DATA_BLOCK_SIZE, remainder - CRC_BYTES_PER_BLOCK);
            let expected_crc = calc_crc(&body[pos..pos+block_size]);
            let actual_crc = Parser::read_u16(&body[pos+block_size..]);

            if expected_crc != actual_crc {
                return false;
            }

            let consumed = block_size + CRC_BYTES_PER_BLOCK;

            pos += consumed;
            remainder -= consumed;

        }

        true
    }

    fn extract_user_data(body: &[u8], dest: &mut [u8]) -> usize {

        let mut write_pos : usize = 0;
        let mut read_pos : usize = 0;
        let mut remainder : usize = body.len();

        while  remainder > 0  {

            let block_size = cmp::min(DATA_BLOCK_SIZE, remainder - CRC_BYTES_PER_BLOCK);

            Parser::mem_copy(&body[read_pos..read_pos+block_size], &mut dest[write_pos..write_pos+block_size]);

            write_pos += block_size;

            let consumed = block_size + CRC_BYTES_PER_BLOCK;

            read_pos += consumed;
            remainder -= consumed;
        }

        write_pos
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
            ParserState::WaitForBody(received, remaining, user_bytes) => self.decode_body(slice, received, remaining, user_bytes, handler),
        }
    }

    // skip over values until you find SYNC1
    fn wait_sync1(&mut self, slice: &[u8]) -> usize {
        for value in slice {
            if *value == SYNC1 {
                self.header_arr[0] = SYNC1;
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
                self.header_arr[1] = SYNC2;
                self.state = ParserState::WaitForHeader(2);
                1
            }
            _ =>  {
                self.state = ParserState::WaitSync1;
                1
            }
        }
    }

    // TODO - clean this up in 1.7 and use API method
    fn mem_copy(src: &[u8], dest: &mut [u8]) -> () {
        assert!(dest.len() >= src.len());
        for i in 0..src.len() {
            dest[i] = src[i];
        }
    }

    fn decode_header<T: ParseHandler>(&mut self, slice: &[u8], received: usize, handler: &mut T) -> usize {

        // consume the minimum of the length of the slice or the remainder
        let remaining = HEADER_SIZE - received;
        let consumed = cmp::min(slice.len(), remaining);

        Parser::mem_copy(&slice[0 .. consumed], &mut self.header_arr[received ..]);

        let new_received = received + consumed;

        if new_received < HEADER_SIZE {
            self.state = ParserState::WaitForHeader(new_received);
            return consumed;
        }

        // we have a full header in the buffer. Time to analyze and state transition
        let len = self.header_arr[2];

        if len < 5 {
                handler.on_error(ParseError::BadLength(len));
                self.state = ParserState::WaitSync1;
                return consumed;
        };

        let ctrl = self.header_arr[3];
        let dest = Parser::read_u16(&self.header_arr[4..6]);
        let src = Parser::read_u16(&self.header_arr[6..8]);
        let crc = Parser::read_u16(&self.header_arr[8..10]);

        let expected_crc = calc_crc(&self.header_arr[0 .. 8]);

        if crc != expected_crc {
            handler.on_error(ParseError::BadHeaderCRC);
            self.state = ParserState::WaitSync1;
            return consumed;
        }

        self.header = Header::from(Ctrl::from(ctrl), dest, src);

        let num_user_bytes = len - 5;

        if num_user_bytes == 0 {
            handler.on_frame(&self.header, &self.body_arr[0..0]);
            self.state = ParserState::WaitSync1;
        }
        else {
            self.state = ParserState::WaitForBody(
                0,
                Parser::calc_body_size(num_user_bytes),
                num_user_bytes
            );
        }

        consumed
    }

    fn decode_body<T: ParseHandler>(&mut self, slice: &[u8], received: usize, remaining: usize, num_user_bytes: u8, handler: &mut T) -> usize {

        let consumed = cmp::min(remaining, slice.len());

        // we copy this amount into the body buffer
        Parser::mem_copy(&slice[0..consumed], &mut self.body_arr[received..]);

        let remainder = remaining - consumed;
        let new_received = received + consumed;

        if remainder > 0 {
            self.state = ParserState::WaitForBody(received+consumed, remainder, num_user_bytes);
            return consumed;
        }

        // we have all the data, and now matter what happens we transition back to WaitSync1
        self.state = ParserState::WaitSync1;

        if !Parser::verify_body_crc(&self.body_arr[0..new_received]) {
            handler.on_error(ParseError::BadBodyCRC);
            return consumed;
        }

        // extract the user data and call a successful frame
        let mut user_data: [u8; MAX_USER_DATA] = [0; MAX_USER_DATA];

        let num_user_data = Parser::extract_user_data(&self.body_arr[0 .. new_received], &mut user_data[..]);

        handler.on_frame(&self.header, &user_data[0..num_user_data]);

        consumed
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
