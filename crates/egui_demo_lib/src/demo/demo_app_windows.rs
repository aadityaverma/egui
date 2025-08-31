use std::collections::BTreeSet;

use super::About;
use crate::Demo;
use crate::View as _;
use crate::is_mobile;
use egui::containers::menu;
use egui::style::StyleModifier;
use egui::{Context, Modifiers, ScrollArea, Ui};
// ----------------------------------------------------------------------------

struct DemoGroup {
    demos: Vec<Box<dyn Demo>>,
}

impl std::ops::Add for DemoGroup {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut demos = self.demos;
        demos.extend(other.demos);
        Self { demos }
    }
}

impl DemoGroup {
    pub fn new(demos: Vec<Box<dyn Demo>>) -> Self {
        Self { demos }
    }

    pub fn checkboxes(&mut self, ui: &mut Ui, open: &mut BTreeSet<String>, accessible_windows: &BTreeSet<String>) {
        let Self { demos } = self;
        for demo in demos {
            if demo.is_enabled(ui.ctx()) && accessible_windows.contains(demo.name()) {
                let mut is_open = open.contains(demo.name());
                ui.toggle_value(&mut is_open, demo.name());
                set_open(open, demo.name(), is_open);
            }
        }
    }

    pub fn windows(&mut self, ctx: &Context, open: &mut BTreeSet<String>) {
        let Self { demos } = self;
        for demo in demos {
            let mut is_open = open.contains(demo.name());
            demo.show(ctx, &mut is_open);
            set_open(open, demo.name(), is_open);
        }
    }
}

fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}

// ----------------------------------------------------------------------------

pub struct DemoGroups {
    about: About,
    demos: DemoGroup,
    tests: DemoGroup,
}

impl Default for DemoGroups {
    fn default() -> Self {
        Self {
            about: About::default(),
            demos: DemoGroup::new(vec![
                Box::<super::paint_bezier::PaintBezier>::default(),
                Box::<super::code_editor::CodeEditor>::default(),
                Box::<super::code_example::CodeExample>::default(),
                Box::<super::creative_studio::CreativeStudio>::default(),
                Box::<super::dancing_strings::DancingStrings>::default(),
                Box::<super::drag_and_drop::DragAndDropDemo>::default(),
                Box::<super::extra_viewport::ExtraViewport>::default(),
                Box::<super::font_book::FontBook>::default(),
                Box::<super::frame_demo::FrameDemo>::default(),
                Box::<super::highlighting::Highlighting>::default(),
                Box::<super::interactive_container::InteractiveContainerDemo>::default(),
                Box::<super::MiscDemoWindow>::default(),
                Box::<super::modals::Modals>::default(),
                Box::<super::multi_touch::MultiTouch>::default(),
                Box::<super::painting::Painting>::default(),
                Box::<super::panels::Panels>::default(),
                Box::<super::popups::PopupsDemo>::default(),
                Box::<super::scene::SceneDemo>::default(),
                Box::<super::screenshot::Screenshot>::default(),
                Box::<super::scrolling::Scrolling>::default(),
                Box::<super::sliders::Sliders>::default(),
                Box::<super::strip_demo::StripDemo>::default(),
                Box::<super::table_demo::TableDemo>::default(),
                Box::<super::text_edit::TextEditDemo>::default(),
                Box::<super::text_layout::TextLayoutDemo>::default(),
                Box::<super::tooltips::Tooltips>::default(),
                Box::<super::undo_redo::UndoRedoDemo>::default(),
                Box::<super::widget_gallery::WidgetGallery>::default(),
                Box::<super::ui_widgets::UiWidgets>::default(),
                Box::<super::window_options::WindowOptions>::default(),
            ]),
            tests: DemoGroup::new(vec![
                Box::<super::tests::ClipboardTest>::default(),
                Box::<super::tests::CursorTest>::default(),
                Box::<super::tests::GridTest>::default(),
                Box::<super::tests::IdTest>::default(),
                Box::<super::tests::InputEventHistory>::default(),
                Box::<super::tests::InputTest>::default(),
                Box::<super::tests::LayoutTest>::default(),
                Box::<super::tests::ManualLayoutTest>::default(),
                Box::<super::tests::SvgTest>::default(),
                Box::<super::tests::TessellationTest>::default(),
                Box::<super::tests::WindowResizeTest>::default(),
            ]),
        }
    }
}

impl DemoGroups {
    pub fn checkboxes(&mut self, ui: &mut Ui, open: &mut BTreeSet<String>, accessible_windows: &BTreeSet<String>) {
        let Self {
            about,
            demos,
            tests,
        } = self;

        {
            if accessible_windows.contains(about.name()) {
                let mut is_open = open.contains(about.name());
                ui.toggle_value(&mut is_open, about.name());
                set_open(open, about.name(), is_open);
            }
        }
        ui.separator();
        demos.checkboxes(ui, open, accessible_windows);
        ui.separator();
        tests.checkboxes(ui, open, accessible_windows);
    }

