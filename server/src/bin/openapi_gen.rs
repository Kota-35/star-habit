use std::path::PathBuf;
use utoipa::OpenApi;

fn main() {
    let path: PathBuf = std::env::var("OPENAPI_OUTPUT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            // server/ から見てルートの packages/openapi/star-habbit.json
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../packages/openapi/star-habit.json")
        });

    if let Some(parent) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            eprintln!("Failed to create directory {:?}: {}", parent, e);
            std::process::exit(1);
        }
    }

    let spec = server::routes::openapi::ApiDoc::openapi();
    let json =
        serde_json::to_string_pretty(&spec).expect("OpenAPI to JSON");

    if let Err(e) = std::fs::write(&path, json) {
        eprintln!("Failed to write {:?}: {}", path, e);
        std::process::exit(1);
    }

    println!("OpenAPI schema written to {}", path.display());
}
