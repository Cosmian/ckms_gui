fn main() {
    // get `ckms` `main.rs` file
    let response = reqwest::blocking::get(
        "https://raw.githubusercontent.com/Cosmian/kms/4.11.0/crate/cli/src/main.rs",
    )
    .unwrap();

    let content = response.text().unwrap();

    // println!("cargo:warning={content}");

    // install Klask hook
    let content = content.replace(
        "async fn main_() -> Result<(), CliError> {\n",
        "async fn main_() -> Result<(), CliError> {\n
            let args = std::env::args().collect::<Vec<_>>();\n
            if args.len() < 2 {\n
                let cmd = <Cli as CommandFactory>::command();\n
                klask::run_app(cmd, klask::Settings::default(), |_| {});\n
                return Ok(())\n
            }\n",
    );

    std::fs::write("./src/main.rs", content.as_bytes()).unwrap();
}
