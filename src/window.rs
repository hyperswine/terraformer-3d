use winit::{
    event_loop::{EventLoop},
    window::Window,
};

pub fn load_window() -> (Window, EventLoop<()>) {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();

    (window, event_loop)
}
