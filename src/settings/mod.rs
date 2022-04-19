// ----------
// SETTINGS
// ----------

// Way to represent modifyable system state mostly to do with UI and meta graphics settings
// A single GlobalSetting should modify the system in some way that isnt trivial
struct GlobalSetting {}

impl GlobalSetting {
    fn new() -> Self {
        Self {}
    }
    // Set settings to the corresponding yml file
    fn set_settings(&mut self) {}

    // Read settings from the corresponding yml file
    fn get_settings(&mut self) {}
}
