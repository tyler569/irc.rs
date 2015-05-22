
use std::fmt::{Debug, Display, Formatter, Error};

pub struct Line {
    raw:      String,
    prefix:   Option<String>,
    command:  String,
    params:   Vec<String>,
}

impl Line {
    pub fn parse(line: String) -> Result<Line, &'static str> {
        let mut ret = Line { 
            raw: line,
            prefix: None,
            command: String::new(),
            params: Vec::new()
        };
        {
            if ret.raw.len() == 0 {
                return Err("Empty Line");
            }

            let mut splc: Vec<_>;
            if &ret.raw[0..1] == ":" {
                let splp: Vec<_> = ret.raw.splitn(2, ' ').collect();
                ret.prefix = Some(splp[0].trim_left_matches(':').to_string());
                splc = splp[1].splitn(2, " :").collect();
            } else {
                ret.prefix = None;
                splc = ret.raw.splitn(2, " :").collect();
            }
            let cmd_params: Vec<_> = splc[0].split(' ').collect();
            ret.command = cmd_params[0].to_string();
            for param in &cmd_params[1..] {
                ret.params.push(param.to_string());
            }
            if splc.len() == 2 {
                ret.params.push(splc[1].to_string());
            }
        }

        Ok(ret)
    }

    pub fn raw(&self) -> &String {
        &self.raw
    }
    pub fn prefix(&self) -> &Option<String> {
        &self.prefix
    }
    pub fn command(&self) -> &String {
        &self.command
    }
    pub fn params(&self) -> &Vec<String> {
        &self.params
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.raw)
    }
}

impl Debug for Line {
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
        let test = "".to_string();
        match Line::parse(test) {
            Ok(_) => { panic!("It worked, fail") },
            Err(_) => { panic!("Empty Line") },
        }
    }

    #[test]
    fn parse_ping() {
        let test = "PING :Hello World".to_string();
        let line = Line::parse(test).ok().unwrap();
        assert_eq!(line.raw, "PING :Hello World".to_string());
        assert_eq!(line.command, "PING");
        assert_eq!(line.params, vec!["Hello World"]);
    }
}

