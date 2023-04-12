use std::{ fs, io::{Write, self}};
use include_dir;


static PROJECT_DIR: include_dir::Dir<'_> = include_dir::include_dir!("$CARGO_MANIFEST_DIR/src/languages/typescript/template");


pub struct Typescript {
    pub pwd: String,
}

impl Typescript {
    pub fn new(pwd: String) -> Self {
        return Typescript {
            pwd,
        };
    }

    pub fn generate(&self) -> Result<(), io::Error> {
        fs::create_dir(format!("{}/typescript", self.pwd))?;
        for entry in PROJECT_DIR.entries() {
            match entry.as_file() {
                Some(file) => {
                    let name = file.path().display().to_string();
                    let contents = file.contents_utf8()
                        .expect("Unable to read template contents");
                    let mut new_file = fs::File::create(format!("typescript/{}", name))?;
                    new_file.write(contents.as_bytes())?;
                },
                _ => (),
            };
        };

        fs::create_dir(format!("{}/typescript/src", self.pwd))?;
        if let Some(dir) = PROJECT_DIR.get_dir("src/") {
            for entry in dir.entries() {
                match entry.as_file() {
                    Some(file) => {
                        let path = file.path().display().to_string();
                        let (_, name) = path.rsplit_once("/").unwrap_or(("", "error.ts"));
                        let contents = file.contents_utf8()
                            .expect("Unable to read template contents");
                        let mut new_file = fs::File::create(format!("typescript/src/{}", name))?;
                        new_file.write(contents.as_bytes())?;
                    },
                    _ => (),
                };
            };
        };

        fs::create_dir(format!("{}/typescript/src/tests", self.pwd))?;
        if let Some(dir) = PROJECT_DIR.get_dir("src/tests/") {
            for entry in dir.entries() {
                match entry.as_file() {
                    Some(file) => {
                        let path = file.path().display().to_string();
                        let (_, name) = path.rsplit_once("/").unwrap_or(("", "error.ts"));
                        let contents = file.contents_utf8()
                            .expect("Unable to read template contents");
                        let mut new_file = fs::File::create(format!("typescript/src/tests/{}", name))?;
                        new_file.write(contents.as_bytes())?;
                    },
                    _ => (),
                };
            };
        };

        self.generate_day(1)?;

        let output = std::process::Command::new("npm")
            .current_dir(format!("{}/typescript", self.pwd))
            .arg("install")
            .output()
            .expect("Unable to install npm dependencies");
        println!("{}", String::from_utf8(output.stdout)
            .expect("Unable to install npm dependencies"));

        Ok(())
    }

    pub fn generate_day(&self, day: i32) -> Result<(), io::Error> {
        fs::create_dir(format!("{}/typescript/src/day{}", self.pwd, day))?;
        if let Some(dir) = PROJECT_DIR.get_dir("src/day/") {
            for entry in dir.entries() {
                match entry.as_file() {
                    Some(file) => {
                        let path = file.path().display().to_string();
                        let (_, name) = path.rsplit_once("/").unwrap_or(("", "error.ts"));
                        let contents = file.contents_utf8()
                            .expect("Unable to read template contents");
                        let mut new_file = fs::File::create(format!("typescript/src/day{}/{}", day, name))?;
                        new_file.write(contents.as_bytes())?;
                    },
                    _ => (),
                };
            };
        };

        Ok(())
    }

    pub fn run_test(&self, day: i32) {
        let output = std::process::Command::new("npm")
            .arg("test")
            .current_dir(format!("{}/typescript", self.pwd))
            .env("KATA_DAY", day.to_string())
            .output()
            .expect("Unable to run TypeScript tests");

        println!("{}", String::from_utf8(output.stdout)
            .expect("Unable to run TypeScript tests"));
        println!("{}", String::from_utf8(output.stderr)
            .expect("Unable to run TypeScript tests"));
    }
}
