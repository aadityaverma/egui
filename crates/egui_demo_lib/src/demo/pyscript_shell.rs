//! A Python shell widget integrated into egui that uses PyScript
//!
//! This demonstrates how to create an interactive widget that allows Python code execution
//! within an egui application. The PyScript integration runs Python in the browser.

use egui::{ScrollArea, TextEdit, TextStyle, Ui};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref PY_SCRIPT_SHELL: Mutex<PyScriptShell> = Mutex::new(PyScriptShell::default());
}

#[wasm_bindgen]
pub fn update_output(output: &str) {
    if let Ok(mut shell) = PY_SCRIPT_SHELL.lock() {
        shell.output = output.to_string();
    }
}

#[derive(Debug)]
pub struct PyScriptShell {
    code: String,
    output: String,
}

impl Default for PyScriptShell {
    fn default() -> Self {
        Self {
            code: "print('Hello from Python in egui!')\n# Try clicking the example buttons below".to_string(),
            output: "Python Shell Ready...".to_string(),
        }
    }
}

impl PyScriptShell {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("🐍 Python Shell");
            ui.separator();
            if ui.button("Clear").clicked() {
                self.output.clear();
            }
        });

        ui.separator();

        // Code editor
        ui.label("Python Code:");
        ui.add(
            TextEdit::multiline(&mut self.code)
                .font(TextStyle::Monospace)
                .code_editor()
                .desired_rows(10)
                .desired_width(f32::INFINITY),
        );

        ui.separator();

        // Run button
        ui.horizontal(|ui| {
            if ui.button("▶ Run Python Code").clicked() {
                self.run_code();
            }
            
            if ui.button("📦 Install Package").clicked() {
                if let Some(window) = web_sys::window() {
                    // Try using Pyodide if available
                    if let Ok(pyodide) = js_sys::Reflect::get(&window, &"pyodide".into()) {
                        if pyodide.is_object() {
                            // Try to run the Python package installation code directly with pyodide
                            if let Ok(run_python) = js_sys::Reflect::get(&pyodide, &"runPython".into()) {
                                if let Some(run_python_fn) = run_python.dyn_ref::<js_sys::Function>() {
                                    let install_code = "import micropip\nawait micropip.install(\"requests\")\nprint(\"Package installed!\")";
                                    
                                    // Execute the code with a try/catch to capture output
                                    let code_with_capture = format!(r#"
import sys
from io import StringIO

# Capture stdout
old_stdout = sys.stdout
captured_output = StringIO()
sys.stdout = captured_output

try:
    exec('''{}''')
except Exception as e:
    sys.stdout = old_stdout
    print(f"Error: {{e}}")
    import traceback
    traceback.print_exc()

# Get the output
output = captured_output.getvalue()
sys.stdout = old_stdout

# Update the output element in the HTML
from js import document
output_element = document.getElementById("python-output")
if output_element:
    output_element.innerText = output
"#, install_code.replace("'", "\\'"));
                                    
                                    let result = run_python_fn.call1(&pyodide, &code_with_capture.into());
                                    if let Ok(promise) = result {
                                        let future = JsFuture::from(promise.dyn_into::<js_sys::Promise>().unwrap());
                                        wasm_bindgen_futures::spawn_local(async move {
                                            if let Err(e) = future.await {
                                                log::error!("Failed to install package with pyodide: {:?}", e);
                                            }
                                        });
                                    } else {
                                        log::error!("Failed to call runPython");
                                    }
                                } else {
                                    log::error!("pyodide.runPython is not a function");
                                }
                            } else {
                                log::error!("pyodide.runPython not available");
                            }
                        } else {
                            log::error!("pyodide is not an object");
                        }
                    } else {
                        // If pyodide is not available, just write the code to the textarea
                        log::warn!("Pyodide not available, setting code in textarea...");
                        if let Some(document) = window.document() {
                            if let Some(code_input) = document.get_element_by_id("python-code-input") {
                                if let Ok(code_input_element) = code_input.dyn_into::<web_sys::HtmlTextAreaElement>() {
                                    code_input_element.set_value("import micropip\nawait micropip.install(\"requests\")\nprint(\"Package installed!\")");
                                }
                            }
                        }
                    }
                } else {
                    log::error!("no global `window` exists");
                }
            }
            
            ui.separator();
            ui.label("💡 Tip: Use 'from js import document' to access web APIs");
        });
        
        ui.separator();
        
        // Example buttons to help with testing
        ui.horizontal(|ui| {
            if ui.button("📝 Hello World").clicked() {
                self.code = "print('Hello from Python in egui!')".to_string();
                self.run_code();
            }
            
            if ui.button("📊 Math Example").clicked() {
                self.code = r#"import math

# Calculate and print squares
numbers = [1, 2, 3, 4, 5]
squares = [x**2 for x in numbers]

print(f"Numbers: {numbers}")
print(f"Squares: {squares}")

# Calculate square roots
sqrt_numbers = [math.sqrt(x) for x in squares]
print(f"Square roots: {sqrt_numbers}")

# Pi and e
print(f"Pi is approximately {math.pi:.4f}")
print(f"E is approximately {math.e:.4f}")
"#.to_string();
                self.run_code();
            }
            
            if ui.button("🌐 DOM Access").clicked() {
                self.code = r#"from js import document

# Access the page title
title = document.title
print(f"Page title: {title}")

# Count elements
elements = document.querySelectorAll("*")
print(f"Total elements on page: {len(elements)}")

# Find the canvas
canvas = document.getElementById("the_canvas_id")
if canvas:
    print(f"Found canvas with id 'the_canvas_id'")
    print(f"Canvas width: {canvas.width}")
    print(f"Canvas height: {canvas.height}")
else:
    print("Canvas not found")
"#.to_string();
                self.run_code();
            }
        });

        ui.separator();

        // Output area
        ui.label("Output:");
        ScrollArea::vertical()
            .id_salt("pyscript_output")
            .max_height(200.0)
            .stick_to_bottom(true)
            .show(ui, |ui| {
                // Try to get current output from the HTML element if available
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Some(output_elem) = document.get_element_by_id("python-output") {
                            // Update our local copy of the output if different
                            let html_output = output_elem.text_content().unwrap_or_default();
                            if self.output != html_output {
                                self.output = html_output;
                            }
                        }
                    }
                }
                
                ui.add(
                    TextEdit::multiline(&mut self.output)
                        .font(TextStyle::Monospace)
                        .code_editor()
                        .desired_rows(10)
                        .desired_width(f32::INFINITY)
                        .interactive(false),
                );
            });
    }

    fn run_code(&mut self) {
        if let Some(window) = web_sys::window() {
            // Set the code in the HTML textarea that PyScript will execute
            if let Some(document) = window.document() {
                if let Some(code_input) = document.get_element_by_id("python-code-input") {
                    if let Ok(code_input_element) = code_input.dyn_into::<web_sys::HtmlTextAreaElement>() {
                        code_input_element.set_value(&self.code);
                        
                        // The pyodide object may not be immediately available, so we'll check if it's loaded
                        // and if so, execute directly, otherwise try the HTML button approach
                        if let Ok(pyodide) = js_sys::Reflect::get(&window, &"pyodide".into()) {
                            if pyodide.is_object() {
                                // Try to run the Python code directly with pyodide
                                if let Ok(run_python) = js_sys::Reflect::get(&pyodide, &"runPython".into()) {
                                    if let Some(run_python_fn) = run_python.dyn_ref::<js_sys::Function>() {
                                        // Execute the code with a try/catch to capture output
                                        let code_with_capture = format!(r#"
import sys
from io import StringIO

# Capture stdout
old_stdout = sys.stdout
captured_output = StringIO()
sys.stdout = captured_output

try:
    exec('''{}''')
except Exception as e:
    sys.stdout = old_stdout
    print(f"Error: {{e}}")
    import traceback
    traceback.print_exc()

# Get the output
output = captured_output.getvalue()
sys.stdout = old_stdout

# Update the output element in the HTML
from js import document
output_element = document.getElementById("python-output")
if output_element:
    output_element.innerText = output
"#, self.code.replace("'", "\\'"));
                                        
                                        let result = run_python_fn.call1(&pyodide, &code_with_capture.into());
                                        if let Ok(promise) = result {
                                            let future = JsFuture::from(promise.dyn_into::<js_sys::Promise>().unwrap());
                                            wasm_bindgen_futures::spawn_local(async move {
                                                if let Err(e) = future.await {
                                                    log::error!("Failed to run Python code with pyodide: {:?}", e);
                                                }
                                            });
                                        } else {
                                            log::error!("Failed to call runPython");
                                        }
                                    } else {
                                        log::error!("pyodide.runPython is not a function");
                                    }
                                } else {
                                    log::error!("pyodide.runPython not available");
                                }
                            } else {
                                log::warn!("pyodide is not an object - PyScript runtime may not be ready. The runtime may still be loading. Try again in a few seconds.");
                                if let Some(output_elem) = document.get_element_by_id("python-output") {
                                    output_elem.set_inner_html("PyScript runtime loading... Please wait a few seconds and try again.");
                                }
                            }
                        } else {
                            // Pyodide is not available, which could mean it hasn't loaded yet
                            // or PyScript is not properly configured. 
                            log::warn!("Pyodide not available, trying alternative methods...");
                            
                            // Check if the HTML run_python_code function is available (from the py-script block)
                            if let Ok(run_python_fn) = js_sys::Reflect::get(&window, &"run_python_code".into()) {
                                if let Some(func) = run_python_fn.dyn_ref::<js_sys::Function>() {
                                    if let Err(e) = func.call0(&window) {
                                        log::error!("Failed to call run_python_code: {:?}", e);
                                    }
                                } else {
                                    log::error!("run_python_code is not a function, trying button click method...");
                                    
                                    // Try clicking the run button which has py-click="run_python_code" attribute
                                    // Find any button with py-click="run_python_code" attribute
                                    let buttons = document.query_selector_all("[py-click='run_python_code']").unwrap();
                                    if buttons.length() > 0 {
                                        if let Some(button) = buttons.get(0) {
                                            if let Ok(click_fn) = js_sys::Reflect::get(&button, &"click".into()) {
                                                if let Some(click_func) = click_fn.dyn_ref::<js_sys::Function>() {
                                                    if let Err(e) = click_func.call0(&button) {
                                                        log::error!("Failed to click run button: {:?}", e);
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        log::error!("No run button found with py-click='run_python_code'");
                                        if let Some(output_elem) = document.get_element_by_id("python-output") {
                                            output_elem.set_inner_html("Could not find PyScript execution button. The runtime may not be initialized properly.");
                                        }
                                    }
                                }
                            } else {
                                log::error!("run_python_code function not available");
                                
                                // As a last resort, try to directly manipulate HTML elements and trigger any available PyScript execution
                                if let Some(output_elem) = document.get_element_by_id("python-output") {
                                    output_elem.set_inner_html("PyScript/Pyodide runtime not available. Check console for errors.");
                                }
                            }
                        }
                    } else {
                        log::error!("python-code-input is not a textarea");
                    }
                } else {
                    log::error!("python-code-input element not found");
                }
            } else {
                log::error!("document not found on window");
            }
        } else {
            log::error!("no global `window` exists");
        }
    }
}

impl crate::Demo for PyScriptShell {
    fn name(&self) -> &'static str {
        "🐍 PyScript Shell"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        let mut shell = PY_SCRIPT_SHELL.lock().unwrap();
        egui::Window::new(shell.name())
            .open(open)
            .min_width(600.0)
            .default_height(500.0)
            .show(ctx, |ui| shell.ui(ui));
    }
}