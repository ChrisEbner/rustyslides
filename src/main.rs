//! A slide creation tool for presentations written in Rust
//! Written in 2020 by 
//!  Christian Ebner <chris@christian-ebner.com>


use std::env;
use std::io::Error;


enum TemplateType {
    Paragraph,
    Headline
}

struct Template {
    ttype: TemplateType,
    tstring: String
}

/// Wat
fn main() {
    let args: Vec<String> = env::args().collect();

    let input_string = &args[1];
    
    let template = convert_to_template(&input_string, derive_input_type(&args[2]).unwrap());

    match template {
        Some(t) => println!("Converted to: {}", t.tstring),
        None => println!("Whoops")
    }
}

/// This should be removed as soon as Input types are derived from the Message that is relayed.
fn derive_input_type(env_input: &str) -> Result<TemplateType, Error> {
    match env_input {
        "Paragraph" => Ok(TemplateType::Paragraph),
        "Headline" => Ok(TemplateType::Headline),
        _ => panic!("Problem!")
    }
}

fn convert_to_template(input_string: &str, template_type: TemplateType) -> Option<Template> {
    match template_type {
        TemplateType::Paragraph => {
            let mut o_string: String = String::from("<p>");

            o_string.push_str(input_string);
            o_string.push_str("</p>");

            return Some(Template {
                ttype: TemplateType::Paragraph,
                tstring: o_string
            })
        },
        TemplateType::Headline => {
            let mut o_string: String = String::from("<h1>");

            o_string.push_str(input_string);
            o_string.push_str("</h1>");

            return Some(Template {
                ttype: TemplateType::Headline,
                tstring: o_string
            })
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_return_() {
        assert_eq!(1, 1)
    }
}
