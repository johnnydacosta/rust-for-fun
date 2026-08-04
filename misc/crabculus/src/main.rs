fn main() {
    let infix = String::from("(10 + 2) * 3 / 45");
    println!("{}", infix);

    let tokens = tokenize(&infix);
    match tokens {
        Ok(t) => {
            println!("tokens = {:?}", t);
        }
        Err(e) => {
            eprintln!("{e}")
        }
    }
}

fn tokenize(infix: &String) -> Result<Vec<String>, String> {
    if infix.is_empty() {
        return Err(String::from("Cannot tokenize empty string."));
    }

   let mut tokens: Vec<String> = Vec::new();
   let mut buf: Vec<char> = Vec::new();

    for c in infix.chars() {
        if c.is_ascii_whitespace() {
            continue
        }
        if c.is_numeric() {
            buf.push(c);
            continue
        }
        if "/*-+()".contains(c) {
            if !buf.is_empty() {
                tokens.push(buf.iter().collect());
                buf.clear();
            }
            tokens.push(String::from(c));
        }
    }
    if !buf.is_empty() {
        tokens.push(buf.iter().collect());
        buf.clear();
    }

    Ok(tokens)
}