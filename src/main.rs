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
        let lp = Line::parse(linestr).ok().unwrap();
        println!("{:?} {:?} {:?}", lp.prefix(), lp.command(), lp.params());
        if lp.command() == "PING" {
            write!(&mut wbuf, "PONG :{}\n", lp.params().last().unwrap()).ok().unwrap();
        }
	}
}
