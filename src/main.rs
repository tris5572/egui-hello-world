#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Windows のリリースビルド時にコンソールウィンドウを隠す
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui::{self, FontData, FontDefinitions, FontFamily, TextEdit};

fn main() -> eframe::Result {
    // ログ出力は行わないため、コメントアウト
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // 画像表示を行うための準備
            egui_extras::install_image_loaders(&cc.egui_ctx);

            // フォント追加のために new を使用する
            Ok(Box::new(MyApp::new(cc)))
            // Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    name: String,
    age: u32,
}

impl MyApp {
    // カスタムフォントを追加するために定義
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "my_font".to_owned(),
            FontData::from_static(include_bytes!("../assets/fonts/NotoSansJP-Regular.ttf")),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "my_font".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        Self::default()
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // let text_input = TextEdit::singleline(&mut self.name);
        // text_input.return_key(None);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name(名前): ");
                // ui.text_edit_singleline(&mut self.name)
                //     .labelled_by(name_label.id);
                // エンターキーを押下してもフォーカスが外れないように設定したテキストフィールドを配置
                ui.add(TextEdit::singleline(&mut self.name).return_key(None))
                    .labelled_by(name_label.id)
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            ui.image(egui::include_image!("../assets/ferris.png"));
        });
    }
}
