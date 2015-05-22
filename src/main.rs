#![feature(collections)]
#![feature(io)]

use std::net::{TcpStream};
use std::io::{BufRead, BufReader, Write, stdout};

mod parser;

fn main() {

	let name = "irc.freenode.net:6667";	
	let mut con = TcpStream::connect(name).ok().unwrap();
	let mut rbuf = BufReader::new(con.try_clone().ok().unwrap());
	let mut wbuf = con.broadcast(stdout());
	write!(&mut wbuf, "NICK tbot_3\r\n").ok().unwrap();
	write!(&mut wbuf, "USER tbot_3 0 * :tbot - new and improved\r\n").ok().unwrap();
	write!(&mut wbuf, "JOIN #openredstone\r\n").ok().unwrap();
	for line in rbuf.lines() {
        let mut lp = parser::Line::new();
        let linestr = line.ok().unwrap();
        lp.parse(linestr);
        println!("{:?} {:?} {:?}", lp.prefix(), lp.command(), lp.params());
	}
}
