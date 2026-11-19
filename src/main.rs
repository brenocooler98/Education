#[cfg(not(target_os = "android"))]
use eframe::egui;

#[cfg(not(target_os = "android"))]
fn main() -> eframe::Result<()> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Education")
            .with_inner_size([1100.0, 720.0])
            .with_min_inner_size([640.0, 480.0])
            .with_resizable(true),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Education",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());

            Ok(Box::new(EducationApp::default()))
        }),
    )
}

#[cfg(not(target_os = "android"))]
#[derive(Default)]
struct EducationApp;

#[cfg(not(target_os = "android"))]
impl eframe::App for EducationApp {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);

                ui.heading("Education");

                ui.add_space(10.0);

                ui.label("Welcome to Education!");

                ui.add_space(20.0);

                ui.label("Your learning journey starts here.");

                ui.add_space(30.0);

                if ui.button("Start Learning").clicked() {
                    log::info!("Start Learning clicked");
                }
            });
        });
    }
}

#[cfg(target_os = "android")]
fn main() {}
