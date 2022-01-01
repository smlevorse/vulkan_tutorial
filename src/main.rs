use anyhow::Result;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

fn main() -> Result<()> {
    pretty_env_logger::init();

    // Window

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Vulkan Tutorial (Rust)")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop);

    // App

    let mut app = unsafe { App::create(&window)? };
    let mut destroying = false;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            // Render a frame
            Event::MainEventsCleared if !destroying =>
                unsafe { app.render(&window) }.unwrap(),
            // Destroy the Vulkan App
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                destroying = true;
                *control_flow = ControlFlow::Exit;
                unsafe { app.destroy(); }
            },
            _ => {}
        }
    });
}

#[derive(Clone, Debug)]
struct App {}

impl App {
    // Create the Vulkan App
    unsafe fn create(window: &Window) -> Result<Self> {
        Ok(Self {})
    }

    // Renders a frame for our Vulkan App
    unsafe fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    // Destroys the Vulkan App
    unsafe fn destroy(&mut self) {}
}

#[derive(Clone, Debug, Default)]
struct AppData {}
