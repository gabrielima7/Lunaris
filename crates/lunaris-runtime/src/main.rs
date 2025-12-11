//! Lunaris Engine Runtime Executable

fn main() {
    if let Err(e) = lunaris_runtime::init() {
        eprintln!("Failed to initialize Lunaris: {e}");
        std::process::exit(1);
    }

    println!("Lunaris Engine v{}", lunaris_core::VERSION);
}
