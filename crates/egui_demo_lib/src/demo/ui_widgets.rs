use egui::{Button, CollapsingHeader, Context, Grid, util::undoer::Undoer, Window, ScrollArea, TextStyle, TextWrapMode, Align, Color32, DragValue, Rect, Slider, Vec2, Id, Modal, ProgressBar, ComboBox, Popup, PopupCloseBehavior, RectAlign, RichText, Tooltip, Frame, include_image, Pos2, Scene, Widget};
use crate::egui_github_link_file;
use super::drag_and_drop::DragAndDropDemo;
use super::interactive_container::InteractiveContainerDemo;
use egui::color_picker::{Alpha, color_picker_color32};
use egui::containers::menu::{MenuConfig, SubMenuButton};
use egui_extras::{Size, StripBuilder};



pub const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
pub const LOREM_IPSUM_LONG: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.\n\nCurabitur pretium tincidunt lacus. Nulla gravida orci a odio. Nullam various, turpis et commodo pharetra, est eros bibendum elit, nec luctus magna felis sollicitudin mauris. Integer in mauris eu nibh euismod gravida. Duis ac tellus et risus vulputate vehicula. Donec lobortis risus a elit. Etiam tempor. Ut ullamcorper, ligula eu tempor congue, eros est euismod turpis, id tincidunt sapien risus a quam. Maecenas fermentum consequat mi. Donec fermentum. Pellentesque malesuada nulla a mi. Duis sapien sem, aliquet nec, commodo eget, consequat quis, neque. Aliquam faucibus, elit ut dictum aliquet, felis nisl adipiscing sapien, sed malesuada diam lacus eget erat. Cras mollis scelerisque nunc. Nullam arcu. Aliquam consequat. Curabitur augue lorem, dapibus quis, laoreet et, pretium ac, nisi. Aenean magna nisl, mollis quis, molestie eu, feugiat in, orci. In hac habitasse platea dictumst.";

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum Enum {
    First,
    Second,
    Third,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct TextState {
    pub text: String,
}

impl Default for TextState {
    fn default() -> Self {
        Self {
            text: "Edit this text".to_owned(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(PartialEq)]
struct ScrollTo {
    track_item: usize,
    tack_item_align: Option<Align>,
    offset: f32,
    delta: f32,
}

impl Default for ScrollTo {
    fn default() -> Self {
        Self {
            track_item: 25,
            tack_item_align: Some(Align::Center),
            offset: 0.0,
            delta: 64.0,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Default, PartialEq)]
struct ScrollStickTo {
    n_items: usize,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, PartialEq)]
pub struct PopupsDemo {
    align4: RectAlign,
    gap: f32,
    #[cfg_attr(feature = "serde", serde(skip))]
    close_behavior: PopupCloseBehavior,
    popup_open: bool,
    checked: bool,
    color: egui::Color32,
}

impl Default for PopupsDemo {
    fn default() -> Self {
        Self {
            align4: RectAlign::default(),
            gap: 4.0,
            close_behavior: PopupCloseBehavior::CloseOnClick,
            popup_open: false,
            checked: false,
            color: egui::Color32::RED,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ModalsDemo {
    user_modal_open: bool,
    save_modal_open: bool,
    save_progress: Option<f32>,

    role: String,
    name: String,
}

impl Default for ModalsDemo {
    fn default() -> Self {
        Self {
            user_modal_open: false,
            save_modal_open: false,
            save_progress: None,
            role: Self::ROLES[0].to_owned(),
            name: "John Doe".to_owned(),
        }
    }
}

impl ModalsDemo {
    const ROLES: [&'static str; 2] = ["user", "admin"];
}



#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct SceneDemo {
    scene_rect: Rect,
}

impl Default for SceneDemo {
    fn default() -> Self {
        Self {
            scene_rect: Rect::ZERO, // `egui::Scene` will initialize this to something valid
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct CodeExample {
    language: String,
    code: String,
}

impl Default for CodeExample {
    fn default() -> Self {
        Self {
            language: "rs".into(),
            code: "// A very simple example\n\nfn main() {\n\tprintln!(\"Hello world!\");\n}\n"
            .into(),
        }
    }
}


#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct UiWidgets {
    enabled: bool,
    visible: bool,
    boolean: bool,
    opacity: f32,
    radio: Enum,
    scalar: f32,
    string: String,
    color: egui::Color32,
    animate_progress_bar: bool,

    #[cfg(feature = "chrono")]
    #[cfg_attr(feature = "serde", serde(skip))]
    date: Option<chrono::NaiveDate>,

    #[cfg(feature = "chrono")]
    with_date_button: bool,

    text_state: TextState,
    undoer: Undoer<TextState>,
    scroll_to: ScrollTo,
    scroll_stick_to: ScrollStickTo,
    popups_demo: PopupsDemo,
    modals_demo: ModalsDemo,
    tooltips_enabled: bool,
    dnd_demo: super::drag_and_drop::DragAndDropDemo,
    interactive_container_demo: super::interactive_container::InteractiveContainerDemo,
    scene_demo: SceneDemo,
    code_example: CodeExample,
}

impl Default for UiWidgets {
    fn default() -> Self {
        Self {
            enabled: true,
            visible: true,
            opacity: 1.0,
            boolean: false,
            radio: Enum::First,
            scalar: 42.0,
            string: Default::default(),
            color: egui::Color32::LIGHT_BLUE.linear_multiply(0.5),
            animate_progress_bar: false,
            #[cfg(feature = "chrono")]
            date: None,
            #[cfg(feature = "chrono")]
            with_date_button: true,
            text_state: TextState::default(),
            undoer: Undoer::default(),
            scroll_to: ScrollTo::default(),
            scroll_stick_to: ScrollStickTo::default(),
            popups_demo: PopupsDemo::default(),
            modals_demo: ModalsDemo::default(),
            tooltips_enabled: true,
            dnd_demo: DragAndDropDemo::default(),
            interactive_container_demo: InteractiveContainerDemo::default(),
            scene_demo: SceneDemo::default(),
            code_example: CodeExample::default(),
        }
    }
}

impl crate::Demo for UiWidgets {
    fn name(&self) -> &'static str {
        "UI Widgets"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        use crate::View as _;
        Window::new(self.name())
            .open(open)
            .vscroll(true)
            .resizable(true)
            .default_height(500.0)
            .show(ctx, |ui| self.ui(ui));
    }
}

impl crate::View for UiWidgets {
    fn ui(&mut self, ui: &mut egui::Ui) {
        CollapsingHeader::new("Basic Widgets")
            .default_open(true)
            .show(ui, |ui| {
                Grid::new("my_grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        self.gallery_grid_contents(ui);
                    });
            });

        CollapsingHeader::new("Text Editing")
            .default_open(false)
            .show(ui, |ui| {
                self.text_editing_ui(ui);
            });

        CollapsingHeader::new("Panels and Layout")
            .default_open(false)
            .show(ui, |ui| {
                self.panels_and_layout_ui(ui);
            });

        CollapsingHeader::new("Scrolling")
            .default_open(false)
            .show(ui, |ui| {
                self.scrolling_ui(ui);
            });

        CollapsingHeader::new("Popups, Modals, and Tooltips")
            .default_open(false)
            .show(ui, |ui| {
                self.popups_modals_and_tooltips_ui(ui);
            });

        CollapsingHeader::new("Interactive Elements")
            .default_open(false)
            .show(ui, |ui| {
                self.interactive_elements_ui(ui);
            });

        CollapsingHeader::new("Code Example")
            .default_open(false)
            .show(ui, |ui| {
                self.code_example_ui(ui);
            });
    }
}

impl UiWidgets {
    fn gallery_grid_contents(&mut self, ui: &mut egui::Ui) {
        let Self {
            enabled: _,
            visible: _,
            opacity: _,
            boolean,
            radio,
            scalar,
            string,
            color,
            animate_progress_bar,
            #[cfg(feature = "chrono")]
            date,
            #[cfg(feature = "chrono")]
            with_date_button,
            .. // ignore the rest
        } = self;

        ui.add(doc_link_label("Label", "label"));
        ui.label("Welcome to the widget gallery!");
        ui.end_row();

        ui.add(doc_link_label("Hyperlink", "Hyperlink"));
        use egui::special_emojis::GITHUB;
        ui.hyperlink_to(
            format!("{GITHUB} egui on GitHub"),
            "https://github.com/emilk/egui",
        );
        ui.end_row();

        ui.add(doc_link_label("TextEdit", "TextEdit"));
        ui.add(egui::TextEdit::singleline(string).hint_text("Write something here"));
        ui.end_row();

        ui.add(doc_link_label("Button", "button"));
        if ui.button("Click me!").clicked() {
            *boolean = !*boolean;
        }
        ui.end_row();

        ui.add(doc_link_label("Link", "link"));
        if ui.link("Click me!").clicked() {
            *boolean = !*boolean;
        }
        ui.end_row();

        ui.add(doc_link_label("Checkbox", "checkbox"));
        ui.checkbox(boolean, "Checkbox");
        ui.end_row();

        ui.add(doc_link_label("RadioButton", "radio"));
        ui.horizontal(|ui| {
            ui.radio_value(radio, Enum::First, "First");
            ui.radio_value(radio, Enum::Second, "Second");
            ui.radio_value(radio, Enum::Third, "Third");
        });
        ui.end_row();

        ui.add(doc_link_label("SelectableLabel", "SelectableLabel"));
        ui.horizontal(|ui| {
            ui.selectable_value(radio, Enum::First, "First");
            ui.selectable_value(radio, Enum::Second, "Second");
            ui.selectable_value(radio, Enum::Third, "Third");
        });
        ui.end_row();

        ui.add(doc_link_label("ComboBox", "ComboBox"));

        egui::ComboBox::from_label("Take your pick")
            .selected_text(format!("{radio:?}"))
            .show_ui(ui, |ui| {
                ui.selectable_value(radio, Enum::First, "First");
                ui.selectable_value(radio, Enum::Second, "Second");
                ui.selectable_value(radio, Enum::Third, "Third");
            });
        ui.end_row();

        ui.add(doc_link_label("Slider", "Slider"));
        ui.add(egui::Slider::new(scalar, 0.0..=360.0).suffix("°"));
        ui.end_row();

        ui.add(doc_link_label("DragValue", "DragValue"));
        ui.add(egui::DragValue::new(scalar).speed(1.0));
        ui.end_row();

        ui.add(doc_link_label("ProgressBar", "ProgressBar"));
        let progress = *scalar / 360.0;
        let progress_bar = egui::ProgressBar::new(progress)
            .show_percentage()
            .animate(*animate_progress_bar);
        *animate_progress_bar = ui
            .add(progress_bar)
            .on_hover_text("The progress bar can be animated!")
            .hovered();
        ui.end_row();

        ui.add(doc_link_label("Color picker", "color_edit"));
        ui.color_edit_button_srgba(color);
        ui.end_row();

        ui.add(doc_link_label("Image", "Image"));
        let egui_icon = egui::include_image!("../../data/icon.png");
        ui.add(egui::Image::new(egui_icon.clone()));
        ui.end_row();

        ui.add(doc_link_label(
            "Button with image",
            "Button::image_and_text",
        ));
        if ui
            .add(egui::Button::image_and_text(egui_icon, "Click me!"))
            .clicked()
        {
            *boolean = !*boolean;
        }
        ui.end_row();

        #[cfg(feature = "chrono")]
        if *with_date_button {
            let date = date.get_or_insert_with(|| chrono::offset::Utc::now().date_naive());
            ui.add(doc_link_label_with_crate(
                "egui_extras",
                "DatePickerButton",
                "DatePickerButton",
            ));
            ui.add(egui_extras::DatePickerButton::new(date));
            ui.end_row();
        }

        ui.add(doc_link_label("Separator", "separator"));
        ui.separator();
        ui.end_row();

        ui.add(doc_link_label("CollapsingHeader", "collapsing"));
        ui.collapsing("Click to see what is hidden!", |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("It's a ");
                ui.add(doc_link_label("Spinner", "spinner"));
                ui.add_space(4.0);
                ui.add(egui::Spinner::new());
            });
        });
        ui.end_row();

        ui.hyperlink_to(
            "Custom widget",
            super::toggle_switch::url_to_file_source_code(),
        );
        ui.add(super::toggle_switch::toggle(boolean)).on_hover_text(
            "It's easy to create your own widgets!\n\nThis toggle switch is just 15 lines of code.",
        );
        ui.end_row();
    }

    fn text_editing_ui(&mut self, ui: &mut egui::Ui) {
        let output = egui::TextEdit::multiline(&mut self.text_state.text)
            .hint_text("Type something!")
            .show(ui);

        ui.horizontal(|ui| {
            let can_undo = self.undoer.has_undo(&self.text_state);
            let can_redo = self.undoer.has_redo(&self.text_state);

            let undo = ui.add_enabled(can_undo, Button::new("⟲ Undo")).clicked();
            let redo = ui.add_enabled(can_redo, Button::new("⟳ Redo")).clicked();

            if undo {
                if let Some(undo_text) = self.undoer.undo(&self.text_state) {
                    self.text_state = undo_text.clone();
                }
            }
            if redo {
                if let Some(redo_text) = self.undoer.redo(&self.text_state) {
                    self.text_state = redo_text.clone();
                }
            }
        });

        self.undoer
            .feed_state(ui.ctx().input(|input| input.time), &self.text_state);

        ui.horizontal(|ui| {
            ui.label("Move cursor to the:");

            if ui.button("start").clicked() {
                let text_edit_id = output.response.id;
                if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), text_edit_id) {
                    let ccursor = egui::text::CCursor::new(0);
                    state
                        .cursor
                        .set_char_range(Some(egui::text::CCursorRange::one(ccursor)));
                    state.store(ui.ctx(), text_edit_id);
                    ui.ctx().memory_mut(|mem| mem.request_focus(text_edit_id)); // give focus back to the [`TextEdit`].
                }
            }

            if ui.button("end").clicked() {
                let text_edit_id = output.response.id;
                if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), text_edit_id) {
                    let ccursor = egui::text::CCursor::new(self.text_state.text.chars().count());
                    state
                        .cursor
                        .set_char_range(Some(egui::text::CCursorRange::one(ccursor)));
                    state.store(ui.ctx(), text_edit_id);
                    ui.ctx().memory_mut(|mem| mem.request_focus(text_edit_id)); // give focus back to the [`TextEdit`].
                }
            }
        });
    }

    fn panels_and_layout_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Panels");

        egui::TopBottomPanel::top("top_panel")
            .resizable(true)
            .min_height(32.0)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Expandable Upper Panel");
                    });
                    lorem_ipsum(ui);
                });
            });

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(150.0)
            .width_range(80.0..=200.0)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Left Panel");
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    lorem_ipsum(ui);
                });
            });

        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(150.0)
            .width_range(80.0..=200.0)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Right Panel");
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    lorem_ipsum(ui);
                });
            });

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(0.0)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Bottom Panel");
                });
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Central Panel");
            });
            egui::ScrollArea::vertical().show(ui, |ui| {
                lorem_ipsum(ui);
            });
        });

        ui.label("StripBuilder");

        let dark_mode = ui.visuals().dark_mode;
        let faded_color = ui.visuals().window_fill();
        let faded_color = |color: egui::Color32| -> egui::Color32 {
            use egui::Rgba;
            let t = if dark_mode { 0.95 } else { 0.8 };
            egui::lerp(Rgba::from(color)..=Rgba::from(faded_color), t).into()
        };

        let body_text_size = egui::TextStyle::Body.resolve(ui.style()).size;
        StripBuilder::new(ui)
            .size(Size::exact(50.0))
            .size(Size::remainder())
            .size(Size::relative(0.5).at_least(60.0))
            .size(Size::exact(body_text_size))
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    ui.painter().rect_filled(
                        ui.available_rect_before_wrap(),
                        0.0,
                        faded_color(egui::Color32::BLUE),
                    );
                    ui.label("width: 100%\nheight: 50px");
                });
                strip.strip(|builder| {
                    builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                        strip.cell(|ui| {
                            ui.painter().rect_filled(
                                ui.available_rect_before_wrap(),
                                0.0,
                                faded_color(egui::Color32::RED),
                            );
                            ui.label("width: 50%\nheight: remaining");
                        });
                        strip.strip(|builder| {
                            builder.sizes(Size::remainder(), 3).vertical(|mut strip| {
                                strip.empty();
                                strip.cell(|ui| {
                                    ui.painter().rect_filled(
                                        ui.available_rect_before_wrap(),
                                        0.0,
                                        faded_color(egui::Color32::YELLOW),
                                    );
                                    ui.label("width: 50%\nheight: 1/3 of the red region");
                                });
                                strip.empty();
                            });
                        });
                    });
                });
                strip.strip(|builder| {
                    builder
                        .size(Size::remainder())
                        .size(Size::exact(120.0))
                        .size(Size::remainder())
                        .size(Size::exact(70.0))
                        .horizontal(|mut strip| {
                            strip.empty();
                            strip.strip(|builder| {
                                builder
                                    .size(Size::remainder())
                                    .size(Size::exact(60.0))
                                    .size(Size::remainder())
                                    .vertical(|mut strip| {
                                        strip.empty();
                                        strip.cell(|ui| {
                                            ui.painter().rect_filled(
                                                ui.available_rect_before_wrap(),
                                                0.0,
                                                faded_color(egui::Color32::GOLD),
                                            );
                                            ui.label("width: 120px\nheight: 60px");
                                        });
                                    });
                            });
                            strip.empty();
                            strip.cell(|ui| {
                                ui.painter().rect_filled(
                                    ui.available_rect_before_wrap(),
                                    0.0,
                                    faded_color(egui::Color32::GREEN),
                                );
                                ui.label("width: 70px\n\nheight: 50%, but at least 60px.");
                            });
                        });
                });
                strip.cell(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.add(egui_github_link_file!());
                    });
                });
            });
    }

    fn scrolling_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Scroll to a specific item or pixel offset");

        let num_items = 500;

        let mut track_item = false;
        let mut go_to_scroll_offset = false;
        let mut scroll_top = false;
        let mut scroll_bottom = false;
        let mut scroll_delta = None;

        ui.horizontal(|ui| {
            ui.label("Scroll to a specific item index:");
            track_item |= ui
                .add(Slider::new(&mut self.scroll_to.track_item, 1..=num_items).text("Track Item"))
                .dragged();
        });

        ui.horizontal(|ui| {
            ui.label("Item align:");
            track_item |= ui
                .radio_value(&mut self.scroll_to.tack_item_align, Some(Align::Min), "Top")
                .clicked();
            track_item |= ui
                .radio_value(&mut self.scroll_to.tack_item_align, Some(Align::Center), "Center")
                .clicked();
            track_item |= ui
                .radio_value(&mut self.scroll_to.tack_item_align, Some(Align::Max), "Bottom")
                .clicked();
            track_item |= ui
                .radio_value(&mut self.scroll_to.tack_item_align, None, "None (Bring into view)")
                .clicked();
        });

        ui.horizontal(|ui| {
            ui.label("Scroll to a specific offset:");
            go_to_scroll_offset |= ui
                .add(DragValue::new(&mut self.scroll_to.offset).speed(1.0).suffix("px"))
                .dragged();
        });

        ui.horizontal(|ui| {
            scroll_top |= ui.button("Scroll to top").clicked();
            scroll_bottom |= ui.button("Scroll to bottom").clicked();
        });

        ui.horizontal(|ui| {
            ui.label("Scroll by");
            DragValue::new(&mut self.scroll_to.delta)
                .speed(1.0)
                .suffix("px")
                .ui(ui);
            if ui.button("⬇").clicked() {
                scroll_delta = Some(self.scroll_to.delta * Vec2::UP); // scroll down (move contents up)
            }
            if ui.button("⬆").clicked() {
                scroll_delta = Some(self.scroll_to.delta * Vec2::DOWN); // scroll up (move contents down)
            }
        });

        let mut scroll_area = ScrollArea::vertical().max_height(200.0).auto_shrink(false);
        if go_to_scroll_offset {
            scroll_area = scroll_area.vertical_scroll_offset(self.scroll_to.offset);
        }

        ui.separator();
        let (current_scroll, max_scroll) = scroll_area
            .show(ui, |ui| {
                if scroll_top {
                    ui.scroll_to_cursor(Some(Align::TOP));
                }
                if let Some(scroll_delta) = scroll_delta {
                    ui.scroll_with_delta(scroll_delta);
                }

                ui.vertical(|ui| {
                    for item in 1..=num_items {
                        if track_item && item == self.scroll_to.track_item {
                            let response =
                                ui.colored_label(Color32::YELLOW, format!("This is item {item}"));
                            response.scroll_to_me(self.scroll_to.tack_item_align);
                        } else {
                            ui.label(format!("This is item {item}"));
                        }
                    }
                });

                if scroll_bottom {
                    ui.scroll_to_cursor(Some(Align::BOTTOM));
                }

                let margin = ui.visuals().clip_rect_margin;

                let current_scroll = ui.clip_rect().top() - ui.min_rect().top() + margin;
                let max_scroll = ui.min_rect().height() - ui.clip_rect().height() + 2.0 * margin;
                (current_scroll, max_scroll)
            })
            .inner;
        ui.separator();

        ui.label(format!(
            "Scroll offset: {current_scroll:.0}/{max_scroll:.0} px"
        ));

        ui.separator();

        ui.label("Bidirectional scrolling");
        ScrollArea::both().show(ui, |ui| {
            ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
            for _ in 0..100 {
                ui.label(LOREM_IPSUM);
            }
        });

        ui.label("Stick to end");
        let text_style = TextStyle::Body;
        let row_height = ui.text_style_height(&text_style);
        ScrollArea::vertical().stick_to_bottom(true).show_rows(
            ui,
            row_height,
            self.scroll_stick_to.n_items,
            |ui, row_range| {
                for row in row_range {
                    let text = format!("This is row {}", row + 1);
                    ui.label(text);
                }
            },
        );
        self.scroll_stick_to.n_items += 1;
        ui.ctx().request_repaint();
    }

    fn popups_modals_and_tooltips_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Popups");
        let response = Frame::group(ui.style())
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.vertical_centered(|ui| ui.button("Click, right-click and hover me!"))
                    .inner
            })
            .inner;

        self.popups_demo.apply_options(Popup::menu(&response).id(Id::new("menu")))
            .show(|ui| self.popups_demo.nested_menus(ui));

        self.popups_demo.apply_options(Popup::context_menu(&response).id(Id::new("context_menu")))
            .show(|ui| self.popups_demo.nested_menus(ui));

        if self.popups_demo.popup_open {
            self.popups_demo.apply_options(Popup::from_response(&response).id(Id::new("popup")))
                .show(|ui| {
                    ui.label("Popup contents");
                });
        }

        let mut tooltip = Tooltip::for_enabled(&response);
        tooltip.popup = self.popups_demo.apply_options(tooltip.popup);
        tooltip.show(|ui| {
            ui.label("Tooltips are popups, too!");
        });

        ui.label("Modals");
        ui.horizontal(|ui| {
            if ui.button("Open User Modal").clicked() {
                self.modals_demo.user_modal_open = true;
            }

            if ui.button("Open Save Modal").clicked() {
                self.modals_demo.save_modal_open = true;
            }
        });

        if self.modals_demo.user_modal_open {
            let modal = Modal::new(Id::new("Modal A")).show(ui.ctx(), |ui| {
                ui.set_width(250.0);

                ui.heading("Edit User");

                ui.label("Name:");
                ui.text_edit_singleline(&mut self.modals_demo.name);

                ComboBox::new("role", "Role")
                    .selected_text(&self.modals_demo.role)
                    .show_ui(ui, |ui| {
                        for r in ModalsDemo::ROLES {
                            ui.selectable_value(&mut self.modals_demo.role, r.to_owned(), r);
                        }
                    });

                ui.separator();

                egui::Sides::new().show(
                    ui,
                    |_ui| {},
                    |ui| {
                        if ui.button("Save").clicked() {
                            self.modals_demo.save_modal_open = true;
                        }
                        if ui.button("Cancel").clicked() {
                            // You can call `ui.close()` to close the modal.
                            // (This causes the current modals `should_close` to return true)
                            ui.close();
                        }
                    },
                );
            });

            if modal.should_close() {
                self.modals_demo.user_modal_open = false;
            }
        }

        if self.modals_demo.save_modal_open {
            let modal = Modal::new(Id::new("Modal B")).show(ui.ctx(), |ui| {
                ui.set_width(200.0);
                ui.heading("Save? Are you sure?");

                ui.add_space(32.0);

                egui::Sides::new().show(
                    ui,
                    |_ui| {},
                    |ui| {
                        if ui.button("Yes Please").clicked() {
                            self.modals_demo.save_progress = Some(0.0);
                        }

                        if ui.button("No Thanks").clicked() {
                            ui.close();
                        }
                    },
                );
            });

            if modal.should_close() {
                self.modals_demo.save_modal_open = false;
            }
        }

        if let Some(progress) = self.modals_demo.save_progress {
            Modal::new(Id::new("Modal C")).show(ui.ctx(), |ui| {
                ui.set_width(70.0);
                ui.heading("Saving…");

                ProgressBar::new(progress).ui(ui);

                if progress >= 1.0 {
                    self.modals_demo.save_progress = None;
                    self.modals_demo.save_modal_open = false;
                    self.modals_demo.user_modal_open = false;
                } else {
                    self.modals_demo.save_progress = Some(progress + 0.003);
                    ui.ctx().request_repaint();
                }
            });
        }

        ui.label("Tooltips");
        ui.label("All labels in this demo have tooltips.")
            .on_hover_text("Yes, even this one.");

        ui.label("Some widgets have multiple tooltips!")
            .on_hover_text("The first tooltip.")
            .on_hover_text("The second tooltip.");

        ui.label("Tooltips can contain interactive widgets.")
            .on_hover_ui(|ui| {
                ui.label("This tooltip contains a link:");
                ui.hyperlink_to("www.egui.rs", "https://www.egui.rs/")
                    .on_hover_text("The tooltip has a tooltip in it!");
            });

        ui.label("You can put selectable text in tooltips too.")
            .on_hover_ui(|ui| {
                ui.style_mut().interaction.selectable_labels = true;
                ui.label("You can select this text.");
            });

        ui.label("This tooltip shows at the mouse cursor.")
            .on_hover_text_at_pointer("Move me around!!");

        ui.separator(); // ---------------------------------------------------------

        let tooltip_ui = |ui: &mut egui::Ui| {
            ui.horizontal(|ui| {
                ui.label("This tooltip was created with");
                ui.code(".on_hover_ui(…)");
            });
        };
        let disabled_tooltip_ui = |ui: &mut egui::Ui| {
            ui.label("A different tooltip when widget is disabled.");
            ui.horizontal(|ui| {
                ui.label("This tooltip was created with");
                ui.code(".on_disabled_hover_ui(…)");
            });
        };

        ui.label("You can have different tooltips depending on whether or not a widget is enabled:")
            .on_hover_text("Check the tooltip of the button below, and see how it changes depending on whether or not it is enabled.");

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.tooltips_enabled, "Enabled")
                .on_hover_text("Controls whether or not the following button is enabled.");

            ui.add_enabled(self.tooltips_enabled, egui::Button::new("Sometimes clickable"))
                .on_hover_ui(tooltip_ui)
                .on_disabled_hover_ui(disabled_tooltip_ui);
        });
    }

    fn interactive_elements_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Drag and Drop");
        crate::View::ui(&mut self.dnd_demo, ui);

        ui.label("Interactive Container");
        crate::View::ui(&mut self.interactive_container_demo, ui);

        ui.label("Scene");
        let scene = Scene::new()
            .max_inner_size([350.0, 1000.0])
            .zoom_range(0.1..=2.0);

        let mut reset_view = false;
        let mut inner_rect = Rect::NAN;
        let mut scene_rect = self.scene_demo.scene_rect;
        let response = scene
            .show(ui, &mut scene_rect, |ui| {
                reset_view = ui.button("Reset view").clicked();

                ui.add_space(16.0);

                self.gallery_grid_contents(ui);

                ui.put(
                    Rect::from_min_size(Pos2::new(0.0, -64.0), Vec2::new(200.0, 16.0)),
                    egui::Label::new("You can put a widget anywhere").selectable(false),
                );

                inner_rect = ui.min_rect();
            })
            .response;

        if reset_view || response.double_clicked() {
            self.scene_demo.scene_rect = inner_rect;
        }
        self.scene_demo.scene_rect = scene_rect;
    }

    fn code_example_ui(&mut self, ui: &mut egui::Ui) {
        self.code(ui);
    }

    fn samples_in_grid(&mut self, ui: &mut egui::Ui) {
        // Note: we keep the code narrow so that the example fits on a mobile screen.

        let Self { code_example, .. } = self;
        let CodeExample { language: _, code: _ } = code_example; // for brevity later on
        
        // Example variables for the demo
        let mut name = "John".to_string();
        let mut age = 42;

        show_code(ui, r#"ui.heading(\"Example\");"#);
        ui.heading("Example");
        ui.end_row();

        show_code(
            ui,
            r#"            ui.horizontal(|ui| {
                ui.label(\"Name\");
                ui.text_edit_singleline(&mut name);
            });"#,
        );
        // Putting things on the same line using ui.horizontal:
        ui.horizontal(|ui| {
            ui.label("Name");
            ui.text_edit_singleline(&mut name);
        });
        ui.end_row();

        show_code(
            ui,
            r#"            ui.add(
                egui::DragValue::new(&mut age)
                    .range(0..=120)
                    .suffix(\" years\"),
            );"#,
        );
        ui.add(egui::DragValue::new(&mut age).range(0..=120).suffix(" years"));
        ui.end_row();

        show_code(
            ui,
            r#"            if ui.button(\"Increment\").clicked() {
                age += 1;
            }"#,
        );
        if ui.button("Increment").clicked() {
            age += 1;
        }
        ui.end_row();

        #[expect(clippy::literal_string_with_formatting_args)]
        show_code(ui, r#"ui.label(format!(\"{name} is {age}\"));"#);
        ui.label(format!("{name} is {age}"));
        ui.end_row();
    }

    fn code(&mut self, ui: &mut egui::Ui) {
        show_code(
            ui,
            r#"            pub struct CodeExample {
    name: String,
    age: u32,
}

impl CodeExample {
    fn ui(&mut self, ui: &mut egui::Ui) {
        // Saves us from writing `&mut self.name` etc
        let Self { name, age } = self;"#,
        );

        ui.horizontal(|ui| {
            let font_id = egui::TextStyle::Monospace.resolve(ui.style());
            let indentation = 2.0 * 4.0 * ui.fonts(|f| f.glyph_width(&font_id, ' '));
            ui.add_space(indentation);

            egui::Grid::new("code_samples")
                .striped(true)
                .num_columns(2)
                .show(ui, |ui| {
                    self.samples_in_grid(ui);
                });
        });

        crate::rust_view_ui(ui, "    }\n}");
    }
}

