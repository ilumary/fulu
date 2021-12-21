use eframe::egui::{self, Button, Color32, CtxRef, Label, Layout, TopBottomPanel};

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);

pub enum Message {
    OpenFileDialog(std::path::PathBuf),
    SaveFileDialog(std::path::PathBuf),
}

pub struct CardData {
    fields: recipeapi::RecipeCollection,
    pub message_channel: (
        std::sync::mpsc::Sender<Message>,
        std::sync::mpsc::Receiver<Message>,
    )
}

impl CardData {
    pub fn new() -> CardData {
        let mut hello = CardData { 
            fields: recipeapi::RecipeCollection::create_new_collection("Testing".to_string()),
            message_channel: std::sync::mpsc::channel(),
        };
        let test = recipeapi::Recipe::recipe_builder("Recipe 1".to_string(), 1, "Description 1".to_string(), std::collections::HashMap::new());
        hello.fields.add_recipe(test);
        hello
    }  

    pub fn render_recipe_cards(&self, ui: &mut eframe::egui::Ui) {
        for a in self.fields.recipes() {
            ui.add_space(PADDING);
            // render title
            let title = format!("{}", a.name());
            ui.colored_label(WHITE, title);
            // render desc
            ui.add_space(PADDING);
            let desc = Label::new(&a.description());
            ui.add(desc);
        }
    }

    pub(crate) fn render_top_panel(&self, ctx: &CtxRef) {
        // define a TopBottomPanel widget
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.);
            egui::menu::bar(ui, |ui| {
                // logo
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.add(Label::new("📓").text_style(egui::TextStyle::Heading));
                });
                // controls
                ui.with_layout(Layout::right_to_left(), |ui| {
                    let _close_btn = ui.add(Button::new("❌").text_style(egui::TextStyle::Body));
                    let _refresh_btn = ui.add(Button::new("🔄").text_style(egui::TextStyle::Body));
                    let save_btn = ui.add(Button::new("💾").text_style(egui::TextStyle::Body));
                    let load_btn = ui.add(Button::new("📁").text_style(egui::TextStyle::Body));
                    
                    if load_btn.clicked() {
                        let task = rfd::AsyncFileDialog::new()
                            .add_filter("Toml Files", &["toml"])
                            .set_directory("~")
                            .pick_file();
        
                        let message_sender = self.message_channel.0.clone();
        
                        execute(async move {
                            let file = task.await;
        
                            if let Some(file) = file {
                                let file_path = file.path().to_path_buf();
                                message_sender.send(Message::OpenFileDialog(file_path)).ok();
                            }
                        });
        
                    }

                    if save_btn.clicked() {
                        let task = rfd::AsyncFileDialog::new()
                            .set_directory("~")
                            .save_file();
        
                        let message_sender = self.message_channel.0.clone();
        
                        execute(async move {
                            let file = task.await;
        
                            if let Some(file) = file {
                                let file_path = file.path().to_path_buf();
                                message_sender.send(Message::SaveFileDialog(file_path)).ok();
                            }
                        });
        
                    }
                });
            });
            ui.add_space(10.);
        });
    }
}

pub fn execute<F: std::future::Future<Output = ()> + Send + 'static>(f: F) {
    std::thread::spawn(move || {
        futures::executor::block_on(f);
    });
}