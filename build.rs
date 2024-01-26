use serde::Deserialize;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

fn main() {
    let client = reqwest::blocking::Client::builder()
        .user_agent("Cosmian/KMS_GUI")
        .build()
        .unwrap();

    let latest_kms_release = client
        .get("https://api.github.com/repos/Cosmian/kms/releases/latest")
        .send()
        .unwrap()
        .json::<Release>()
        .unwrap()
        .tag_name;

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
        "async fn main_() -> Result<(), CliError> {\n",
        "async fn main_() -> Result<(), CliError> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        let cmd = <Cli as CommandFactory>::command().name(\"Cosmian KMS\");
        klask::run_app(cmd, klask::Settings::default(), |_| {});
        return Ok(())
    }\n\n",
    );

    std::fs::write("./src/main.rs", content.as_bytes()).unwrap();
}
