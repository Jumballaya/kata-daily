use std::{ fs, io::Write };


pub struct Typescript {
    pub pwd: String,
    pub day: i32,
    pub implementation_files: ImplementationFiles,
    pub test_files: TestFiles,
    pub build_files: BuildFiles,
}

pub struct BuildFiles {
    pub package_json_file: String,
    pub tsconfig_json_file: String,
}

impl BuildFiles {
    pub fn new() -> Self {
        let package_json_file = String::from(include_str!("template/package.json"));
        let tsconfig_json_file = String::from(include_str!("template/tsconfig.json"));
        return Self { package_json_file, tsconfig_json_file };
    }

    pub fn generate(&self, pwd: &str) {
        println!("Generating build files");
        println!("{}/package.json", pwd);
        let mut package_json_file = fs::File::create(format!("{}/package.json", pwd))
            .expect("Unable to create package.json file");
        package_json_file.write(self.package_json_file.as_bytes())
            .expect("Unable to write to package.json file");

        println!("{}/tsconfig.json", pwd);
        let mut tsconfig_json_file = fs::File::create(format!("{}/tsconfig.json", pwd))
            .expect("Unable to create tsconfig.json file");
        tsconfig_json_file.write(self.tsconfig_json_file.as_bytes())
            .expect("Unable to write to tsconfig.json file");
    }
}

pub struct ImplementationFiles {
    pub stack_file: String,
    pub queue_file: String,
}

impl ImplementationFiles {
    pub fn new() -> Self {
        let stack_file = String::from(include_str!("template/src/Stack.ts"));
        let queue_file = String::from(include_str!("template/src/Queue.ts"));
        return Self { stack_file, queue_file };
    }

    pub fn generate(&self, pwd: &str) {
        println!("Generating src folder");
        println!("{}", format!("{}/src", pwd));
        fs::create_dir(format!("{}/src", pwd))
            .expect("Unable to create test folder");
        self.generate_day(pwd, 1);
    }

    pub fn generate_day(&self, pwd: &str, day: i32) {
        println!("{}", format!("Generating folder: day {}", day));
        println!("{}", format!("{}/src/day{}", pwd, day));
        fs::create_dir(format!("{}/src/day{}", pwd, day))
            .expect("Unable to create day folder");

        println!("Generating Stack implementation file");
        println!("{}", format!("{}/src/day{}/Stack.ts", pwd, day));
        let mut stack_file = fs::File::create(format!("{}/src/day{}/Stack.ts", pwd, day))
            .expect("Unable to create Stack implementation file");
        stack_file.write(self.stack_file.as_bytes())
            .expect("Unable to write to Stack implementation file");

        println!("Generating Queue implementation file");
        println!("{}", format!("{}/src/day{}/Queue.ts", pwd, day));
        let mut queue_file = fs::File::create(format!("{}/src/day{}/Queue.ts", pwd, day))
            .expect("Unable to create Queue implementation file");
        queue_file.write(self.queue_file.as_bytes())
            .expect("Unable to write to Stack implementation file");
    }
}

pub struct TestFiles {
    pub stack_test_file: String,
    pub queue_test_file: String,
    pub test_runner_file: String,
}

impl TestFiles {
    pub fn new() -> Self {
        let stack_test_file = String::from(include_str!("template/src/tests/Stack.test.ts"));
        let queue_test_file = String::from(include_str!("template/src/tests/Queue.test.ts"));
        let test_runner_file = String::from(include_str!("template/src/test-runner.ts"));
        return Self { stack_test_file, queue_test_file, test_runner_file };
    }

    pub fn generate(&self, pwd: &str) {
        println!("Generating Test runner file");
        println!("{}", format!("{}/src/test-runner.ts", pwd));
        let mut stack_test_file = fs::File::create(format!("{}/src/test-runner.ts", pwd))
            .expect("Unable to create test runner file");
        stack_test_file.write(self.test_runner_file.as_bytes())
            .expect("Unable to write to the test runner file");
        self.generate_day(pwd, 1);
    }

    pub fn generate_day(&self, pwd: &str, day: i32) {
        println!("Generating test folder");
        println!("{}", format!("{}/src/day{}/tests", pwd, day));
        fs::create_dir(format!("{}/src/day{}/tests", pwd, day))
            .expect("Unable to create test folder");

        println!("Generating Stack test files");
        println!("{}", format!("{}/src/day{}/tests/Stack.test.ts", pwd, day));
        let mut stack_test_file = fs::File::create(format!("{}/src/day{}/tests/Stack.test.ts", pwd, day))
            .expect("Unable to create Stack test file");
        stack_test_file.write(self.stack_test_file.as_bytes())
            .expect("Unable to write to the Stack test file");

        println!("Generating Queue test files");
        println!("{}", format!("{}/src/day{}/tests/Queue.test.ts", pwd, day));
        let mut queue_test_file = fs::File::create(format!("{}/src/day{}/tests/Queue.test.ts", pwd, day))
            .expect("Unable to create Queue test file");
        queue_test_file.write(self.queue_test_file.as_bytes())
            .expect("Unable to write to the Queue test file");
    }
}

impl Typescript {
    pub fn new(day: i32, pwd: String) -> Self {
        return Typescript {
            implementation_files: ImplementationFiles::new(),
            test_files: TestFiles::new(),
            build_files: BuildFiles::new(),
            day,
            pwd,
        };
    }

    pub fn generate(&self) {
        self.build_files.generate(self.pwd.as_str());
        self.implementation_files.generate(self.pwd.as_str());
        self.test_files.generate(self.pwd.as_str());
        let output = std::process::Command::new("npm")
            .arg("install")
            .output()
            .expect("Unable to install npm dependencies");
        println!("{}", String::from_utf8(output.stdout)
            .expect("Unable to install npm dependencies"));
    }

    pub fn generate_day(&self, day: i32) {
        self.implementation_files.generate_day(self.pwd.as_str(), day);
        self.test_files.generate_day(self.pwd.as_str(), day);
    }

    pub fn run_test(&self, day: i32) {
        let output = std::process::Command::new("npm")
            .arg("test")
            .env("KATA_DAY", day.to_string())
            .output()
            .expect("Unable to run TypeScript tests");

        println!("{}", String::from_utf8(output.stdout)
            .expect("Unable to run TypeScript tests"));
        println!("{}", String::from_utf8(output.stderr)
            .expect("Unable to run TypeScript tests"));
    }
}
