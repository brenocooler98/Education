use eframe::egui;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppScreen {
    Home,
    Subjects,
    Lessons,
    Exercises,
    Settings,
}

impl AppScreen {
    pub fn title(&self) -> &'static str {
        match self {
            Self::Home => "Home",
            Self::Subjects => "Subjects",
            Self::Lessons => "Lessons",
            Self::Exercises => "Exercises",
            Self::Settings => "Settings",
        }
    }
}

pub struct EducationApp {
    screen: AppScreen,
    dark_mode: bool,
    username: String,
    level: u32,
    xp: u32,
    completed_lessons: u32,
}

impl Default for EducationApp {
    fn default() -> Self {
        Self {
            screen: AppScreen::Home,
            dark_mode: true,
            username: "Student".to_owned(),
            level: 1,
            xp: 0,
            completed_lessons: 0,
        }
    }
}

impl EducationApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self::default()
    }

    pub fn navigate(&mut self, screen: AppScreen) {
        self.screen = screen;
    }

    fn navigation_button(
        &mut self,
        ui: &mut egui::Ui,
        screen: AppScreen,
    ) {
        let selected = self.screen == screen;

        if ui.selectable_label(selected, screen.title()).clicked() {
            self.navigate(screen);
        }
    }

    fn show_navigation(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("education_navigation")
            .default_width(190.0)
            .min_width(150.0)
            .show(ctx, |ui| {
                ui.add_space(16.0);
                ui.heading("Education");
                ui.label("Learning Platform");

                ui.separator();
                ui.add_space(12.0);

                self.navigation_button(ui, AppScreen::Home);
                self.navigation_button(ui, AppScreen::Subjects);
                self.navigation_button(ui, AppScreen::Lessons);
                self.navigation_button(ui, AppScreen::Exercises);
                self.navigation_button(ui, AppScreen::Settings);

                ui.separator();
                ui.label(format!("Level {}", self.level));
                ui.label(format!("XP: {}", self.xp));
            });
    }

    fn show_top_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("education_top_bar")
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading(self.screen.title());

                    ui.with_layout(
                        egui::Layout::right_to_left(
                            egui::Align::Center
                        ),
                        |ui| {
                            ui.label(&self.username);
                        },
                    );
                });
            });
    }

    fn show_home(&mut self, ui: &mut egui::Ui) {
        ui.heading(format!("Welcome, {}!", self.username));
        ui.label("Your learning journey starts here.");

        ui.add_space(20.0);

        ui.group(|ui| {
            ui.heading("Your Progress");
            ui.label(format!("Level: {}", self.level));
            ui.label(format!("Experience: {} XP", self.xp));
            ui.label(format!(
                "Completed Lessons: {}",
                self.completed_lessons
            ));

            let progress = (self.xp % 100) as f32 / 100.0;

            ui.add(
                egui::ProgressBar::new(progress)
                    .show_percentage()
            );
        });

        ui.add_space(20.0);

        if ui.button("Explore Subjects").clicked() {
            self.navigate(AppScreen::Subjects);
        }

        if ui.button("Continue Learning").clicked() {
            self.navigate(AppScreen::Lessons);
        }
    }

    fn show_subjects(&mut self, ui: &mut egui::Ui) {
        ui.heading("Subjects");
        ui.label("Choose a subject to start learning.");

        ui.add_space(16.0);

        let subjects = [
            "Mathematics",
            "Portuguese",
            "Science",
            "History",
            "Geography",
            "Programming",
        ];

        for subject in subjects {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label(subject);

                    if ui.button("Explore").clicked() {
                        self.navigate(AppScreen::Lessons);
                    }
                });
            });
        }
    }

    fn show_lessons(&mut self, ui: &mut egui::Ui) {
        ui.heading("Lessons");
        ui.label("Explore educational lessons.");

        ui.add_space(16.0);
        ui.label("Lessons will be available soon.");
    }

    fn show_exercises(&mut self, ui: &mut egui::Ui) {
        ui.heading("Exercises");
        ui.label("Practice what you have learned.");

        ui.add_space(16.0);
        ui.label("Exercises will be available soon.");
    }

    fn show_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        ui.add_space(16.0);

        ui.label("Profile");
        ui.text_edit_singleline(&mut self.username);

        ui.separator();

        if ui.checkbox(&mut self.dark_mode, "Dark Mode").changed() {
            let visuals = if self.dark_mode {
                egui::Visuals::dark()
            } else {
                egui::Visuals::light()
            };

            ui.ctx().set_visuals(visuals);
        }

        ui.separator();

        ui.label(format!(
            "Education v{}",
            env!("CARGO_PKG_VERSION")
        ));
    }

    fn show_content(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(16.0);

                match self.screen {
                    AppScreen::Home => self.show_home(ui),
                    AppScreen::Subjects => self.show_subjects(ui),
                    AppScreen::Lessons => self.show_lessons(ui),
                    AppScreen::Exercises => self.show_exercises(ui),
                    AppScreen::Settings => self.show_settings(ui),
                }
            });
        });
    }
}

impl eframe::App for EducationApp {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        self.show_top_bar(ctx);

        let mobile_layout = ctx.screen_rect().width() < 650.0;

        if mobile_layout {
            egui::TopBottomPanel::bottom("mobile_navigation")
                .show(ctx, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        self.navigation_button(ui, AppScreen::Home);
                        self.navigation_button(ui, AppScreen::Subjects);
                        self.navigation_button(ui, AppScreen::Lessons);
                        self.navigation_button(ui, AppScreen::Exercises);
                        self.navigation_button(ui, AppScreen::Settings);
                    });
                });
        } else {
            self.show_navigation(ctx);
        }

        self.show_content(ctx);
    }
}
