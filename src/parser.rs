use ast::*;
use error::Error;

use nom::types::CompleteStr;
use nom::{eol, space};

use std::num;

// 4. Syntax

fn is_letter(c: char) -> bool {
    match c {
        'A'..='Z' => true,
        _ => false,
    }
}

fn is_digit(c: char) -> bool {
    match c {
        '0'..='9' => true,
        _ => false,
    }
}

// 6. Constants

named!(string_constant<CompleteStr, StringConstant>,
    do_parse!(
        s: delimited!(tag!("\""), take_until!("\""), tag!("\"")) >>
        (StringConstant(s.to_string()))
    ));

// 7. Variables

named!(numeric_variable<CompleteStr, NumericVariable>,
    alt!(simple_numeric_variable));

named!(simple_numeric_variable<CompleteStr, NumericVariable>,
    do_parse!(
        letter: take_while_m_n!(1, 1, is_letter) >>
        digit: opt!(map_res!(take_while_m_n!(1, 1, is_digit), from_decimal)) >>
        (NumericVariable::Simple {
            letter: letter.chars().next().unwrap(), digit })
    ));

named!(string_variable<CompleteStr, StringVariable>,
    do_parse!(
        letter: take_while_m_n!(1, 1, is_letter) >>
        tag!("$") >>
        (StringVariable { letter: letter.chars().next().unwrap() })
    ));

// 8. Expressions

named!(expression<CompleteStr, Expression>,
    alt!(
        map!(numeric_expression, Expression::Numeric) |
        map!(string_expression, Expression::String)
    ));

named!(string_expression<CompleteStr, StringExpression>,
    alt!(
        map!(string_variable, StringExpression::Variable) |
        map!(string_constant, StringExpression::Constant)
    ));

named!(numeric_expression<CompleteStr, NumericExpression>,
    map_res!(take_while_m_n!(1, 10, is_digit),
        |_| -> Result<NumericExpression, ()> {
            Ok(NumericExpression)
        })
    );

// 11. LET statement

named!(let_statement<CompleteStr, Statement>,
    map!(alt!(numeric_let_statement | string_let_statement), Statement::Let));

named!(numeric_let_statement<CompleteStr, LetStatement>,
    do_parse!(
        tag!("LET") >> space >>
        variable: numeric_variable >>
        opt!(space) >>
        tag!("=") >>
        opt!(space) >>
        expression: numeric_expression >>
        (LetStatement::Numeric{ variable, expression })
    ));

named!(string_let_statement<CompleteStr, LetStatement>,
    do_parse!(
        tag!("LET") >> space >>
        variable: string_variable >>
        opt!(space) >>
        tag!("=") >>
        opt!(space) >>
        expression: string_expression >>
        (LetStatement::String{ variable, expression })
    ));

// 14. PRINT statement

named!(print_item_expression<CompleteStr, PrintItem>,
    map!(expression, PrintItem::Expression));

named!(print_item_tab<CompleteStr, PrintItem>,
    do_parse!(
        tag!("TAB(") >>
        numeric_expression: numeric_expression >>
        tag!(")") >>
        (PrintItem::TabCall(numeric_expression))
    ));

named!(print_item<CompleteStr, PrintItem>,
    alt!(print_item_expression | print_item_tab));

named!(print_item_comma<CompleteStr, PrintItem>,
    map!(tag!(","), |_| PrintItem::Comma));

named!(print_item_semicolon<CompleteStr, PrintItem>,
    map!(tag!(";"), |_| PrintItem::Semicolon));

named!(print_separator<CompleteStr, PrintItem>,
    do_parse!(
        many0!(space) >>
        sep: alt!(print_item_comma | print_item_semicolon) >>
        (sep)
    ));

named!(print_list<CompleteStr, Vec<PrintItem>>,
    do_parse!(
        items: many0!(pair!(opt!(print_item), print_separator)) >>
        trailing_item: opt!(print_item) >>
        (new_print_items(items, trailing_item))
    ));

named!(print_statement<CompleteStr, Statement>,
    do_parse!(
        tag!("PRINT") >>
        print_list: opt!(sep!(space, print_list)) >>
        (Statement::Print(PrintStatement{ list: print_list.unwrap_or_else(Vec::new) }))
    ));

// Program

fn from_decimal(input: CompleteStr) -> Result<u16, num::ParseIntError> {
    u16::from_str_radix(&input, 10)
}

named!(line_number<CompleteStr, u16>,
    map_res!(take_while_m_n!(1, 4, is_digit), from_decimal));

named!(stop_statement<CompleteStr, Statement>,
    do_parse!(
        tag!("STOP") >>
        (Statement::Stop)
    ));

named!(end_statement<CompleteStr, Statement>,
    do_parse!(
        tag!("END") >>
        (Statement::End)
    ));

named!(statement<CompleteStr, Statement>,
    do_parse!(
        statement: alt!(print_statement | let_statement | stop_statement | end_statement) >>
        (statement)));

named!(pub block<CompleteStr, Block>,
    do_parse!(
        line_number: line_number >>
        statement: sep!(space, statement) >>
        opt!(space) >>
        (Block::Line{ line_number, statement })
    ));

named!(end_of_line<CompleteStr, CompleteStr>, alt!(eof!() | eol));

named!(pub program<CompleteStr, Result<Program, Error>>,
    do_parse!(
        blocks: many0!(terminated!(block, end_of_line)) >>
        opt!(end_of_line) >>
        (Program::new(blocks))
    ));
