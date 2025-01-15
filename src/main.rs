use std::{io::Write, process::ExitCode};

use clap::Parser;

/// Create an HTML file using custom markdown
#[derive(Parser)]
struct CLi {
    /// The style path
    style: std::path::PathBuf,
    /// The input path
    input: std::path::PathBuf,
    /// The name that will be given to the output file
    #[arg(default_value("./output.html"))]
    output: std::path::PathBuf,
    /// The html format file
    #[arg(default_value(None))]
    format: Option<std::path::PathBuf>,
}

#[derive(Debug)]
struct Element {
    key: String,
    _nickname: String,
    modifiers: Vec<Modifier>,
    html: (String, String),
}

#[derive(Debug, PartialEq)]
enum Modifier {
    Recursive,
    Interrupt,
    NewLine,
    Until(String),
}

const ESCAPE_CHAR: &str = "\\";

const DEBUG: bool = false;

fn main() -> ExitCode {
    if DEBUG {println!("--- RUNNING IN DEBUG MODE ---")}
    
    let args = CLi::parse();

    println!("Generating HTML file for input: {}, stylized with: {}", args.input.to_str().unwrap_or("Should be seeing the input file"), args.style.to_str().unwrap_or("Should be seeing the style file"));
    
    if DEBUG {println!("Checking files")}
    if args.input.extension() != Some(std::ffi::OsString::from("cmdf").as_ref()) {
        match args.input.extension() {
            Some(val) => {
                println!("Wrong extension '{}' for input file", val.to_str().unwrap_or("unknown"));
                return ExitCode::FAILURE;
            },
            None => {
                println!("Input file must have extension 'cmdf'");
                return ExitCode::FAILURE;
            }
        }
    }
    if args.style.extension() != Some(std::ffi::OsString::from("cmds").as_ref()) {
        match args.style.extension() {
            Some(val) => {
                println!("Wrong extension '{}' for style file, expected 'cmds'", val.to_str().unwrap_or("unknown"));
                return ExitCode::FAILURE;
            },
            None => {
                println!("Style file must have extension 'cmds'");
                return ExitCode::FAILURE;
            }
        }
    }

    let input_file = match std::fs::read_to_string(&args.input) {
        Ok(file) => file,
        Err(error) => {
            println!("Error reading input file, {}", error);
            return ExitCode::FAILURE;
        }
    };
    let style_file = match std::fs::read_to_string(&args.style) {
        Ok(val) => val,
        Err(error) => {
            println!("Error reading style file, {}", error);
            return ExitCode::FAILURE;
        }
    };

    let statements = style_file.split("}\n\n");
    let mut elements: Vec<Element>  = Vec::new();

    let mut has_new_line = false;

    for statement in statements {
        let s = match statement.find("{") {
            Some(index) => statement.split_at(index),
            None => {
            println!("Syntax error: missing '{{' in statement at line {}", statement.lines().next().unwrap_or(""));
            return ExitCode::FAILURE;
            }
        };

        let mut s0 = s.0.split_ascii_whitespace();

        let mut s1 = s.1.split("{{content}}");

        let key = match s0.next() {
            Some(val) => val.to_string(),
            None => {
                println!("Error fetching key on line {}", statement.lines().next().unwrap_or(""));
                return ExitCode::FAILURE;
            },
        };
        let mut nickname = String::new();
        let mut modifiers = Vec::new();

        loop {
            match s0.next() {
                Some(val) => {
                    if val == ":" {
                        nickname = match s0.next() {
                            Some(val) => val.to_string(),
                            None => {
                                println!("Syntax Error: Invalid nickname");
                                return ExitCode::FAILURE;
                            },
                        };
                    } else if val == "/" {
                        let next = match s0.next() {
                            Some(val) => val,
                            None => {
                                println!("Syntax Error on line: {}", statement.lines().next().unwrap_or(""));
                                return ExitCode::FAILURE;
                            }
                        };
                        modifiers.push(match next{
                            "recursive" => Modifier::Recursive,
                            "interrupt" => Modifier::Interrupt,
                            "new-line"   => {has_new_line = true; Modifier::NewLine},
                            "until" => Modifier::Until(match s0.next() {
                                Some(val) => val.to_string(),
                                None => {
                                    println!("Until modifier expected a value on line: {}", statement.lines().next().unwrap_or(""));
                                    return ExitCode::FAILURE;
                                }
                            }),
                            _ => {
                                println!("Unknown Modifier on line: {}", statement.lines().next().unwrap_or(""));
                                return ExitCode::FAILURE;
                            }
                        });
                    }
                },
                None => {
                    break;
                },
            }
        }

        let html: (String, String);

        if modifiers.contains(&Modifier::NewLine) {
            html = match s1.next() {
                Some(val) => (val.replace("{", "").replace("}", ""), String::new()),
                None => {
                    println!("HTML content is empty in line: {}", statement.lines().next().unwrap_or(""));
                    return ExitCode::FAILURE;
                }
            };
        } else {
            html = (
                match s1.next() {
                    Some(val) => val.replace("{", ""),
                    None => {
                        println!("Error reading HTML, check syntax on line: {}", statement.lines().next().unwrap_or(""));
                        return ExitCode::FAILURE;
                    }
                },
                match s1.next() {
                    Some(val) => val.to_string().replace("}", ""),
                    None => {
                        println!("Error reading HTML, check syntax on line: {}", statement.lines().next().unwrap_or(""));
                        return ExitCode::FAILURE;
                    }
                }
            );
        }

        elements.push(Element {
            key: key, 
            _nickname: nickname, 
            modifiers, 
            html: html,
        });

        if !has_new_line {
            elements.push(Element {
                key: String::from("\\n"),
                _nickname: String::from("new-line"),
                modifiers: vec![Modifier::NewLine],
                html: (String::from("<br>"), String::from("")),
            });
        }
    }

    // println!("{:?}", elements);

    let mut output = match std::fs::File::create_new(args.output.clone()) {
        Ok(file) => file,
        Err(error) => {
            println!("Error creating output file, {}", error);
            return ExitCode::FAILURE;
        }
    };
    // for _ in 0..input_file.lines().count() {
    //     let curr = match input.next() {
    //         Option::None => {break;}
    //         Option::Some(val) => val
    //     };

    //     output.write_all(gen_output(&elements, curr).as_bytes()).expect("");        

    //     // output.write_all(b"<br>\n").expect("Eroor writing to file");
    // }

    if DEBUG {println!("{:?}", input_file);}
    let binding = match gen_output(&elements, &input_file.replace("  \n", "\\")) {
        Ok(val) => val,
        Err(error) => {
            println!("Error Generating Output, {}", error);
            return ExitCode::FAILURE;
        }
    };
    let out = &binding.as_bytes();

    let html_format: (String, String) = match &args.format {
        Some(val) => {
            let form = match std::fs::read_to_string(val) {
                Ok(str) => str,
                Err(error) => {
                    println!("{}", error);
                    return ExitCode::FAILURE;
                }
            };
            let mut split_form = form.split("{{cumd}}");
            (match split_form.next() {
                Some(val) => val.to_string(),
                None => {
                    println!("Syntax error in the format file");
                    return ExitCode::FAILURE;
                }
            }, 
            match split_form.next() {
                Some(val) => val.to_string(),
                None => {
                    println!("Syntax error in the format file");
                    return ExitCode::FAILURE;
                }
            })
        },
        None => (String::from("<!DOCTYPE html> \n") +
            "<html lang=\"en\"> \n" +
            "<head> \n" +
                "<meta charset=\"UTF-8\"> \n" +
                "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"> \n" +
                "<title>Document</title> \n" +
                "<link rel=\"stylesheet\" href=\"style.css\">\n" +
            "</head> \n" +
            "<body>", String::from("</body>") +
            "</html>")
    };
    match output.write_all(html_format.0.as_bytes()) {
        Ok(_) => {},
        Err(error) => {
            println!("Error writing to output file, {}", error);
            return ExitCode::FAILURE;
        }
    }
    match output.write_all(out) {
        Ok(_) => {},
        Err(error) => {
            println!("Error writing to output file, {}", error);
            return ExitCode::FAILURE;
        }
    }
    match output.write_all(html_format.1.as_bytes()) {
        Ok(_) => {},
        Err(error) => {
            println!("Error writing to output file, {}", error);
            return ExitCode::FAILURE;
        }
    }

    println!("Succesfully generated HTML file at: {}", args.output.to_str().unwrap_or("You broke this in a unique kind of way"));
    return ExitCode::SUCCESS;
}

