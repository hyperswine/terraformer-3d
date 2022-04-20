use winit::{
    event_loop::{EventLoop},
    window::Window,
};

// creates a window context and event loop. Needs a pollster and a run() function to actually render the window
pub fn load_window() -> (Window, EventLoop<()>) {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();

    (window, event_loop)
}
