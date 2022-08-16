use winres::WindowsResource;

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() != "windows" {
        return;
    }

    if std::env::var("PROFILE").unwrap() == "release" {
        let mut res = WindowsResource::new();
        res.set_icon("icon.ico");
        if let Err(e) = res.compile() {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
