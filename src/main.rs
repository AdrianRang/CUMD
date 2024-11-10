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
    Recursive,
    Interrupt,
    newLine,
    Until(String),
}

const ESCAPE_CHAR: &str = "\\";

fn main() {

    let args = CLi::parse();

    let input_file = std::fs::read_to_string(&args.input).expect("could not read file");
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
                            "recursive" => Modifier::Recursive,
                            "interrupt" => Modifier::Interrupt,
                            "new-line"   => Modifier::newLine,
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

        if modifiers.contains(&Modifier::newLine) {
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

    // println!("{:?}", elements);

    let mut output = std::fs::File::create_new(args.output).expect("Error creating output file");
    // for _ in 0..input_file.lines().count() {
    //     let curr = match input.next() {
    //         Option::None => {break;}
    //         Option::Some(val) => val
    //     };

    //     output.write_all(gen_output(&elements, curr).as_bytes()).expect("");        

    //     // output.write_all(b"<br>\n").expect("Eroor writing to file");
    // }
    println!("{:?}", input_file);
    output.write_all(gen_output(&elements, &input_file).as_bytes()).expect("msg");
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
                        } == element.key {
                            println!("{:?}", element.key);
                            let mut until = String::from("\\\\");
                            for modifier in &element.modifiers {
                                match modifier {
                                    Modifier::Until(val) => {until = val.to_string()},
                                    _ => continue,
                                }
                            }
                            println!("{:?}", curr);
                            let replaced_curr = curr.replacen(element.key.as_str(), "", 1);
                            let mut split_content = replaced_curr.splitn(2, &until);
                            let content = gen_output(&elements, split_content.next().expect("smth went wromng"));
                            out += element.html.0.as_str();
                            out += content.as_str();
                            out += element.html.1.as_str();
                            out += &gen_output(&elements, match split_content.next() {
                                Some(val) => val,
                                None => "",
                            });
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
                    Modifier::Until(val) => {
                        if curr.contains(&element.key) {
                            println!("until");
                            if element.modifiers.contains(&Modifier::Recursive){
                                let escape_key = ESCAPE_CHAR.to_string() + &element.key;
                                let replaced_curr = curr.replace(&escape_key, "¬¡“£¢∞§");
                                let snips = replaced_curr.split(&element.key);
                                let mut content: Vec<&str> = Vec::new();
                                for snip in snips {
                                    content.push(snip.splitn(1, val).next().expect("no finish"));
                                }
                                for i in 0..content.len() {
                                    if i % 2 == 1 {
                                        out += &element.html.0;
                                        out += &gen_output(elements, content[i]).replace("¬¡“£¢∞§", &escape_key);
                                        out += &element.html.1;
                                    } else {
                                        out += &gen_output(elements, content[i]).replace("¬¡“£¢∞§", &escape_key);
                                    }
                                }
                                found_key = true;
                            } else {
                                let escape_key = ESCAPE_CHAR.to_string() + &element.key;
                                let replaced_curr = curr.replace(&escape_key, "¬¡“£¢∞§");
                                let snips = replaced_curr.split(&element.key);
                                let mut content: Vec<&str> = Vec::new();
                                for snip in snips {
                                    content.push(snip.splitn(1, val).next().expect("no finish"));
                                }
                                for i in 0..content.len() {
                                    if i % 2 == 1 {
                                        out += &element.html.0;
                                        out += &content[i].replace("¬¡“£¢∞§",&element.key);
                                        out += &element.html.1;
                                    } else {
                                        out += &gen_output(elements, content[i]);
                                    }
                                }
                                found_key = true;
                            }
                        }
                    },
                    Modifier::newLine => {
                        if curr.contains(&element.key){
                            println!("newLine");
                            let content = curr.split_at(curr.find(&element.key).expect("impossible"));
                            println!("{:?}", curr);
                            out += content.0;
                            out += &element.html.0;
                            out += gen_output(elements, content.1.replacen(&element.key, "", 1).as_str()).as_str();
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
                    println!("none");
                    println!("{:?}", content.0);
                    out += "\n";
                    out += &gen_output(&elements, content.1);
                    found_key = true;
                }
            }
        }
        if found_key {break;}
    }
    if !found_key {
        println!("plain text");
        println!("{:?}", curr);
        out += curr;
    }
    return out;
}