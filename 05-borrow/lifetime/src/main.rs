struct TextParser<'a> {
    document: &'a str,
}

impl<'a> TextParser<'a> {
    fn first_word(&self) -> &'a str {
        let bytes = self.document.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &self.document[0..i];
            }
        }
        self.document
    }
}

fn main() {
    let text = String::from("Rust is awesome");
    let parser = TextParser { document: &text };

    let word = parser.first_word();
    println!("First word: {}", word);
}