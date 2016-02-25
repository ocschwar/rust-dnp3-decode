use link::header::Header;

pub trait FrameHandler {
    fn on_frame(&self, header: Header, slice: &[u8]);
}

enum ParserState {
    WaitSync,
    WaitForHeader{remaining : usize},
    WaitForBody
}

const HEADER_SIZE : usize = 10;
const MAX_BODY_SIZE : usize = 282;

pub struct Parser {
    state: ParserState,
    header: [u8; HEADER_SIZE],
    body: [u8; MAX_BODY_SIZE],
}

impl Parser {

    fn new () -> Parser {
        Parser {
            state: ParserState::WaitSync,
            header: [0; HEADER_SIZE],
            body: [0; MAX_BODY_SIZE]
        }
    }

    fn decode(&self, slice: &[u8], handler: &mut FrameHandler) {

    }
}
