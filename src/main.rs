use std::env;
use std::fmt;

#[derive(Debug)]
enum Node {
    Leaf(i32),
    Branch(Vec<Node>),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Leaf(value) => write!(f, "{}", value),
            Node::Branch(children) => {
                write!(f, "[")?;
                for (i, child) in children.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", child)?;
                }
                write!(f, "]")
            }
        }
    }
}

#[derive(Debug)]
struct ParseError {
    message: String,
    position: usize,
}

impl ParseError {
    fn new(message: &str, position: usize) -> Self {
        Self {
            message: message.to_string(),
            position,
        }
    }
}

fn skip_whitespace(input: &str, pos: &mut usize) {
    while let Some(ch) = input.chars().nth(*pos) {
        if ch.is_whitespace() {
            *pos += 1;
        } else {
            break;
        }
    }
}

fn parse_node(input: &str, pos: &mut usize) -> Result<Node, ParseError> {
    skip_whitespace(input, pos);

    match input.chars().nth(*pos) {
        Some('[') => {
            *pos += 1;
            let mut children = Vec::new();
            let mut first = true;

            loop {
                skip_whitespace(input, pos);
                if let Some(']') = input.chars().nth(*pos) {
                    *pos += 1;
                    return Ok(Node::Branch(children));
                }

                if !first {
                    if input.chars().nth(*pos) == Some(',') {
                        *pos += 1;
                        skip_whitespace(input, pos);
                    } else {
                        return Err(ParseError::new("Expected comma between branch children", *pos));
                    }
                }

                if *pos >= input.len() {
                    return Err(ParseError::new("Unterminated branch", *pos));
                }

                children.push(parse_node(input, pos)?);
                first = false;
            }
        }
        Some(c) if c == '-' || c.is_ascii_digit() => {
            let start = *pos;
            if c == '-' {
                *pos += 1;
            }
            let mut has_digits = false;
            while let Some(d) = input.chars().nth(*pos) {
                if d.is_ascii_digit() {
                    has_digits = true;
                    *pos += 1;
                } else {
                    break;
                }
            }
            if !has_digits {
                return Err(ParseError::new("Expected integer leaf value", start));
            }
            let number_str = &input[start..*pos];
            let value = number_str.parse::<i32>().map_err(|_| ParseError::new("Invalid integer value", start))?;
            Ok(Node::Leaf(value))
        }
        Some(_) => Err(ParseError::new("Unexpected token", *pos)),
        None => Err(ParseError::new("Unexpected end of input", *pos)),
    }
}

fn parse_tree(input: &str) -> Result<Node, ParseError> {
    let mut pos = 0;
    let node = parse_node(input, &mut pos)?;
    skip_whitespace(input, &mut pos);
    if pos != input.len() {
        return Err(ParseError::new("Unexpected trailing characters", pos));
    }
    Ok(node)
}

fn minimax(node: &Node, maximize: bool) -> i32 {
    match node {
        Node::Leaf(value) => *value,
        Node::Branch(children) => {
            let values = children.iter().map(|child| minimax(child, !maximize));
            if maximize {
                values.max().unwrap_or(i32::MIN)
            } else {
                values.min().unwrap_or(i32::MAX)
            }
        }
    }
}

fn usage(program: &str) {
    eprintln!("Usage: {} '<nested-list>'", program);
    eprintln!("Example: {} '[1,[2,3],[[4],5]]'", program);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = if args.len() == 2 {
        args[1].trim().to_string()
    } else {
        eprintln!("No nested list provided. Using sample input.");
        "[1,[2,3],[[4],5]]".to_string()
    };

    match parse_tree(&input) {
        Ok(tree) => {
            println!("Parsed tree: {}", tree);
            let result = minimax(&tree, true);
            println!("Minimax result: {}", result);
        }
        Err(err) => {
            eprintln!("Parse error at position {}: {}", err.position, err.message);
            usage(&args[0]);
            std::process::exit(1);
        }
    }
}
