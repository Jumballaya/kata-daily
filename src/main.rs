use std::{str::FromStr, env, io};
use config::Config;
use languages::typescript::typescript::Typescript;

mod cli;
mod languages;
mod config;

fn main() -> Result<(), io::Error> {
    let lang_choice = std::env::args().nth(1).expect("No language given").to_lowercase();
    let command_choice = std::env::args().nth(2).expect("No command given").to_lowercase();
    let lang = cli::Language::from_str(lang_choice.as_str()).unwrap();
    let cmd = cli::Commands::from_str(command_choice.as_str()).unwrap();
    let args = cli::CLI { language: lang, command: cmd };
    let pwd_buf = env::current_dir()?;
    let pwd = String::from(pwd_buf.to_str().expect("Could not get pwd"));

    match args.language {
        cli::Language::TypeScript => {
            let ts = Typescript::new(1, pwd.clone());

            match args.command {
                cli::Commands::GenerateProject => {
                    let config = Config::new();
                    config.write(format!("{}", pwd.as_str()))?;
                    ts.generate();
                },
                cli::Commands::RunTest => {
                    let config = Config::from_file()?;
                    ts.run_test(config.day);
                },
                cli::Commands::GenerateDay => {
                    let mut config = Config::from_file()?;
                    config.day += 1;
                    ts.generate_day(config.day);
                    config.write(format!("{}", pwd.as_str()))?;
                }
            };
        },
        _ => {},
    };

    Ok(())
}
