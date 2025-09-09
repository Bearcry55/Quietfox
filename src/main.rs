use std::fs;
use std::path::PathBuf;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Icon},
};
use wry::webview::WebViewBuilder;

fn load_icon(path: &str) -> Result<Icon, Box<dyn std::error::Error>> {
    let image_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path);
    let image_data = fs::read(&image_path)?;
    
    // Use image crate to decode the image
    let image = image::load_from_memory(&image_data)?;
    let rgba_image = image.to_rgba8();
    let (width, height) = rgba_image.dimensions();
    
    let icon = Icon::from_rgba(rgba_image.into_raw(), width, height)?;
    Ok(icon)
}

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    
    // Create window builder
    let mut window_builder = WindowBuilder::new()
        .with_title("Quietfox Browser")
        .with_inner_size(tao::dpi::LogicalSize::new(900.0, 700.0));
    
    // Try to load and set the icon
    if let Ok(icon) = load_icon("Quitefox.png") {  // Change "logo.png" to your icon file name
        window_builder = window_builder.with_window_icon(Some(icon));
    } else {
        eprintln!("Warning: Could not load icon file");
    }
    
    let window = window_builder.build(&event_loop)?;

    // Load local HTML file (homepage.html)
    let mut homepage_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    homepage_path.push("homepage.html");
    if !homepage_path.exists() {
        eprintln!("Error: homepage.html not found at {:?}", homepage_path);
        std::process::exit(1);
    }

    let url = url::Url::from_file_path(&homepage_path)
        .expect("Failed to construct file:// URL")
        .to_string();

    // Load init.js script
    let mut js_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    js_path.push("init.js");
    let init_js = fs::read_to_string(&js_path)
        .expect("Failed to read init.js from project root");

    // Random User Agent for privacy (anti-fingerprinting)
    let user_agents = [
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/119.0",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Safari/605.1.15",
    ];
    let random_ua = user_agents[fastrand::usize(..user_agents.len())];

    let _webview = WebViewBuilder::new(window)?
        .with_url(&url)?
        .with_user_agent(random_ua)
        .with_initialization_script(&init_js)
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent { event, .. } = event {
            if let WindowEvent::CloseRequested = event {
                *control_flow = ControlFlow::Exit;
            }
        }
    });
}