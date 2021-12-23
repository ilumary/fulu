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
            ui.add_space(PADDING);
            ui.with_layout(Layout::top_down(egui::Align::Min), |ui| {
                let title = format!("{}", a.name()); 
                ui.colored_label(WHITE, title);
            });
            // Show Button
            ui.with_layout(Layout::top_down(egui::Align::Max), |ui| {
                let _show_btn = ui.add(Button::new("SELECT").text_style(egui::TextStyle::Button).frame(true).text_color(Color32::from_rgb(0, 190, 220)));
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }

        if self.current_collection.is_empty() {
            ui.add_space(PADDING * 2.);
            ui.colored_label(WHITE, "Start by importing collection");
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }

        ui.add_space(PADDING);
        //Add, Edit, Delete current Recipe Button 
        ui.vertical_centered( |ui| {
            ui.with_layout(Layout::top_down(egui::Align::Min), |ui| {
                let add_btn = ui.add(Button::new("ADD").text_style(egui::TextStyle::Button).text_color(Color32::from_rgb(0, 190, 220)));
                let edit_btn = ui.add(Button::new("EDIT").text_style(egui::TextStyle::Button).text_color(Color32::from_rgb(0, 190, 220)));
                let delete_btn = ui.add(Button::new("DELETE").text_style(egui::TextStyle::Button).text_color(Color32::from_rgb(0, 190, 220)));

                if add_btn.clicked() { self.action_add_btn(); }
                if edit_btn.clicked() { self.action_edit_btn(); }
                if delete_btn.clicked() { self.action_delete_btn(); }
            });
        });
    }

    pub(crate) fn render_top_panel(&mut self, ctx: &CtxRef) {
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
                    let close_btn = ui.add(Button::new("❌").text_style(egui::TextStyle::Body));
                    //refactor refresh to "create" to create new collection
                    let refresh_btn = ui.add(Button::new("🔄").text_style(egui::TextStyle::Body));
                    let save_btn = ui.add(Button::new("💾").text_style(egui::TextStyle::Body));
                    let load_btn = ui.add(Button::new("📁").text_style(egui::TextStyle::Body));
                    
                    if close_btn.clicked() { self.action_close_btn(); }
                    if refresh_btn.clicked() { self.action_refresh_btn(); }
                    if load_btn.clicked() { self.action_open_btn(); }
                    if save_btn.clicked() { self.action_save_btn(); }
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

    fn action_save_btn(&self) {
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

    fn action_open_btn(&self) {
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

    fn action_close_btn(&mut self) {
        self.current_recipe = None;
    }

    fn action_refresh_btn(&self) {}

    fn action_add_btn(&self) {}
    fn action_edit_btn(&self) {}
    fn action_delete_btn(&self) {}
}

pub fn execute<F: std::future::Future<Output = ()> + Send + 'static>(f: F) {
    std::thread::spawn(move || {
        futures::executor::block_on(f);
    });
}