    pub fn windows(&mut self, ctx: &Context, open: &mut BTreeSet<String>) {
        let Self {
            about,
            demos,
            tests,
        } = self;
        {
            let mut is_open = open.contains(about.name());
            about.show(ctx, &mut is_open);
            set_open(open, about.name(), is_open);
        }
        demos.windows(ctx, open);
        tests.windows(ctx, open);
    }
}

// ----------------------------------------------------------------------------

/// A menu bar in which you can select different demo windows to show.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Default, Clone)]
struct User {
    username: String,
    password: String,
    accessible_windows: BTreeSet<String>,
    is_admin: bool,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct DemoWindows {
    #[cfg_attr(feature = "serde", serde(skip))]
    groups: DemoGroups,

    open: BTreeSet<String>,

    #[cfg_attr(feature = "serde", serde(skip))]
    users: Vec<User>,

    #[cfg_attr(feature = "serde", serde(skip))]
    logged_in_user: Option<String>,

    #[cfg_attr(feature = "serde", serde(skip))]
    username_input: String,

    #[cfg_attr(feature = "serde", serde(skip))]
    password_input: String,

    #[cfg_attr(feature = "serde", serde(skip))]
    user_management_open: bool,
}

impl Default for DemoWindows {
    fn default() -> Self {
        let mut open = BTreeSet::new();

        // Explains egui very well
        set_open(&mut open, About::default().name(), true);

        // Explains egui very well
        set_open(
            &mut open,
            super::code_example::CodeExample::default().name(),
            true,
        );

        // Shows off the features
        set_open(
            &mut open,
            super::widget_gallery::WidgetGallery::default().name(),
            true,
        );

        let all_windows: BTreeSet<String> = [
            About::default().name(),
            super::code_example::CodeExample::default().name(),
            super::creative_studio::CreativeStudio::default().name(),
            super::widget_gallery::WidgetGallery::default().name(),
            super::window_options::WindowOptions::default().name(),
            super::paint_bezier::PaintBezier::default().name(),
            super::code_editor::CodeEditor::default().name(),
            super::dancing_strings::DancingStrings::default().name(),
            super::drag_and_drop::DragAndDropDemo::default().name(),
            super::extra_viewport::ExtraViewport::default().name(),
            super::font_book::FontBook::default().name(),
            super::frame_demo::FrameDemo::default().name(),
            super::highlighting::Highlighting::default().name(),
            super::interactive_container::InteractiveContainerDemo::default().name(),
            super::MiscDemoWindow::default().name(),
            super::modals::Modals::default().name(),
            super::multi_touch::MultiTouch::default().name(),
            super::painting::Painting::default().name(),
            super::panels::Panels::default().name(),
            super::popups::PopupsDemo::default().name(),
            super::scene::SceneDemo::default().name(),
            super::screenshot::Screenshot::default().name(),
            super::scrolling::Scrolling::default().name(),
            super::sliders::Sliders::default().name(),
            super::strip_demo::StripDemo::default().name(),
            super::table_demo::TableDemo::default().name(),
            super::text_edit::TextEditDemo::default().name(),
            super::text_layout::TextLayoutDemo::default().name(),
            super::tooltips::Tooltips::default().name(),
            super::undo_redo::UndoRedoDemo::default().name(),
            super::ui_widgets::UiWidgets::default().name(),
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let users = vec![
            User {
                username: "admin".to_owned(),
                password: "admin".to_owned(),
                accessible_windows: all_windows,
                is_admin: true,
            },
            User {
                username: "user1".to_owned(),
                password: "password".to_owned(),
                accessible_windows: [
                    super::widget_gallery::WidgetGallery::default().name().to_owned(),
                    About::default().name().to_owned(),
                ]
                .into_iter()
                .collect(),
                is_admin: false,
            },
            User {
                username: "user2".to_owned(),
                password: "password".to_owned(),
                accessible_windows: [
                    super::window_options::WindowOptions::default().name().to_owned(),
                    super::code_example::CodeExample::default().name().to_owned(),
                ]
                .into_iter()
                .collect(),
                is_admin: false,
            },
        ];

        Self {
            groups: Default::default(),
            open,
            users,
            logged_in_user: None,
            username_input: "".to_owned(),
            password_input: "".to_owned(),
            user_management_open: false,
        }
    }
}

impl DemoWindows {
    /// Show the app ui (menu bar and windows).
    pub fn ui(&mut self, ctx: &Context) {
        if is_mobile(ctx) {
            self.mobile_ui(ctx);
        } else {
            self.desktop_ui(ctx);
        }
    }

