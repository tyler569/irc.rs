#![feature(io)]

use std::net::{TcpStream};
use std::io::{BufRead, BufReader, Write, stdout};

mod parser;
use parser::Line;

fn main() {

    let name = "irc.freenode.net:6667"; 
    let con = TcpStream::connect(name).ok().unwrap();
    let rbuf = BufReader::new(con.try_clone().ok().unwrap());
    let mut wbuf = con.broadcast(stdout());
    write!(&mut wbuf, "NICK tbot_3\r\n").ok().unwrap();
    write!(&mut wbuf, "USER tbot_3 0 * :tbot - new and improved\r\n").ok().unwrap();
    write!(&mut wbuf, "JOIN #openredstone\r\n").ok().unwrap();
    for line in rbuf.lines() {
        let linestr = line.ok().unwrap();
        let l = linestr.trim_right_matches('\r');
        let lp = Line::parse(l).ok().unwrap();
        println!("> {}\n> {:?}", lp, lp);
        match *lp.command() {
            Some("PING") => {
                write!(&mut wbuf, "PONG :{}\n", lp.params().last().unwrap())
                    .ok().unwrap();
            },
            Some(_) => {},
            None => { panic!("No command?") },
        }
    }
}
