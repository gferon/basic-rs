use ast::*;

fn evaluate_numeric_expression(expression: &NumericExpression) -> u64 {
    42
}

fn evaluate_print(statement: &PrintStatement, output: &mut String) {
    for item in &statement.list {
        match item {
            PrintItem::Expression(expression) => match expression {
                Expression::String(string_expression) => match string_expression {
                    StringExpression::Constant(constant) => {
                        *output += &constant.0;
                    }
                    _ => (),
                },
                _ => (),
            },
            PrintItem::TabCall(numeric_expression) => {
                for _ in 0..evaluate_numeric_expression(numeric_expression) {
                    *output += "\t"
                }
            }
            PrintItem::Comma => *output += "\t",
            PrintItem::Semicolon => (),
        }
    }

    let last_item_is_semicolon = statement
        .list
        .last()
        .map(|s| match s {
            PrintItem::Semicolon => true,
            _ => false,
        })
        .unwrap_or(false);
    if !last_item_is_semicolon {
        output.push('\n');
    }
}

// Return value `true` means evalution should continue.
fn evaluate_statement(statement: &Statement, output: &mut String) -> bool {
    match statement {
        Statement::Print(statement) => {
            evaluate_print(statement, output);
            true
        }
        Statement::Let(_) => true,
        Statement::Stop => false,
        Statement::End => false,
    }
}

pub fn evaluate(program: &Program) -> String {
    let mut output = String::new();
    for block in &program.blocks {
        match block {
            Block::Line { statement, .. } => {
                if !evaluate_statement(statement, &mut output) {
                    break;
                }
            }
        }
    }
    output
}
