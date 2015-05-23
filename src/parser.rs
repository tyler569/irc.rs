
use std::fmt::{Debug, Display, Formatter, Error};

use self::Line::*;

pub enum Line<'a> {
    FromLine(&'a str),
    FromData(Option<&'a str>, &'a str, Vec<&'a str>),
}

impl<'a> Line<'a> {
    pub fn raw(&'a self) -> &'a str {
        match *self {
            FromLine(x) => x,
            FromData(prefix, cmd, ref params) => {
                //TODO: assemble line
                "TBI"
            }
        }
    }
    pub fn prefix(&'a self) -> Option<&'a str> {
        match *self {
            FromLine(x) => {
                if &x[0..1] == ":" {
                    let t: Vec<_> = x.splitn(2, ' ').collect();
                    Some(t[0].trim_left_matches(':'))
                } else {
                    None
                }
            },
            FromData(prefix, _, _) => prefix,
        }
    }
    pub fn command(&'a self) -> &'a str {
        match *self {
            FromLine(x) => {
                let cmd_pos = match self.prefix() {
                    Some(_) => 1,
                    None => 0,
                };
                let t: Vec<_> = x.splitn(3, ' ').collect();
                t[cmd_pos]
            },
            FromData(_, cmd, _) => cmd,
        }
    }               
    pub fn params(&'a self) -> Vec<&'a str> {
        match *self {
            FromLine(x) => {
                let params_pos = match self.prefix() {
                    Some(_) => 2,
                    None => 1,
                };
                let cmd_params: Vec<_> = x.splitn(params_pos + 1, ' ').collect();
                let params_trail = cmd_params[params_pos];
                let params_trail_vec: Vec<_> = params_trail.splitn(2, " :").collect();
                let mut params: Vec<_> = params_trail_vec[0].split(' ').collect();
                if params_trail_vec.len() > 1 {
                    params.push(params_trail_vec[1]);
                }
                params
            },
            FromData(_, _, ref params) => (*params).clone()
        }
    }
}

impl<'a> Display for Line<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.raw())
    }
}

impl<'a> Debug for Line<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?} {:?} {:?}", self.prefix(), self.command(), self.params())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

/*  No checking at the moment
    #[test]
    #[should_panic(expected = "Empty Line")]
    fn test_parse_empty() {
        let test = "".to_string();
        match Line::FromLine(test) {
            Ok(_) => { panic!("It worked, fail") },
            Err(_) => { panic!("Empty Line") },
        }
    }
*/

    #[test]
    fn test_parse_ping() {
        let line = Line::FromLine("PING :Hello World");
        assert_eq!(line.command(), "PING");
        assert_eq!(line.params(), &["Hello World"]);
    }

    #[test]
    fn test_compose_msg() {
        let line = Line::FromData(None, "PRIVMSG", vec!["#test", "Hello World"]);
        assert_eq!(line.raw(), "PRIVMSG #test :Hello World");
    }
}

