use std::io::Write;

use clap::Parser;

/// Create an HTML file using custom markdown
#[derive(Parser)]
struct CLi {
    /// The style path
    style: std::path::PathBuf,
    /// The input path
    input: std::path::PathBuf,
    /// the name that will be given to the output file
    #[arg(default_value("output.html"))]
    output: String,
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
    Recursive/*(String)*/,
    Interrupt,
    ReplaceRecursive,
    Until(String),
}

fn main() {
    let args = CLi::parse();

    let input_file = std::fs::read_to_string(&args.input).expect("could not read file");
    let mut input = input_file.split("\n");
    let style_file = std::fs::read_to_string(&args.style).expect("could not read file");

    let statements = style_file.split("}\n\n");
    let mut elements: Vec<Element>  = Vec::new();

    for statement in statements {
        let s = statement.split_at(statement.find("{").expect("Syntax error"));
        let mut s0 = s.0.split_ascii_whitespace();

        let mut s1 = s.1.split("{{content}}");

        let key = s0.next().expect("Syntax Error").to_string();
        let mut nickname = String::new();
        let mut modifiers = Vec::new();

        loop {
            match s0.next() {
                Some(val) => {
                    if val == ":" {
                        nickname = s0.next().expect("msg").to_string();
                    } else if val == "/" {
                        modifiers.push(match s0.next().expect("Syntax Error"){
                            "recursive" => Modifier::Recursive/*(s0.next().expect("Expected stopping value for modifier 'recursive").to_string())*/,
                            "interrupt" => Modifier::Interrupt,
                            "replace-recursive"   => Modifier::ReplaceRecursive,
                            "until" => Modifier::Until(s0.next().expect("expect value for modifier 'until'").to_string()),
                            _ => panic!("Unknown modifier")
                        });
                    }
                },
                None => {
                    break;
                },
            }
        }

        let html;

        if modifiers.contains(&Modifier::ReplaceRecursive) {
            html = (s1.next().expect("html is empty").replace("{", "").replace("}", ""), String::new());
        } else {
            html = (s1.next().expect("no motr spliut").replace("{", ""), 
            s1.next().expect("no motr spliut").to_string().replace("}", ""));
        }

        elements.push(Element {
            key: key, 
            _nickname: nickname, 
            modifiers, 
            html: html,
        });
    }

    println!("{:?}", elements);

    let mut output = std::fs::File::create_new(args.output).expect("Error creating output file");
    loop {
        let curr = match input.next() {
            Option::None => {break;}
            Option::Some(val) => val
        };

        output.write_all(gen_output(&elements, curr).as_bytes()).expect("");        

        // output.write_all(b"<br>\n").expect("Eroor writing to file");
    }
}

fn gen_output(elements: &Vec<Element>, curr: &str) -> String {
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
                            } == element.key{
                                let content = gen_output(elements, &curr.replacen(element.key.as_str(), "", 1));
                                out += element.html.0.as_str();
                                out += content.as_str();
                                out += element.html.1.as_str();
                                found_key = true;
                            }
                        },
                        Modifier::Interrupt => {
                            if curr.contains(element.key.as_str()) {
                                let full_content = curr.split_at(curr.find(&element.key).expect("msg"));
                                let content = full_content.0.replace(element.key.as_str(), "");
                                out += element.html.0.as_str();
                                out += content.as_str();
                                out += element.html.1.as_str();
                                // out += full_content.1.replace(element.key.as_str(), "").replace("!", "").as_str();
                                // out += full_content.1.replace(element.key.as_str(), "").split_at(match full_content.1.find("\n") {Some(num) => {num}, None => full_content.1.len()}).0;
                                found_key = true;
                            }
                        },
                        Modifier::ReplaceRecursive => {
                            if curr.contains(&element.key){
                                let content = curr.split_at(curr.find(&element.key).expect("impossible"));
                                out += content.0;
                                out += &element.html.0;
                                out += gen_output(elements, content.1.replacen(&element.key, "", 1).as_str()).as_str();
                                // out += curr.replace(&element.key, &element.html.0).as_str();
                                found_key = true;
                            }
                        },
                        Modifier::Until(_val) => {
                            panic!("The modifier 'until' is still Unimplemented.")
                        },
                    }
                }
                _ => {
                    if match curr.split_ascii_whitespace().next() {
                        Some(n) => n,
                        None => {continue;},
                    } == element.key {
                        let content = curr.replacen(element.key.as_str(), "", 1);
                        out += element.html.0.as_str();
                        out += content.as_str();
                        out += element.html.1.as_str();
                        found_key = true;
                    }
                }
            }
            if found_key {break;}
        }
        if !found_key {
            out += curr;
        }
        return out;
}

