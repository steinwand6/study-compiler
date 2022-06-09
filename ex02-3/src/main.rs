fn main() {
    convert_to_re_polish_from_infix("a + b * c + d");
    convert_to_re_polish_from_infix("(a + b) * c + d");
    convert_to_re_polish_from_infix("a + b * c * (d + e)");
    convert_to_re_polish_from_infix("a * b * c + d + e");
}

fn convert_to_re_polish_from_infix(infix: &str) {
    let mut stack = Vec::<char>::new();
    stack.push('(');
    for part in infix.chars() {
        match part {
            'a'..='z' => print!("{}", part),
            '*' | '/' => {
                while stack.len() > 0 {
                    match stack[stack.len() - 1] {
                        '*' | '/' => print!("{}", stack.pop().unwrap()),
                        _ => break,
                    }
                }
                stack.push(part)
            }
            '+' | '-' => {
                while stack.len() > 0 {
                    match stack[stack.len() - 1] {
                        '*' | '/' | '+' | '-' => print!("{}", stack.pop().unwrap()),
                        _ => break,
                    }
                }
                stack.push(part)
            }
            ')' => {
                while stack.len() > 0 {
                    match stack[stack.len() - 1] {
                        '*' | '/' | '+' | '-' => print!("{}", stack.pop().unwrap()),
                        ')' => _ = stack.pop(),
                        _ => break,
                    }
                }
                stack.push(part)
            }
            '(' => {
                while stack.len() > 0 {
                    match stack[stack.len() - 1] {
                        '*' | '/' | '+' | '-' => print!("{}", stack.pop().unwrap()),
                        ')' | '(' => _ = stack.pop(),
                        _ => break,
                    }
                }
                stack.push(part)
            }
            _ => {}
        }
    }
    while stack.len() > 0 {
        match stack[stack.len() - 1] {
            '*' | '/' | '+' | '-' => print!("{}", stack.pop().unwrap()),
            ')' => _ = stack.pop(),
            _ => break,
        }
    }
    println!("")
}
