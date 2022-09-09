mod conditions;
mod echo;
mod executor;
mod lang_parser;
mod regex;
mod variables;

use executor::Executor;
use lang_parser::{LangParser, ParseErr};
use std::{env, fs, path::Path};
use thiserror::Error;

#[derive(Debug, Error)]
enum TopLevelErr {
    #[error("Missing file path: `{0}`")]
    MissingFilePath(String),
    #[error("`{0}` not found")]
    FileNotFound(String),
    #[error("IO Error: `{0}`")]
    IoError(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    ParseErr(#[from] ParseErr),
}

fn main() -> Result<(), TopLevelErr> {
    let file_path = env::args().nth(1).ok_or(TopLevelErr::MissingFilePath(
        "example: mybash ./src/main.mb".into(),
    ))?;

    let path = Path::new(&file_path);

    if !path.exists() {
        return Err(TopLevelErr::FileNotFound(file_path));
    }

    let content = fs::read_to_string(path)?;
    let result = content.parse::<LangParser>()?;
    Executor::execute(result.experssions);

    Ok(())
}