    fn about_is_open(&self) -> bool {
        self.open.contains(About::default().name())
    }

    fn mobile_ui(&mut self, ctx: &Context) {
        if self.about_is_open() {
            let mut close = false;
            egui::CentralPanel::default().show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink(false)
                    .show(ui, |ui| {
                        self.groups.about.ui(ui);
                        ui.add_space(12.0);
                        ui.vertical_centered_justified(|ui| {
                            if ui
                                .button(egui::RichText::new("Continue to the demo!").size(20.0))
                                .clicked()
                            {
                                close = true;
                            }
                        });
                    });
            });
            if close {
                set_open(&mut self.open, About::default().name(), false);
            }
        } else {
            self.mobile_top_bar(ctx);
            self.groups.windows(ctx, &mut self.open);
        }
    }

    fn mobile_top_bar(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            menu::MenuBar::new()
                .config(menu::MenuConfig::new().style(StyleModifier::default()))
                .ui(ui, |ui| {
                    let font_size = 16.5;

                    ui.menu_button(egui::RichText::new("⏷ demos").size(font_size), |ui| {
                        let mut all_demos = BTreeSet::new();
                        for demo in &self.groups.demos.demos {
                            all_demos.insert(demo.name().to_owned());
                        }
                        for test in &self.groups.tests.demos {
                            all_demos.insert(test.name().to_owned());
                        }
                        all_demos.insert(self.groups.about.name().to_owned());
                        self.demo_list_ui(ui, &all_demos);
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        use egui::special_emojis::GITHUB;
                        ui.hyperlink_to(
                            egui::RichText::new("🦋").size(font_size),
                            "https://bsky.app/profile/ernerfeldt.bsky.social",
                        );
                        ui.hyperlink_to(
                            egui::RichText::new(GITHUB).size(font_size),
                            "https://github.com/emilk/egui",
                        );
                    });
                });
        });
    }

    fn login_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Login");

        ui.horizontal(|ui| {
            ui.label("Username:");
            ui.text_edit_singleline(&mut self.username_input);
        });

        ui.horizontal(|ui| {
            ui.label("Password:");
            ui.add(egui::TextEdit::singleline(&mut self.password_input).password(true));
        });

        if ui.button("Login").clicked() {
            for user in &self.users {
                if self.username_input == user.username && self.password_input == user.password {
                    self.logged_in_user = Some(user.username.clone());
                }
            }
        }
    }

    fn desktop_ui(&mut self, ctx: &Context) {
        egui::SidePanel::right("egui_demo_panel")
            .resizable(false)
            .default_width(160.0)
            .min_width(160.0)
            .show(ctx, |ui| {
                ui.add_space(4.0);
                ui.vertical_centered(|ui| {
                    ui.heading("✒ egui demos");
                });

                ui.separator();

                if let Some(username) = self.logged_in_user.clone() {
                    let user = self.users.iter().find(|u| u.username == username).unwrap().clone();
                    ui.label(format!("Welcome, {}", username));

                    if user.is_admin {
                        if ui.button("User Management").clicked() {
                            self.user_management_open = true;
                        }
                    }

                    if ui.button("Logout").clicked() {
                        self.logged_in_user = None;
                        self.username_input = "".to_owned();
                        self.password_input = "".to_owned();
                    }
                    ui.separator();
                    self.demo_list_ui(ui, &user.accessible_windows);
                } else {
                    self.login_ui(ui);
                }
            });

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            menu::MenuBar::new().ui(ui, |ui| {
                file_menu_button(ui);
            });
        });

        if self.logged_in_user.is_some() {
            self.groups.windows(ctx, &mut self.open);
        }

        self.user_management_window(ctx);
    }

    fn user_management_window(&mut self, ctx: &Context) {
        let mut user_to_delete = None;
        let mut new_user = None;

        if self.user_management_open {
            egui::Window::new("User Management")
                .open(&mut self.user_management_open)
                .vscroll(true)
                .show(ctx, |ui| {
                    ui.heading("Users");

                    for (i, user) in self.users.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(&user.username);
                            if ui.button("Delete").clicked() {
                                user_to_delete = Some(i);
                            }
                        });

                        let all_windows: BTreeSet<String> = self.groups.demos.demos.iter().map(|d| d.name().to_string()).collect();
                        for window_name in all_windows {
                            let mut enabled = user.accessible_windows.contains(&window_name);
                            ui.checkbox(&mut enabled, &window_name);
                            if enabled {
                                user.accessible_windows.insert(window_name.clone());
                            } else {
                                user.accessible_windows.remove(&window_name);
                            }
                        }
                        ui.separator();
                    }

                    ui.heading("Create New User");
                    let mut new_username = String::new();
                    let mut new_password = String::new();
                    ui.horizontal(|ui| {
                        ui.label("Username:");
                        ui.text_edit_singleline(&mut new_username);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Password:");
                        ui.text_edit_singleline(&mut new_password);
                    });
                    if ui.button("Create").clicked() {
                        new_user = Some(User {
                            username: new_username,
                            password: new_password,
                            accessible_windows: BTreeSet::new(),
                            is_admin: false,
                        });
                    }
                });
        }

        if let Some(i) = user_to_delete {
            self.users.remove(i);
        }

        if let Some(user) = new_user {
            self.users.push(user);
        }
    }

    fn demo_list_ui(&mut self, ui: &mut egui::Ui, accessible_windows: &BTreeSet<String>) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                self.groups.checkboxes(ui, &mut self.open, accessible_windows);
                ui.separator();
                if ui.button("Organize windows").clicked() {
                    ui.ctx().memory_mut(|mem| mem.reset_areas());
                }
            });
        });
    }
}

