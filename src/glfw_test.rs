extern crate native;
extern crate glfw = "glfw-rs";
extern crate gl;

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
    glfw::set_error_callback(~ErrorContext);

    glfw::start(proc() {
        let window = glfw::Window::create(800, 600, "Hello this is window", glfw::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.make_context_current();

        while !window.should_close() {
            let (w, h) = window.get_framebuffer_size();
            let ratio = w/h;

            gl::Viewport(0, 0, w, h);
            gl::Clear(gl::COLOR_BUFFER_BIT);


            glfw::poll_events();
            for (_, event) in window.flush_events() {
                handle_window_event(&window, event);
            }

            window.swap_buffers();
        }
    });
}

struct ErrorContext;
impl glfw::ErrorCallback for ErrorContext {
    fn call(&self, _: glfw::Error, description: ~str) {
        println!("GLFW Error: {:s}", description);
    }
}

fn handle_window_event(window: &glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
