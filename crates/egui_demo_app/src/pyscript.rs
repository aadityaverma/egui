//! PyScript integration for egui web apps
use egui::{Button, RichText, Ui};
use wasm_bindgen::prelude::*;

/// A struct to manage communication with PyScript
pub struct PyScriptManager {
    /// Flag to check if PyScript is loaded
    pyscript_loaded: bool,
    /// Store Python code to execute
    python_code: String,
    /// Output from Python execution
    python_output: String,
    /// Package to install
    package_to_install: String,
}

impl PyScriptManager {
    pub fn new() -> Self {
        Self {
            pyscript_loaded: false,
            python_code: "print('Hello from Rust calling Python!')\n# You can execute any Python code here".to_string(),
            python_output: "Ready to execute Python code...".to_string(),
            package_to_install: String::new(),
        }
    }

    /// Check if PyScript is loaded
    pub fn check_pyscript_loaded(&mut self) {
        if !self.pyscript_loaded {
            // Check if PyScript is available in the browser
            let _window = web_sys::window().expect("no global `window` exists");
            let document = _window.document().expect("should have a document on window");
            
            // Check for PyScript elements in the DOM
            let element = document.get_element_by_id("python-output");
            if element.is_some() {
                self.pyscript_loaded = true;
                self.python_output = "PyScript is loaded and ready!".to_string();
            }
        }
    }

    /// Execute Python code via PyScript
    pub fn run_python_code(&mut self) {
        if !self.pyscript_loaded {
            self.python_output = "PyScript is not loaded yet.".to_string();
            return;
        }

        // Update the Python code input in the DOM
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        
        if let Some(input) = document.get_element_by_id("python-code-input") {
            let input = input.dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
            input.set_value(&self.python_code);
        }

        // Try to execute the PyScript function directly
        match self.execute_pyscript_function("run_python_code") {
            Ok(_) => {
                self.python_output = "Python code execution initiated...".to_string();
            }
            Err(e) => {
                self.python_output = format!("Error executing Python code: {:?}", e);
            }
        }
    }

    /// Install a Python package using micropip
    pub fn install_package(&mut self) {
        if !self.pyscript_loaded {
            self.python_output = "PyScript is not loaded yet.".to_string();
            return;
        }

        // Update the Python code to install the package
        let install_code = format!(
            "import micropip\nawait micropip.install('{}')\nprint('Package {} installed successfully!')",
            self.package_to_install, self.package_to_install
        );

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        
        if let Some(input) = document.get_element_by_id("python-code-input") {
            let input = input.dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
            input.set_value(&install_code);
        }

        match self.execute_pyscript_function("run_python_code") {
            Ok(_) => {
                self.python_output = format!("Installing package: {}...", self.package_to_install);
            }
            Err(e) => {
                self.python_output = format!("Error installing package: {:?}", e);
            }
        }
    }

    /// Execute a PyScript function directly
    fn execute_pyscript_function(&self, func_name: &str) -> Result<(), JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        
        // Try to call the PyScript function using eval
        let eval_code = format!(
            "(function() {{ try {{ if(typeof {} === 'function') {{ {}(); return true; }} else {{ console.error('{} is not a function'); return false; }} }} catch(e) {{ console.error(e); return false; }} }})()",
            func_name, func_name, func_name
        );
        
        let result = js_sys::eval(&eval_code)?;
        let result_bool = result.as_bool().unwrap_or(false);
        
        if result_bool {
            Ok(())
        } else {
            Err(JsValue::from_str("Function execution failed"))
        }
    }

    /// Set the Python code to execute
    pub fn set_python_code(&mut self, code: &str) {
        self.python_code = code.to_string();
    }
}

/// UI for PyScript integration
pub struct PyScriptUi {
    manager: PyScriptManager,
}

impl PyScriptUi {
    pub fn new() -> Self {
        Self {
            manager: PyScriptManager::new(),
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        self.manager.check_pyscript_loaded();

        ui.heading("🐍 PyScript Integration");
        
        ui.label(RichText::new("Execute Python code directly from egui!").color(ui.visuals().text_color()));
        
        ui.separator();

        // Python code editor
        ui.horizontal(|ui| {
            ui.label("Python Code:");
            if ui.button("Clear").clicked() {
                self.manager.set_python_code("");
            }
        });
        
        let _code_response = ui.add(
            egui::TextEdit::multiline(&mut self.manager.python_code)
                .code_editor()
                .desired_rows(10)
                .desired_width(f32::INFINITY)
        );
        
        // Run button
        ui.horizontal(|ui| {
            if ui.add(Button::new(RichText::new("▶ Run Python Code").strong())).clicked() {
                self.manager.run_python_code();
            }
            
            ui.label("Install Package:");
            ui.text_edit_singleline(&mut self.manager.package_to_install);
            if ui.button("📦 Install").clicked() && !self.manager.package_to_install.is_empty() {
                self.manager.install_package();
            }
        });

        ui.separator();

        // Output panel
        ui.label("Python Output:");
        ui.add(
            egui::TextEdit::multiline(&mut self.manager.python_output)
                .desired_rows(5)
                .desired_width(f32::INFINITY)
                .lock_focus(true)
                .interactive(false)  // Make it read-only
        );
        
        // Inform user about PyScript status
        if !self.manager.pyscript_loaded {
            ui.colored_label(ui.visuals().warn_fg_color, "PyScript is still loading...");
        }
    }
}