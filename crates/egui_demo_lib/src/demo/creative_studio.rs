use egui::{
    color_picker::{color_picker_color32, Alpha},
    util::undoer::Undoer,
    Color32, Context, DragValue, Frame, Grid, Id, Modal, Pos2, Rect, RichText, ScrollArea, Sense,
    Slider, TextEdit, Ui, UiBuilder, Vec2, Window,
};
use egui_extras::{Size, StripBuilder};

/// A creative studio that merges multiple advanced egui features into a unified workspace
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct CreativeStudio {
    // Code editor state
    code: String,
    code_undoer: Undoer<String>,
    
    // Color palette
    primary_color: Color32,
    secondary_color: Color32,
    colors: Vec<Color32>,
    
    // Layout state
    selected_tool: Tool,
    panels: Vec<Panel>,
    
    // Drag and drop
    dragged_item: Option<usize>,
    drop_target: Option<usize>,
    
    // Scene navigation
    scene_zoom: f32,
    scene_offset: Vec2,
    
    // Modal states
    save_modal_open: bool,
    settings_modal_open: bool,
    
    // Widget gallery
    slider_value: f32,
    toggle_state: bool,
    text_input: String,
    
    // Interactive container
    click_count: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum Tool {
    Select,
    Brush,
    Text,
    Shape,
    Code,
}

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
struct Panel {
    id: usize,
    title: String,
    content: PanelContent,
    position: Pos2,
    size: Vec2,
    visible: bool,
}

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum PanelContent {
    Code(String),
    ColorPalette,
    WidgetGallery,
    SceneView,
    Properties,
}

impl Default for CreativeStudio {
    fn default() -> Self {
        Self {
            code: r#"// Welcome to the Creative Studio!
// This is a live code editor with syntax highlighting
fn creative_function() -> String {
    let colors = vec!["red", "blue", "green"];
    colors.join(", ")
}"#.to_string(),
            code_undoer: Undoer::default(),
            primary_color: Color32::from_rgb(0x66, 0x99, 0xFF),
            secondary_color: Color32::from_rgb(0xFF, 0x66, 0x99),
            colors: vec![
                Color32::RED,
                Color32::GREEN,
                Color32::BLUE,
                Color32::YELLOW,
                Color32::PURPLE,
                Color32::CYAN,
            ],
            selected_tool: Tool::Select,
            panels: vec![
                Panel {
                    id: 1,
                    title: "Scene".to_string(),
                    content: PanelContent::SceneView,
                    position: Pos2::new(200.0, 100.0),
                    size: Vec2::new(400.0, 300.0),
                    visible: true,
                },
                Panel {
                    id: 2,
                    title: "Properties".to_string(),
                    content: PanelContent::Properties,
                    position: Pos2::new(650.0, 100.0),
                    size: Vec2::new(200.0, 300.0),
                    visible: true,
                },
            ],
            dragged_item: None,
            drop_target: None,
            scene_zoom: 1.0,
            scene_offset: Vec2::ZERO,
            save_modal_open: false,
            settings_modal_open: false,
            slider_value: 50.0,
            toggle_state: true,
            text_input: "Hello Creative Studio!".to_string(),
            click_count: 0,
        }
    }
}

impl crate::Demo for CreativeStudio {
    fn name(&self) -> &'static str {
        "🎨 Creative Studio"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        Window::new(self.name())
            .open(open)
            .default_size(Vec2::new(1000.0, 700.0))
            .show(ctx, |ui| {
                use crate::View as _;
                self.ui(ui);
            });
    }
}

impl crate::View for CreativeStudio {
    fn ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            self.toolbar_ui(ui);
            ui.separator();
            
            StripBuilder::new(ui)
                .size(Size::exact(200.0))
                .size(Size::remainder())
                .size(Size::exact(200.0))
                .horizontal(|mut strip| {
                    strip.cell(|ui| {
                        self.left_panel_ui(ui);
                    });
                    strip.cell(|ui| {
                        self.main_canvas_ui(ui);
                    });
                    strip.cell(|ui| {
                        self.right_panel_ui(ui);
                    });
                });
        });

        // Handle modals
        self.save_modal_ui(ui.ctx());
        self.settings_modal_ui(ui.ctx());
    }
}

