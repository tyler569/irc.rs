
pub struct Line<'a> {
    raw:      String,
    prefix:   Option<&'a str>,
    command:  &'a str,
    params:   Vec<&'a str>,
}

impl<'a> Line<'a> {
    pub fn new() -> Line<'a> {
        Line {
            raw: String::new(),
            prefix: None,
            command: "",
            params: Vec::new(),
        }
    }

    pub fn parse(&'a mut self, line: String) -> Result<(), &'static str> {

        if line.len() == 0 {
            return Err("Empty Line");
        }

        self.raw = line;
        
        let mut splc: Vec<_>;
        if &self.raw[0..1] == ":" {
            let splp: Vec<_> = self.raw.splitn(2, ' ').collect();
            self.prefix = Some(splp[0]);
            splc = splp[1].splitn(2, " :").collect();
        } else {
            self.prefix = None;
            splc = self.raw.splitn(2, " :").collect();
        }
        let cmd_params: Vec<_> = splc[0].split(' ').collect();
        self.command = cmd_params[0];
        self.params.push_all(cmd_params.tail());
        if splc.len() == 2 {
            self.params.push(splc[1]);
        }

        Ok(())
    }

    pub fn raw(&'a self) -> &'a String {
        &self.raw
    }
    pub fn prefix(&'a self) -> Option<&'a str> {
        self.prefix
    }
    pub fn command(&'a self) -> &'a str {
        self.command
    }
    pub fn params(&'a self) -> &'a Vec<&'a str> {
        &self.params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Empty Line")]
    fn parse_empty() {
        let test = "".to_string();
        let mut line = Line::new();
        match line.parse(test) {
            Ok(_) => { panic!("It worked, fail") },
            Err(_) => { panic!("Empty Line") },
        }
    }

    #[test]
    fn parse_ping() {
        let test = "PING :Hello World".to_string();
        let mut line = Line::new();
        line.parse(test);

        assert_eq!(line.raw, "PING :Hello World".to_string());
        assert_eq!(line.command, "PING");
        assert_eq!(line.params, vec!["Hello World"]);
    }
}

