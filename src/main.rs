//! A slide creation tool for presentations written in Rust
//! Written in 2020 by 
//!  Christian Ebner <chris@christian-ebner.com>

use std::env;
mod Markdownparser;


enum TemplateType {
    Paragraph,
    Headline
}

struct Template {
    ttype: TemplateType,
    tstring: String
}


// Wat
fn main() {
    let args: Vec<String> = env::args().collect();

    let input_string = &args[1];

    let parser: Markdownparser::Parser = Markdownparser::Parser::new();

    let ast = parser.parse_to_blocks(input_string);

    for block in ast {
        println!("{:?}", block);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_return_() {
        assert_eq!(1, 1)
    }
}
