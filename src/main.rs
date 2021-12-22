mod ui;

use eframe::{
    egui::{
        CentralPanel, ScrollArea, Vec2, SidePanel,
    },
    epi::App,
    run_native, NativeOptions,
};
use ui::CardData;

impl App for CardData {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        ctx.request_repaint();

        loop {
            match self.message_channel.1.try_recv() {
                Ok(message) => {
                    if let ui::Message::OpenFileDialog(ref x) = message {
                        println!("Open File {}", x.to_string_lossy());
                    }
                    if let ui::Message::SaveFileDialog(ref x) = message {
                        println!("Save File {}", x.to_string_lossy());
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }

        self.render_top_panel(ctx);
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                SidePanel::left("Collection overview panel").resizable(false).min_width(280.).show(ctx, |ui| {
                    ScrollArea::vertical().show(ui, |ui| {
                        self.render_recipe_cards(ui);
                    });
                 });
                 CentralPanel::default().show(ctx, |ui| { 
                    ui.vertical_centered(|ui| { 
                        ScrollArea::vertical().show(ui, |ui| {
                            self.render_detail_view(ui);
                        });
                    });
                 });
            });
        });
        self.render_footer(ctx);
    }

    fn name(&self) -> &str{
        "Rezepthandler"
    }
}

fn main() {
    let app = CardData::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(960., 540.));
    run_native(Box::new(app), win_option);
}