fn gen_output(elements: &Vec<Element>, curr: &str) -> Result<String, &'static str> {
    let mut out: String = String::new();

    let mut found_key = false;
    for element in elements {
        match element.modifiers.get(0) {
            Some(modi) => {
                match modi {
                    Modifier::Recursive => {
                        if match curr.split_ascii_whitespace().next() {
                            Some(n) => n,
                            None => {continue;},
                        } == element.key {
                            if DEBUG {println!("{:?}", element.key);}
                            let mut until = String::from("\\\\");
                            for modifier in &element.modifiers {
                                match modifier {
                                    Modifier::Until(val) => {until = val.to_string()},
                                    _ => continue,
                                }
                            }
                            if DEBUG {println!("{:?}", curr)};
                            let replaced_curr = curr.replacen(element.key.as_str(), "", 1);
                            let mut split_content = replaced_curr.splitn(2, &until);
                            let content = match gen_output(&elements, match split_content.next() {
                                Some(val) => val,
                                None => {
                                    return Err("An error occured while generating the output");
                                }
                            }) {
                                Ok(content) => content,
                                Err(error) => return Err(error),
                            };
                            out += element.html.0.as_str();
                            out += content.as_str();
                            out += element.html.1.as_str();
                            out += match &gen_output(&elements, match split_content.next() {
                                Some(val) => val,
                                None => "",
                            }) {
                                Ok(output) => &output,
                                Err(err) => return Err(err),
                            };
                            found_key = true;
                        }
                    },
                    Modifier::Interrupt => {
                        if curr.contains(element.key.as_str()) {
                            let full_content = curr.split_at(match curr.find(&element.key) {
                                Some(val) => val,
                                None => return Err("I dont really know what happened"),
                            });
                            let content = full_content.0.replace(element.key.as_str(), "");
                            out += element.html.0.as_str();
                            out += content.as_str();
                            out += element.html.1.as_str();
                            // out += full_content.1.replace(element.key.as_str(), "").replace("!", "").as_str();
                            // out += full_content.1.replace(element.key.as_str(), "").split_at(match full_content.1.find("\n") {Some(num) => {num}, None => full_content.1.len()}).0;
                            found_key = true;
                        }
                    },
                    Modifier::Until(val) => {
                        if curr.contains(&element.key) {
                            if DEBUG {println!("until");}
                            if element.modifiers.contains(&Modifier::Recursive){
                                let escape_key = ESCAPE_CHAR.to_string() + &element.key;
                                let replaced_curr = curr.replace(&escape_key, "¬¡“£¢∞§");
                                let snips = replaced_curr.split(&element.key);
                                let mut content: Vec<&str> = Vec::new();
                                for snip in snips {
                                    content.push(match snip.splitn(1, val).next() {
                                        Some(val) => val,
                                        None => return Err("Hmmm tell the dev you got line 359"),
                                    });
                                }
                                for i in 0..content.len() {
                                    if i % 2 == 1 {
                                        out += &element.html.0;
                                        out += match &gen_output(elements, content[i]) {
                                            Ok(val) => val,
                                            Err(err) => return Err(err),
                                        }.replace("¬¡“£¢∞§", &escape_key).as_str();
                                        out += &element.html.1;
                                    } else {
                                        out += match &gen_output(elements, content[i]) {
                                            Ok(val) => val,
                                            Err(err) => return Err(err),
                                        }.replace("¬¡“£¢∞§", &escape_key).as_str();
                                    }
                                }
                                found_key = true;
                            } else {
                                let escape_key = ESCAPE_CHAR.to_string() + &element.key;
                                let replaced_curr = curr.replace(&escape_key, "¬¡“£¢∞§");
                                let snips = replaced_curr.split(&element.key);
                                let mut content: Vec<&str> = Vec::new();
                                for snip in snips {
                                    content.push(match snip.splitn(1, val).next() {
                                        Some(val) => val,
                                        None => return Err("Hmmm tell the dev you got line 387"),
                                    });
                                }
                                for i in 0..content.len() {
                                    if i % 2 == 1 {
                                        out += &element.html.0;
                                        out += &content[i].replace("¬¡“£¢∞§",&element.key);
                                        out += &element.html.1;
                                    } else {
                                        out += match &gen_output(elements, content[i]) {
                                            Ok(val) => val,
                                            Err(err) => return Err(err),
                                        };
                                    }
                                }
                                found_key = true;
                            }
                        }
                    },
                    Modifier::NewLine => {
                        if curr.contains(&element.key){
                            if DEBUG {println!("newLine");}
                            let content = curr.split_at(match curr.find(&element.key) {
                                Some(val) => val,
                                None => return Err("Impossible you both contain the key and do not contain the key")
                            });
                            if DEBUG {println!("{:?}", curr);}
                            out += content.0;
                            out += &element.html.0;
                            out += match &gen_output(elements, content.1.replacen(&element.key, "", 1).as_str()) {
                                Ok(val) => val.as_str(),
                                Err(err) => return Err(err),
                            };
                            // out += curr.replace(&element.key, &element.html.0).as_str();
                            found_key = true;
                        }
                    },
                }
            }
            _ => {
                if match curr.split_ascii_whitespace().next() {
                    Some(n) => n,
                    None => {continue;},
                } == element.key {
                    let replaced = curr.replacen(element.key.as_str(), "", 1);
                    let content = replaced.split_at(match curr.find("\n"){
                        Some(val) => val,
                        None => 0,
                    });
                    out += element.html.0.as_str();
                    out += &content.0.replace("\n", "");
                    out += element.html.1.as_str();
                    if DEBUG {println!("none");}
                    if DEBUG {println!("{:?}", content.0);}
                    out += "\n";
                    out += match &gen_output(&elements, content.1) {
                        Ok(val) => val,
                        Err(err) => return Err(err),
                    };
                    found_key = true;
                }
            }
        }
        if found_key {break;}
    }
    if !found_key {
        if DEBUG {println!("plain text");}
        if DEBUG {println!("{:?}", curr);}
        out += curr;
    }
    return Ok(out);
}