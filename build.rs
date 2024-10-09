use std::process;

use serde::Deserialize;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

fn get_latest_kms_release(client: &reqwest::blocking::Client) -> Option<String> {
    std::env::var("VERSION")
        .ok()
        .filter(|version| !version.is_empty())
        .or_else(|| {
            client
                .get("https://api.github.com/repos/Cosmian/kms/releases/latest")
                .send()
                .ok()
                .and_then(|response| response.json::<Release>().ok())
                .and_then(|release| {
                    if release.tag_name.is_empty() {
                        None
                    } else {
                        Some(release.tag_name)
                    }
                })
                .or_else(|| option_env!("CARGO_PKG_VERSION").map(|version| version.to_string()))
        })
}

fn main() {
    let client = reqwest::blocking::Client::builder()
        .user_agent("Cosmian/CKMS_GUI")
        .build()
        .unwrap();

    // typically the `VERSION` env var is set by the KMS CI release pipeline
    let latest_kms_release = get_latest_kms_release(&client);

    let latest_kms_release = latest_kms_release.unwrap_or_else(|| {
        println!("Latest KMS release version cannot be empty");
        process::exit(1);
    });

    println!("cargo:warning=Using KMS release {latest_kms_release}");

    // get `ckms` `main.rs` file
    let content = client
        .get(format!(
        "https://raw.githubusercontent.com/Cosmian/kms/{latest_kms_release}/crate/cli/src/main.rs"
    ))
        .send()
        .unwrap()
        .text()
        .unwrap();

    // install Klask hook
    let content = content.replace(
        "async fn main_() -> CliResult<()> {\n",
        "async fn main_() -> CliResult<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        let cmd = <Cli as CommandFactory>::command().name(\"Cosmian KMS\");
        klask::run_app(cmd, klask::Settings::default(), |_| {});
        return Ok(())
    }\n\n",
    );

    std::fs::write("./src/main.rs", content.as_bytes()).unwrap();
}
