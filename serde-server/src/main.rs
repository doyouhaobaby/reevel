#[macro_use]
extern crate serde_derive;
use std::net::{TcpListener, TcpStream};
use std::io::{stdin, BufRead, BufReader, Error, Write};
use std::{env, str, thread};

#[derive(Serialize, Deserialize, Debug)]
struct Point3d {
    x: u32,
    y: u32,
    z: u32,
}

fn main() {
    dbg! (Point3d {x: 5, y: 6, z: 55});
    
}