impl PopupsDemo {
    fn apply_options<'a>(&self, popup: Popup<'a>) -> Popup<'a> {
        popup
            .align(self.align4)
            .gap(self.gap)
            .close_behavior(self.close_behavior)
    }

    fn nested_menus(&mut self, ui: &mut egui::Ui) {
        ui.set_max_width(200.0); // To make sure we wrap long text

        if ui.button("Open…").clicked() {
            ui.close();
        }
        ui.menu_button("Popups can have submenus", |ui| {
            ui.menu_button("SubMenu", |ui| {
                if ui.button("Open…").clicked() {
                    ui.close();
                }
                let _ = ui.button("Item");
                ui.menu_button("Recursive", |ui| self.nested_menus(ui));
            });
            ui.menu_button("SubMenu", |ui| {
                if ui.button("Open…").clicked() {
                    ui.close();
                }
                let _ = ui.button("Item");
            });
            let _ = ui.button("Item");
            if ui.button("Open…").clicked() {
                ui.close();
            }
        });
        ui.menu_image_text_button(
            include_image!("../../data/icon.png"),
            "I have an icon!",
            |ui| {
                let _ = ui.button("Item1");
                let _ = ui.button("Item2");
                let _ = ui.button("Item3");
                let _ = ui.button("Item4");
                if ui.button("Open…").clicked() {
                    ui.close();
                }
            },
        );
        let _ = ui.button("Very long text for this item that should be wrapped");
        SubMenuButton::new("Always CloseOnClickOutside")
            .config(MenuConfig::new().close_behavior(PopupCloseBehavior::CloseOnClickOutside))
            .ui(ui, |ui| {
                ui.checkbox(&mut self.checked, "Checkbox");

                // Customized color SubMenuButton
                let is_bright = self.color.intensity() > 0.5;
                let text_color = if is_bright {
                    egui::Color32::BLACK
                } else {
                    egui::Color32::WHITE
                };
                let mut color_button =
                    SubMenuButton::new(RichText::new("Background").color(text_color));
                color_button.button = color_button.button.fill(self.color);
                color_button.button = color_button
                    .button
                    .right_text(RichText::new(SubMenuButton::RIGHT_ARROW).color(text_color));
                color_button.ui(ui, |ui| {
                    ui.spacing_mut().slider_width = 200.0;
                    color_picker_color32(ui, &mut self.color, Alpha::Opaque);
                });

                if ui.button("Open…").clicked() {
                    ui.close();
                }
            });
    }
}

