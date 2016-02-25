use link::header::Header;

pub trait FrameHandler {
    fn on_frame(&self, header: Header);
}

enum ParserState {
    WaitSync,
    WaitForHeader,
    WaitForBody
}

pub struct Parser {
    state: ParserState
}

impl Parser {
    fn new () -> Parser {
        Parser {
            state: ParserState::WaitSync
        }
    }

    fn decode(handler: &mut FrameHandler) {

    }
}
