extern crate dnp3decode;

use dnp3decode::link::header::LinkHeader;
use dnp3decode::link::header::LinkFunction;

fn main() {

    println!("Creating a header!");

    let header = LinkHeader {
        func: LinkFunction::Unknown(0xFF),
        master:    true,
    	fcb:       true,
    	fcvdfc:    true,
    	dest:      1024,
    	src:       1,
    };

    println!("The header is {}", header);
}