fn lorem_ipsum(ui: &mut egui::Ui) {
    ui.with_layout(
        egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
        |ui| {
            ui.label(egui::RichText::new(LOREM_IPSUM_LONG).small().weak());
            ui.add(egui::Separator::default().grow(8.0));
            ui.label(egui::RichText::new(LOREM_IPSUM_LONG).small().weak());
        },
    );
}

fn doc_link_label<'a>(title: &'a str, search_term: &'a str) -> impl egui::Widget + 'a {
    doc_link_label_with_crate("egui", title, search_term)
}

fn doc_link_label_with_crate<'a>(
    crate_name: &'a str,
    title: &'a str,
    search_term: &'a str,
) -> impl egui::Widget + 'a {
    let url = format!("https://docs.rs/{crate_name}?search={search_term}");
    move |ui: &mut egui::Ui| {
        ui.hyperlink_to(title, url).on_hover_ui(|ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("Search egui docs for");
                ui.code(search_term);
            });
        })
    }
}

fn show_code(ui: &mut egui::Ui, code: &str) {
    let code = remove_leading_indentation(code.trim_start_matches('\n'));
    crate::rust_view_ui(ui, &code);
}

fn remove_leading_indentation(code: &str) -> String {
    fn is_indent(c: &u8) -> bool {
        matches!(*c, b' ' | b'\t')
    }

    let first_line_indent = code.bytes().take_while(is_indent).count();

    let mut out = String::new();

    let mut code = code;
    while !code.is_empty() {
        let indent = code.bytes().take_while(is_indent).count();
        let start = first_line_indent.min(indent);
        let end = code
            .find('\n')
            .map_or_else(|| code.len(), |endline| endline + 1);
        out += &code[start..end];
        code = &code[end..];
    }
    out
}
