use window::WindowManager;

fn main() -> Result<(), String> {
    println!("Starting wallpaper manager...");

    let mut manager = WindowManager::new();
    manager.init()?;
    manager.run_loop()?;

    Ok(())
}
