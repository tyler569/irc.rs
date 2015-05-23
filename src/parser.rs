
use std::fmt::{Debug, Display, Formatter, Error};

pub struct Line<'a> {
    raw:      &'a str,
    prefix:   Option<&'a str>,
    command:  Option<&'a str>,
    params:   Vec<&'a str>,
}

impl<'a> Line<'a> {
    pub fn parse(line: &'a str) -> Result<Line<'a>, &'static str> {
        let mut ret = Line { 
            raw: line,
            prefix: None,
            command: None,
            params: Vec::new()
        };
        {
            if ret.raw.len() == 0 {
                return Err("Empty Line");
            }

            let mut splc: Vec<_>;
            if &ret.raw[0..1] == ":" {
                let splp: Vec<_> = ret.raw.splitn(2, ' ').collect();
                ret.prefix = Some(splp[0].trim_left_matches(':'));
                splc = splp[1].splitn(2, " :").collect();
            } else {
                ret.prefix = None;
                splc = ret.raw.splitn(2, " :").collect();
            }
            let cmd_params: Vec<_> = splc[0].split(' ').collect();
            ret.command = Some(cmd_params[0]);
            for param in &cmd_params[1..] {
                ret.params.push(param);
            }
            if splc.len() == 2 {
                ret.params.push(splc[1]);
            }
        }
        Ok(ret)
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }
    pub fn prefix(&self) -> &Option<&str> {
        &self.prefix
    }
    pub fn command(&self) -> &Option<&str> {
        &self.command
    }
    pub fn params(&self) -> &Vec<&str> {
        &self.params
    }
}

impl<'a> Display for Line<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.raw)
    }
}

impl<'a> Debug for Line<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?} {:?} {:?}", self.prefix, self.command, self.params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Empty Line")]
    fn parse_empty() {
        match Line::parse("") {
            Ok(_) => { panic!("It worked, fail") },
            Err(_) => { panic!("Empty Line") },
        }
    }

    #[test]
    fn parse_ping() {
        let line = Line::parse("PING :Hello World");
        let l = match line {
            Ok(x) => x,
            Err(_) => panic!("Error Parsing"),
        };
        assert_eq!(l.raw, "PING :Hello World");
        assert_eq!(l.command.unwrap(), "PING");
        assert_eq!(l.params, vec!["Hello World"]);
    }
}

