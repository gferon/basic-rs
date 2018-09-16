#[macro_use]
extern crate nom;
extern crate cfg_if;
extern crate itertools;
extern crate wasm_bindgen;

use nom::types::CompleteStr;
use wasm_bindgen::prelude::*;

mod ast;
mod error;
mod format;
mod interpreter;
mod parser;

pub use error::Error;

pub fn execute(input: &str) -> Result<(String, String), Error> {
    let (remaining, ast) = parser::program(CompleteStr(input))?;
    if !remaining.is_empty() {
        return Err(Error::Parser(format!(
            "failed to parse program fully, remaining statements:\n{}",
            remaining
        )));
    }
    interpreter::evaluate(&ast?)
}

#[wasm_bindgen]
#[derive(Debug, Default)]
pub struct Output {
    stdout: String,
    stderr: String,
}

#[wasm_bindgen]
impl Output {
    pub fn stdout(&self) -> String {
        self.stdout.clone()
    }

    pub fn stderr(&self) -> String {
        self.stderr.clone()
    }
}

impl From<(String, String)> for Output {
    fn from((stdout, stderr): (String, String)) -> Self {
        Self { stdout, stderr }
    }
}

impl From<Error> for Output {
    fn from(e: Error) -> Self {
        Self {
            stdout: String::new(),
            stderr: format!("{}", e),
        }
    }
}

#[wasm_bindgen]
pub fn simple_execute(input: &str) -> Output {
    execute(input)
        .map(Output::from)
        .unwrap_or_else(Output::from)
}
