use egui::Slider;
use egui::Vec2;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PdfViewer {
    current_uri: String,
    uri_edit_text: String,
    page: usize,
    scale: f32,
    max_size: Vec2,
    alt_text: String,
}

impl Default for PdfViewer {
    fn default() -> Self {
        Self {
            // Point to the CV image in the web static directory with correct path
            current_uri: "static/images/Aditya_Verma_CV.png".to_owned(),
            uri_edit_text: "static/images/Aditya_Verma_CV.png".to_owned(),
            page: 1,
            scale: 1.0,
            max_size: Vec2::splat(2048.0),
            alt_text: "Aditya Verma CV".to_owned(),
        }
    }
}

impl eframe::App for PdfViewer {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::TopBottomPanel::new(egui::panel::TopBottomSide::Top, "url bar").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                let label = ui.label("Image URI:");
                ui.text_edit_singleline(&mut self.uri_edit_text)
                    .labelled_by(label.id);
                if ui.small_button("✔").clicked() {
                    ctx.forget_image(&self.current_uri);
                    self.uri_edit_text = self.uri_edit_text.trim().to_owned();
                    self.current_uri = self.uri_edit_text.clone();
                };

                #[cfg(all(feature = "image_viewer", not(target_arch = "wasm32")))]
                if ui.button("file…").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.uri_edit_text = format!("file://{}", path.display());
                        self.current_uri = self.uri_edit_text.clone();
                    }
                }
            });
        });

        egui::SidePanel::new(egui::panel::Side::Left, "controls").show(ctx, |ui| {
            // Page selection (PDFs have multiple pages, but we're using a single image)
            ui.label("Page");
            ui.add(Slider::new(&mut self.page, 1..=1).text("page"));

            // Scale
            ui.add_space(5.0);
            ui.label("Scale");
            ui.add(Slider::new(&mut self.scale, 0.1..=4.0).text("scale"));

            // max size
            ui.add_space(5.0);
            ui.label("The calculated size will not exceed the maximum size");
            ui.add(Slider::new(&mut self.max_size.x, 0.0..=2048.0).text("width"));
            ui.add(Slider::new(&mut self.max_size.y, 0.0..=2048.0).text("height"));

            // alt text
            ui.add_space(5.0);
            ui.label("Alt text");
            ui.text_edit_singleline(&mut self.alt_text);

            // forget all images
            if ui.button("Forget all images").clicked() {
                ui.ctx().forget_all_images();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                // Display the CV image
                let mut image = egui::Image::from_uri(&self.current_uri);
                image = image.fit_to_original_size(self.scale);
                image = image.max_size(self.max_size);
                if !self.alt_text.is_empty() {
                    image = image.alt_text(&self.alt_text);
                }

                ui.add_sized(ui.available_size(), image);
                
                ui.add_space(10.0);
                ui.label("Note: This is a PDF viewer that displays a pre-converted image of the PDF.");
                ui.label("In a complete implementation, PDF pages would be rendered dynamically.");
                ui.label(format!("Scale: {:.1}", self.scale));
            });
        });
    }
}