// ----------------------------------------------------------------------------

fn file_menu_button(ui: &mut Ui) {
    let organize_shortcut =
        egui::KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, egui::Key::O);
    let reset_shortcut =
        egui::KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, egui::Key::R);

    // NOTE: we must check the shortcuts OUTSIDE of the actual "File" menu,
    // or else they would only be checked if the "File" menu was actually open!

    if ui.input_mut(|i| i.consume_shortcut(&organize_shortcut)) {
        ui.ctx().memory_mut(|mem| mem.reset_areas());
    }

    if ui.input_mut(|i| i.consume_shortcut(&reset_shortcut)) {
        ui.ctx().memory_mut(|mem| *mem = Default::default());
    }

    ui.menu_button("File", |ui| {
        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);

        // On the web the browser controls the zoom
        #[cfg(not(target_arch = "wasm32"))]
        {
            egui::gui_zoom::zoom_menu_buttons(ui);
            ui.weak(format!(
                "Current zoom: {:.0}%",
                100.0 * ui.ctx().zoom_factor()
            ))
            .on_hover_text("The UI zoom level, on top of the operating system's default value");
            ui.separator();
        }

        if ui
            .add(
                egui::Button::new("Organize Windows")
                    .shortcut_text(ui.ctx().format_shortcut(&organize_shortcut)),
            )
            .clicked()
        {
            ui.ctx().memory_mut(|mem| mem.reset_areas());
        }

        if ui
            .add(
                egui::Button::new("Reset egui memory")
                    .shortcut_text(ui.ctx().format_shortcut(&reset_shortcut)),
            )
            .on_hover_text("Forget scroll, positions, sizes etc")
            .clicked()
        {
            ui.ctx().memory_mut(|mem| *mem = Default::default());
        }
    });
}

#[cfg(test)]
mod tests {
    use crate::{Demo as _, demo::demo_app_windows::DemoGroups};

    use egui_kittest::kittest::{NodeT as _, Queryable as _};
    use egui_kittest::{Harness, OsThreshold, SnapshotOptions, SnapshotResults};

    #[test]
    fn demos_should_match_snapshot() {
        let DemoGroups {
            demos,
            tests,
            about: _,
        } = DemoGroups::default();
        let demos = demos + tests;

        let mut results = SnapshotResults::new();

        for mut demo in demos.demos {
            // Widget Gallery needs to be customized (to set a specific date) and has its own test
            if demo.name() == crate::WidgetGallery::default().name() {
                continue;
            }

            let name = remove_leading_emoji(demo.name());

            let mut harness = Harness::new(|ctx| {
                egui_extras::install_image_loaders(ctx);
                demo.show(ctx, &mut true);
            });

            let window = harness.queryable_node().children().next().unwrap();
            // TODO(lucasmerlin): Windows should probably have a label?
            //let window = harness.get_by_label(name);

            let size = window.rect().size();
            harness.set_size(size);

            // Run the app for some more frames...
            harness.run_ok();

            let mut options = SnapshotOptions::default();

            if name == "Bézier Curve" {
                // The Bézier Curve demo needs a threshold of 2.1 to pass on linux:
                options = options.threshold(OsThreshold::new(0.0).linux(2.1));
            }

            results.add(harness.try_snapshot_options(format!("demos/{name}"), &options));
        }
    }

    fn remove_leading_emoji(full_name: &str) -> &str {
        if let Some((start, name)) = full_name.split_once(' ') {
            if start.len() <= 4 && start.bytes().next().is_some_and(|byte| byte >= 128) {
                return name;
            }
        }
        full_name
    }
}
