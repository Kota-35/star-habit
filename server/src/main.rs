use server::config;

fn main() {
    config::init_env().expect("failed to load config from .env");
    let env = config::env();
    println!(
        "port: {}, firebase_project_id: {}",
        env.port, env.firebase_project_id
    );
}
