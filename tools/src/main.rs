mod fs;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    fs::pack_all_apps()
}
