use crate::cyber_gx_interpreter::lexer::{Token, TokenType};

pub struct Interpreter;

impl Interpreter {
    pub fn execute(tokens: Vec<Token>) {
        let mut output = String::new();
        let mut is_println = false;
        let mut values = Vec::new();
        let mut i = 0;

        while i < tokens.len() {
            match tokens[i].token_type {
                TokenType::Println => {
                    if is_println {
                        Self::format_and_print(&output, &values);
                        output.clear();
                        values.clear();
                    }
                    is_println = true;
                }
                TokenType::String => output.push_str(&tokens[i].value),
                TokenType::Operator if tokens[i].value == "<<" => output.push(' '),
                TokenType::Number => values.push(tokens[i].value.clone()),
                TokenType::Plus => {
                    if let Some(last) = values.pop() {
                        let left: i32 = last.parse().unwrap_or(0);
                        let right: i32 = tokens[i + 1].value.parse().unwrap_or(0);
                        values.push((left + right).to_string());
                        i += 1;
                    }
                }
                TokenType::Placeholder => output.push_str("{}"),
                _ => {}
            }
            i += 1;
        }

        if is_println {
            Self::format_and_print(&output, &values);
        }
    }

    fn format_and_print(text: &str, values: &[String]) {
        let mut result = String::new();
        #[allow(unused_mut)]
        #[allow(unused_variables)]
        let mut index = 0;
        let parts: Vec<&str> = text.split("{}").collect();

        for (i, part) in parts.iter().enumerate() {
            result.push_str(part);
            if i < values.len() {
                result.push_str(&values[i]);
            } else if i < parts.len() - 1 {
                result.push_str("{}");
            }
        }

        println!("{}", result);
    }
}
