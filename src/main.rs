extern crate dnp3;

use dnp3::link::Header;
use dnp3::link::Function;

fn main() {

    let header = Header {
        func: Function::Unknown(0xFF),
        master:    true,
    	fcb:       true,
    	fcvdfc:    true,
    	dest:      1024,
    	src:       1,
    };

    println!("The header is {}", header);
}
