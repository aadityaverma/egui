#!/usr/bin/env python3
"""Script to fix the PyScript Shell implementation in egui"""

# Read the original file
with open('/Users/aditya/wf/egui/crates/egui_demo_lib/src/demo/pyscript_shell.rs', 'r') as f:
    content = f.read()

# Replace the output update code for the install package function
# This is after "exec('''{}''') for install_code" 
old_install_code = '''# Get the output
output = captured_output.getvalue()
sys.stdout = old_stdout

# Update the output element in the HTML
from js import document
output_element = document.getElementById("python-output")
if output_element:
    output_element.innerText = output
"#, install_code.replace('\\'', "\\\\'" ));'''

new_install_code = '''# Get the output
output = captured_output.getvalue()
sys.stdout = old_stdout

# Update the Rust state via the exported function
from js import wasm_bindgen
try:
    wasm_bindgen.update_output(output if output else "Package installation initiated.")
except Exception as e:
    print(f"Error updating output: {e}")
"#, install_code.replace('\\'', "\\\\'" ));'''

content = content.replace(old_install_code, new_install_code)

# Replace the output update code for the run code function
old_run_code = '''# Get the output
output = captured_output.getvalue()
sys.stdout = old_stdout

# Update the output element in the HTML
from js import document
output_element = document.getElementById("python-output")
if output_element:
    output_element.innerText = output
"#, self.code.replace('\\'', "\\\\'" ));'''

new_run_code = '''# Get the output
output = captured_output.getvalue()
sys.stdout = old_stdout

# Update the Rust state via the exported function
from js import wasm_bindgen
try:
    wasm_bindgen.update_output(output if output else "Code executed successfully.")
except Exception as e:
    print(f"Error updating output: {e}")
"#, self.code.replace('\\'', "\\\\'" ));'''

content = content.replace(old_run_code, new_run_code)

# Write the updated file
with open('/Users/aditya/wf/egui/crates/egui_demo_lib/src/demo/pyscript_shell.rs', 'w') as f:
    f.write(content)

print("PyScript Shell fixes applied successfully!")