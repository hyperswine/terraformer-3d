pub struct Shader {
    pub vs: String,
    pub fs: String,
    pub gs: String,
}

impl Shader {
    pub fn new(&self, v_path: &str, f_path: &str, g_path: &str) -> Shader {
        // load each
        let v = self.load_shader(v_path);
        let f = self.load_shader(f_path);
        let g = self.load_shader(g_path);

        Shader {
            vs: v,
            fs: f,
            gs: g,
        }
    }

    // load the shader data into a string (ascii)
    fn load_shader(&self, path: &str) -> String {
        "".to_string()
    }

    // apply the shader, to the swapchain
    pub fn apply(&self) {}
}

pub mod state;