impl CreativeStudio {
    fn toolbar_ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("🎨 Creative Studio").size(18.0));
            ui.separator();
            
            // Tool selection
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.selected_tool, Tool::Select, "🔍 Select");
                ui.selectable_value(&mut self.selected_tool, Tool::Brush, "🖌️ Brush");
                ui.selectable_value(&mut self.selected_tool, Tool::Text, "📝 Text");
                ui.selectable_value(&mut self.selected_tool, Tool::Shape, "⬜ Shape");
                ui.selectable_value(&mut self.selected_tool, Tool::Code, "💻 Code");
            });
            
            ui.separator();
            
            // Action buttons
            if ui.button("💾 Save").clicked() {
                self.save_modal_open = true;
            }
            if ui.button("⚙️ Settings").clicked() {
                self.settings_modal_open = true;
            }
            if ui.button("↶ Undo").clicked() {
                if let Some(prev) = self.code_undoer.undo(&self.code) {
                    self.code = prev.to_string();
                }
            }
            if ui.button("↷ Redo").clicked() {
                if let Some(next) = self.code_undoer.redo(&self.code) {
                    self.code = next.to_string();
                }
            }
        });
    }

    fn left_panel_ui(&mut self, ui: &mut Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Tools & Colors");
            
            // Color palette
            ui.group(|ui| {
                ui.label("Color Palette");
                
                // Primary color picker
                ui.horizontal(|ui| {
                    ui.label("Primary:");
                    color_picker_color32(ui, &mut self.primary_color, Alpha::Opaque);
                });
                
                // Secondary color picker
                ui.horizontal(|ui| {
                    ui.label("Secondary:");
                    color_picker_color32(ui, &mut self.secondary_color, Alpha::Opaque);
                });
                
                // Color swatches with drag and drop
                ui.label("Swatches:");
                Grid::new("color_swatches").num_columns(3).show(ui, |ui| {
                    let _indices: Vec<usize> = (0..self.colors.len()).collect();
                    for (i, color) in self.colors.iter_mut().enumerate() {
                        let response = ui.color_edit_button_srgba(color);
                        
                        // Make colors draggable
                        if response.drag_started() {
                            self.dragged_item = Some(i);
                        }
                        
                        if response.hovered() && self.dragged_item.is_some() {
                            if let Some(from_idx) = self.dragged_item {
                                if from_idx != i {
                                    // Highlight drop target - simplified
                                    ui.painter().rect(
                                        response.rect,
                                        0.0,
                                        Color32::TRANSPARENT,
                                        egui::Stroke::new(2.0, Color32::WHITE),
                                        egui::StrokeKind::Inside,
                                    );
                                }
                            }
                        }
                        
                        if response.drag_stopped() {
                            if let Some(from_idx) = self.dragged_item {
                                if from_idx != i {
                                    // Perform the swap after the loop to avoid double mutable borrow
                                    self.drop_target = Some(i);
                                } else {
                                    self.dragged_item = None;
                                }
                            }
                        }
                        
                        if (i + 1) % 3 == 0 {
                            ui.end_row();
                        }
                    }
                });
                
                // Handle drag-and-drop swap after the loop
                if let (Some(from_idx), Some(target_idx)) = (self.dragged_item, self.drop_target) {
                    if from_idx != target_idx {
                        self.colors.swap(from_idx, target_idx);
                    }
                }
                self.dragged_item = None;
                self.drop_target = None;
            });
            
            // Widget gallery
            ui.group(|ui| {
                ui.label("Widget Gallery");
                
                ui.add(Slider::new(&mut self.slider_value, 0.0..=100.0).text("Value"));
                ui.checkbox(&mut self.toggle_state, "Toggle State");
                ui.text_edit_singleline(&mut self.text_input);
                
                if ui.button("Generate Random Color").clicked() {
                    use std::hash::{Hash, Hasher};
                    use std::collections::hash_map::DefaultHasher;
                    
                    let mut hasher = DefaultHasher::new();
                    self.click_count.hash(&mut hasher);
                    let hash = hasher.finish();
                    
                    self.primary_color = Color32::from_rgb(
                        (hash as u8) ,
                        ((hash >> 8) as u8) ,
                        ((hash >> 16) as u8) ,
                    );
                }
            });
        });
    }

    fn main_canvas_ui(&mut self, ui: &mut Ui) {
        Frame::canvas(ui.style()).show(ui, |ui| {
            ui.set_min_size(Vec2::new(400.0, 300.0));
            
            // Interactive container with scene
            let response = ui
                .scope_builder(
                    UiBuilder::new()
                        .id_salt("creative_canvas")
                        .sense(Sense::click_and_drag()),
                    |ui| {
                        let response = ui.response();
                        let visuals = ui.style().interact(&response);
                        
                        // Draw background
                        let rect = ui.available_rect_before_wrap();
                        ui.painter().rect_filled(
                            rect,
                            0.0,
                            visuals.bg_fill.gamma_multiply(0.1),
                        );
                        
                        // Scene navigation
                        ui.horizontal(|ui| {
                            ui.label("Scene:");
                            ui.add(DragValue::new(&mut self.scene_zoom).speed(0.01).prefix("Zoom: "));
                            ui.add(DragValue::new(&mut self.scene_offset.x).speed(1.0).prefix("X: "));
                            ui.add(DragValue::new(&mut self.scene_offset.y).speed(1.0).prefix("Y: "));
                        });
                        
                        // Interactive elements based on selected tool
                        match self.selected_tool {
                            Tool::Select => {
                                ui.label("Click to select elements");
                            }
                            Tool::Brush => {
                                if response.drag_delta().length() > 0.0 {
                                    let pos = response.interact_pointer_pos();
                                    if let Some(pos) = pos {
                                        ui.painter().circle_filled(
                                            pos,
                                            5.0,
                                            self.primary_color,
                                        );
                                    }
                                }
                            }
                            Tool::Text => {
                                ui.label(RichText::new(&self.text_input).color(self.primary_color));
                            }
                            Tool::Shape => {
                                ui.painter().rect_filled(
                                    Rect::from_center_size(
                                        rect.center(),
                                        Vec2::splat(self.slider_value * 2.0),
                                    ),
                                    0.0,
                                    self.secondary_color,
                                );
                            }
                            Tool::Code => {
                                // Live code editor
                                let mut code = self.code.clone();
                                let response = TextEdit::multiline(&mut code)
                                    .code_editor()
                                    .desired_width(f32::INFINITY)
                                    .desired_rows(10)
                                    .show(ui);
                                
                                if response.response.changed() {
                                    self.code_undoer.feed_state(0.0, &self.code);
                                    self.code = code;
                                }
                                
                                if ui.button("Run Code").clicked() {
                                    // Simulate code execution
                                    ui.label("Code executed!");
                                }
                            }
                        }
                        
                        // Interactive counter
                        ui.vertical_centered(|ui| {
                            ui.add_space(50.0);
                            ui.label(
                                RichText::new(format!("Clicks: {}", self.click_count))
                                    .size(24.0)
                                    .color(self.primary_color),
                            );
                        });
                    },
                )
                .response;
            
            if response.clicked() {
                self.click_count += 1;
            }
            
            if response.drag_delta().length() > 0.0 {
                self.scene_offset += response.drag_delta();
            }
        });
    }

    fn right_panel_ui(&mut self, ui: &mut Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Properties");
            
            // Panel management
            ui.group(|ui| {
                ui.label("Panels");
                for panel in &mut self.panels {
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut panel.visible, &panel.title);
                        if panel.visible {
                            ui.label(format!("@ ({:.0}, {:.0})", panel.position.x, panel.position.y));
                        }
                    });
                }
            });
            
            // Properties based on selected tool
            ui.group(|ui| {
                ui.label("Tool Properties");
                match self.selected_tool {
                    Tool::Brush => {
                        ui.label("Brush Settings");
                        ui.add(Slider::new(&mut self.slider_value, 1.0..=50.0).text("Size"));
                    }
                    Tool::Text => {
                        ui.label("Text Settings");
                        ui.text_edit_singleline(&mut self.text_input);
                    }
                    Tool::Code => {
                        ui.label("Code Settings");
                        ui.checkbox(&mut self.toggle_state, "Auto-save");
                    }
                    _ => {
                        ui.label("No specific properties for this tool");
                    }
                }
            });
            
            // Quick actions
            ui.group(|ui| {
                ui.label("Quick Actions");
                if ui.button("Reset View").clicked() {
                    self.scene_zoom = 1.0;
                    self.scene_offset = Vec2::ZERO;
                }
                if ui.button("Clear Canvas").clicked() {
                    self.click_count = 0;
                }
                if ui.button("Randomize Colors").clicked() {
                    use std::hash::{Hash, Hasher};
                    use std::collections::hash_map::DefaultHasher;
                    
                    let mut hasher = DefaultHasher::new();
                    self.click_count.hash(&mut hasher);
                    let hash = hasher.finish();
                    
                    self.primary_color = Color32::from_rgb(
                        (hash as u8) ,
                        ((hash >> 8) as u8) ,
                        ((hash >> 16) as u8) ,
                    );
                    
                    let mut hasher2 = DefaultHasher::new();
                    self.click_count.wrapping_add(1).hash(&mut hasher2);
                    let hash2 = hasher2.finish();
                    
                    self.secondary_color = Color32::from_rgb(
                        (hash2 as u8) ,
                        ((hash2 >> 8) as u8) ,
                        ((hash2 >> 16) as u8) ,
                    );
                }
            });
        });
    }

    fn save_modal_ui(&mut self, ctx: &Context) {
        if !self.save_modal_open {
            return;
        }
        
        Modal::new(Id::new("save_modal")).show(ctx, |ui| {
            ui.heading("Save Project");
            ui.label("Save your creative project?");
            
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    // Simulate save
                    self.save_modal_open = false;
                }
                if ui.button("Cancel").clicked() {
                    self.save_modal_open = false;
                }
            });
        });
    }

    fn settings_modal_ui(&mut self, ctx: &Context) {
        if !self.settings_modal_open {
            return;
        }
        
        Modal::new(Id::new("settings_modal")).show(ctx, |ui| {
            ui.heading("Settings");
            
            ui.group(|ui| {
                ui.label("Appearance");
                ui.checkbox(&mut self.toggle_state, "Dark Mode");
                ui.add(Slider::new(&mut self.slider_value, 0.0..=100.0).text("UI Scale"));
            });
            
            ui.group(|ui| {
                ui.label("Behavior");
                ui.checkbox(&mut self.toggle_state, "Auto-save");
                ui.checkbox(&mut self.toggle_state, "Show Grid");
            });
            
            if ui.button("Close").clicked() {
                self.settings_modal_open = false;
            }
        });
    }
}
