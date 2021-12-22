use eframe::egui::{self, Button, Color32, CtxRef, Label, Layout, TopBottomPanel, TextStyle, Hyperlink, Align, Separator};

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);

pub enum Message {
    OpenFileDialog(std::path::PathBuf),
    SaveFileDialog(std::path::PathBuf),
}

pub struct CardData {
    current_collection: recipeapi::RecipeCollection,
    current_recipe: Option<recipeapi::Recipe>,
    pub message_channel: (
        std::sync::mpsc::Sender<Message>,
        std::sync::mpsc::Receiver<Message>,
    )
}

impl CardData {
    pub fn new() -> CardData {
        let mut hello = CardData { 
            current_collection: recipeapi::RecipeCollection::create_new_collection("Testing".to_string()),
            current_recipe: None,
            message_channel: std::sync::mpsc::channel(),
        };
        let test = recipeapi::Recipe::recipe_builder("Recipe 1".to_string(), 1, "Description 1".to_string(), std::collections::HashMap::new());
        hello.current_collection.add_recipe(test);
        hello
    }  

    pub fn render_detail_view(&self, ui: &mut eframe::egui::Ui) {
        if self.current_recipe.is_none() {
            ui.add(Label::new("Start by selecting a recipe to show the detailed view").text_style(TextStyle::Heading));
        } else {
            ui.add(Label::new(format!("{}", self.current_recipe.as_ref().unwrap().name())).text_style(TextStyle::Heading));
        }
    }

    pub fn render_recipe_cards(&self, ui: &mut eframe::egui::Ui) {
        for a in self.current_collection.recipes() {
            //TODO check for current recipe and if None render info text
            ui.add_space(PADDING);
            ui.with_layout(Layout::top_down(egui::Align::Min), |ui| {
                let title = format!("{}", a.name()); 
                ui.colored_label(WHITE, title)
            });
            // Show Button
            ui.with_layout(Layout::top_down(egui::Align::Max), |ui| {
                let _show_btn = ui.add(Button::new("SELECT").text_style(egui::TextStyle::Button).frame(true).text_color(Color32::from_rgb(0, 190, 220)));
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
        ui.add_space(PADDING);
        //Add, Edit, Delete current Recipe Button 
        ui.vertical_centered( |ui| {
            ui.with_layout(Layout::top_down(egui::Align::Min), |ui| {
                let _add_btn = ui.add(Button::new("ADD").text_style(egui::TextStyle::Button).text_color(Color32::from_rgb(0, 190, 220)));
                let _edit_btn = ui.add(Button::new("EDIT").text_style(egui::TextStyle::Button).text_color(Color32::from_rgb(0, 190, 220)));
                let _delete_btn = ui.add(Button::new("DELETE").text_style(egui::TextStyle::Button).text_color(Color32::from_rgb(0, 190, 220)));
            });
        });
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

    pub fn render_footer(&self, ctx: &CtxRef) {
        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.with_layout(Layout::bottom_up(Align::Center) ,|ui| {
                ui.add_space(10.);
                ui.add(
                    Hyperlink::new("https://github.com/ilumary")
                        .text("github/ilumary")
                        .text_style(TextStyle::Monospace),
                );
                ui.add(
                    Hyperlink::new("https://github.com/emilk/egui")
                        .text("Made with egui")
                        .text_style(TextStyle::Monospace),
                );
                ui.add_space(10.);
            })
        });
    }
}

pub fn execute<F: std::future::Future<Output = ()> + Send + 'static>(f: F) {
    std::thread::spawn(move || {
        futures::executor::block_on(f);
    });
}