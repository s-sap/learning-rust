// use std::io::{ self, Read };

use crossterm::terminal::{ enable_raw_mode,disable_raw_mode };
use crossterm::event::{ read, Event::Key, KeyCode::Char };


pub struct Editor {
    
}


impl Editor {

    pub fn default() -> Self {
	Editor{}
    }

    pub fn run(&self) {

	enable_raw_mode().unwrap();

	loop {

	    match read() {

		Ok(Key(event)) => {
		    println!("{:?} \r", event);

		    match event.code {
			Char(c) => {
			    if c == 'q' {
				break;
			    }
			},

			_ => (), // if not match DO nothing
		    }

		},
		Err(err) => println!("Error, {}", err),		
		_ => ()
	    }	    
	}

	disable_raw_mode().unwrap();
	
    }    
    
}



// for b in io::stdin().bytes() {

//     match b {

// 	Ok(b) => {
// 	    let c = b as char;	

// 	    if c.is_control() {
// 		println!("Binary: {0:08b} ASCII: {0:#03} \r", b);
// 	    } else {
// 		println!("Binary: {0:08b} \
// 			  ASCII: {0:#03} Character: {1:#?} \r", b,c);
// 	    }	

// 	    if c == 'q' {
// 		disable_raw_mode().unwrap();
// 		break;
// 	    }		    
// 	}

// 	Err(err) => println!("Error, {}", err),
//     }

//     disable_raw_mode().unwrap();

// }

