const INSTRUCTIONS: [&str; 11] = [
    "FUNC",
    "END",
    "CREATE_VAR",
    "SET_VAR",
    "EQ",
    "NE",
    "LT",
    "LE",
    "GT",
    "GE",
    "SHOW_ALL",
];

pub fn parse_line(line: &str) -> Result<Vec<String>, String> {
    let mut line_vec = Vec::new();
    let mut instruction = String::new();

    let mut current_index = 0;

    while current_index < line.len() {
        let current_character = line.chars().nth(current_index).unwrap();

        if current_character == ' ' {
            break;
        }

        current_index += 1;
        instruction.push(current_character);
    }

    if !INSTRUCTIONS.contains(&instruction.as_str()) {
        return Err(format!("Instruction not found: {instruction}"));
    }

    line_vec.push(instruction);
    current_index += 1;

    let mut arg = String::new();
    let mut in_string = false;

    while current_index < line.len() {
        let current_character = line.chars().nth(current_index).unwrap();

        if current_character == ';' {
            break;
        } else if current_character == ' ' && !in_string {
            line_vec.push(arg);
            arg = String::new();

            current_index += 1;
            continue;
        } else if current_character == '"' && !in_string {
            in_string = true;
        } else if current_character == '"' {
            in_string = false;

            arg.push('"');
            line_vec.push(arg);
            arg = String::new();

            current_index += 1;
            continue;
        } else if current_character == '\\' && in_string {
            let next_character = line.chars().nth(current_index + 1).unwrap();

            if next_character == 'n' {
                arg.push('\n');
            } else if next_character == 't' {
                arg.push('\t');
            } else if next_character == 'r' {
                arg.push('\r');
            } else if next_character == '\\' {
                arg.push('\\');
            } else if next_character == '0' {
                arg.push('\0');
            } else if next_character == '"' {
                arg.push('"');
            } else {
                return Err("invalid escape".to_owned());
            }

            current_index += 2;
            continue;
        } else if current_character == '\'' && !in_string {
            let mut char_arg = String::from(current_character);
            let next_character = line.chars().nth(current_index + 1).unwrap();
            let next2_character = line.chars().nth(current_index + 2).unwrap();

            if next_character == '\\' {
                let next3_character = line.chars().nth(current_index + 3).unwrap();

                if next2_character == 'n' {
                    char_arg.push('\n');
                } else if next2_character == 't' {
                    char_arg.push('\t');
                } else if next2_character == 'r' {
                    char_arg.push('\r');
                } else if next2_character == '\\' {
                    char_arg.push('\\');
                } else if next2_character == '0' {
                    char_arg.push('\0');
                } else if next2_character == '\'' {
                    char_arg.push('\'');
                } else {
                    return Err("invalid escape character".to_owned());
                }

                if next3_character != '\'' {
                    return Err("character not closed".to_owned());
                }

                char_arg.push(next3_character);

                current_index += 4;
            } else {
                char_arg.push(next_character);
                char_arg.push(next2_character);
                current_index += 3;
            }

            line_vec.push(char_arg);
            continue;
        }

        arg.push(current_character);
        current_index += 1;
    }

    if in_string {
        return Err("string not closed".to_owned());
    }

    if !arg.is_empty() {
        line_vec.push(arg);
    }

    Ok(line_vec)
}
