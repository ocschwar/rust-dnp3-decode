extern crate dnp3;

use dnp3::link::{Function, Header, Ctrl};

fn main() {

    let header = Header {
        ctrl : Ctrl {
            func: Function::Unknown(0xFF),
            master:    true,
    	    fcb:       true,
    	    fcvdfc:    true,
        },
    	dest:      1024,
    	src:       1,
    };

    println!("The header is {:?}", header);
}
