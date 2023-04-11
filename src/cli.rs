use core::fmt;
use std::{str::FromStr, fmt::Display};

pub enum Commands {
    GenerateProject,
    GenerateDay,
    RunTest,
}

impl FromStr for Commands {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "generate" => Ok(Commands::GenerateProject),
            "day" => Ok(Commands::GenerateDay),
            "test" => Ok(Commands::RunTest),
            _ => Err(format!("could not find command: {}", input)),
        }
    }
}

pub enum Language {
    TypeScript,
    Rust,
    Go,
    Cpp,
    Java
}

impl FromStr for Language {
    type Err = String;

    fn from_str(input: &str) -> Result<Language, Self::Err> {
        match input {
            "typescript" => Ok(Language::TypeScript),
            "rust" => Ok(Language::Rust),
            "go" => Ok(Language::Go),
            "c++" => Ok(Language::Cpp),
            "java" => Ok(Language::Java),
            _ => Err(format!("could not find language: {}", input)),
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            Language::TypeScript => "typescript",
            Language::Rust => "rust",
            Language::Go => "go",
            Language::Cpp => "c++",
            Language::Java => "java",
        };
        write!(f, "{}", out)
    }
}

pub struct CLI {
    pub language: Language,
    pub command: Commands,
}
