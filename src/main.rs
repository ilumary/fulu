mod ui;

use eframe::{
    egui::{
        CentralPanel, CtxRef, Hyperlink, ScrollArea, TextStyle, TopBottomPanel,
        Vec2, SidePanel, Layout, Align,
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
            //render_header(ui);
            ui.vertical_centered(|ui| {
                SidePanel::left("my_left_panel").resizable(false).show(ctx, |ui| {
                    ui.label("Hello World!");
                 });
                ScrollArea::vertical().show(ui, |ui| {
                    self.render_recipe_cards(ui);
                });
                
            });
        });
        render_footer(ctx);
    }

    fn name(&self) -> &str{
        "Rezepthandler"
    }
}

pub(crate) fn render_footer(ctx: &CtxRef) {
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

fn main() {
    let app = CardData::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(960., 540.));
    run_native(Box::new(app), win_option);
}
