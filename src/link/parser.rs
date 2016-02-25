use link::header::Header;

pub trait FrameHandler {
    fn on_frame(&self, header: &Header, slice: &[u8]);
}
/*
enum ParserState {
    WaitSync05,
    WaitSync64,
    WaitForHeader{remaining : usize},
    WaitForBody{header: Header, remaining : usize}
}

const HEADER_SIZE : usize = 8;
const MAX_BODY_SIZE : usize = 282;

pub struct Parser {
    state: ParserState,
    header: [u8; HEADER_SIZE], // beginning after the 0x0564
    body: [u8; MAX_BODY_SIZE], // after the header including the CRCs
}

impl Parser {

    fn new () -> Parser {
        Parser {
            state: ParserState::WaitSync05,
            header: [0; HEADER_SIZE],
            body: [0; MAX_BODY_SIZE]
        }
    }

    fn decode(&self, slice: &[u8], handler: &mut FrameHandler) {

    }
}
*/
