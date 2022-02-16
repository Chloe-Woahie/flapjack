use regex::Regex;

#[derive(Debug)]
pub struct DirectiveSequence {
    pub directives: Vec<Directive>,
}

#[derive(Debug)]
pub struct DirectiveSequenceBuilder {
    lines: Vec<String>,
}

// directive structure in the log will look like:
// INCREMENT checking-bank 46.70 "got paid"
#[derive(Debug)]
pub struct Directive {
    command: String,
    params: Vec<String>,
}

impl DirectiveSequence {
    /* pub fn write_sequence_to_file {

    } */
}

impl DirectiveSequenceBuilder {
    pub fn new(raw_log: String) -> Self {
        // go through a parsing process
        let lines = DirectiveSequenceBuilder::split_and_clean(raw_log);
        Self { lines }
    }

    pub fn build(&self) -> DirectiveSequence {
        let mut directives: Vec<Directive> = vec![];

        for line in &self.lines {
            // we remove and save the first part of the line to be the command
            /*  let mut split = line
            .split(" ")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|param| -> String { param.to_owned() })
            .collect::<Vec<String>>(); */

            let re = Regex::new(r#"(\S+)|"[^"]+""#).unwrap();
            let mut split: Vec<String> = re
                .find_iter(line)
                .filter_map(|chunk| Some(chunk.as_str().to_owned()))
                .collect();
            let command = split.remove(0);
            let directive = Directive {
                command: command,
                params: split,
            };

            directives.push(directive)
        }

        DirectiveSequence {
            directives: directives,
        }
    }

    fn split_and_clean(raw_log: String) -> Vec<String> {
        let split = raw_log.split("\r\n").collect::<Vec<&str>>();
        let mut cleaned: Vec<String> = vec![];

        for part in split {
            if (DirectiveSequenceBuilder::remove_whitespace(part) == "")
                || (part.chars().nth(0).unwrap() == '#')
            {
                continue;
            }
            //println!("{:?}", part);
            cleaned.push(part.to_owned());
        }

        cleaned
    }

    fn remove_whitespace(s: &str) -> String {
        s.split_whitespace().collect()
    }
}
