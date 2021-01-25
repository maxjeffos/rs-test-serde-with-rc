#![allow(dead_code)]
use serde_derive::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug)]
struct Thing {
    position: Rc<Position>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Position {
    x: u8,
    y: u8,
}

fn main() {
    let mut things = Vec::<Thing>::new();

    let p1 = Rc::<Position>::new(Position {
        x: 3,
        y: 4,
    });
   
    let p_rc = Rc::clone(&p1); 
    
    let t = Thing {
        position: p1,
    };

    let t2 = Thing {
        position: p_rc,
    };
    println!("t: {:?}", t);
    println!("t: {:?}", t);

    things.push(t);
    things.push(t2);

    let json_str = serde_json::to_string_pretty(&things).unwrap();
    println!("{}", json_str);
}

