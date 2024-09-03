use std::cell::RefCell;

use eframe::{
    egui::{self, Context, Label, RichText},
    Frame, NativeOptions,
};

use sha2::{Digest, Sha512};

#[derive(Debug)]
struct Account {
    username: String,
    password_hash: String,
}

impl Account {
    fn new(username: impl ToString, password_hash: impl ToString) -> Self {
        Self {
            username: username.to_string(),
            password_hash: password_hash.to_string(),
        }
    }
}

struct Application {
    account: Option<RefCell<Account>>,
    login_window_username: String,
    login_window_password: String,
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if self.is_logged_in() {
            egui::TopBottomPanel::top("top").show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("Account", |ui| {
                        ui.label(&self.account.as_mut().unwrap().get_mut().username);

                        ui.separator();

                        if ui.button("Log out").clicked() {
                            ui.close_menu();
                            self.logout();
                        }
                    });
                });
            });
        }

        if !self.is_logged_in() {
            egui::Window::new("Login").show(ctx, |ui| {
                ui.heading("Login");

                ui.horizontal(|ui| {
                    ui.label("Username: ");
                    ui.text_edit_singleline(&mut self.login_window_username);
                });

                ui.horizontal(|ui| {
                    ui.label("Password: ");
                    ui.text_edit_singleline(&mut self.login_window_password);
                });

                if ui.button("Login").clicked() {
                    self.login(
                        self.login_window_username.clone(),
                        self.login_window_password.clone(),
                    );

                    self.login_window_username.clear();
                    self.login_window_password.clear();
                }
            });
        }

        if self.is_logged_in() {
            egui::Window::new("Account Info").show(ctx, |ui| {
                let account = self.account_mut().unwrap();
                ui.heading("Account Info");

                ui.horizontal(|ui| {
                    ui.label("Username: ");
                    ui.add(Label::new(RichText::new(&account.username).small().monospace()).wrap());
                });

                ui.horizontal(|ui| {
                    ui.label("Password Hash: ");
                    ui.add(
                        Label::new(RichText::new(&account.password_hash).small().monospace())
                            .wrap(),
                    );
                });
            });
        }
    }
}

impl Application {
    fn new() -> Self {
        Self {
            account: None,
            login_window_username: String::new(),
            login_window_password: String::new(),
        }
    }

    fn login(&mut self, username: impl ToString, password: impl ToString) {
        self.account = Some(RefCell::new(Account::new(
            username,
            Self::hash(password.to_string().as_bytes()),
        )));
    }

    fn hash(buffer: &[u8]) -> String {
        let mut hasher = Sha512::new();
        hasher.update(buffer);
        let hash = hasher.finalize();
        format!("{:x}", hash)
    }

    fn logout(&mut self) {
        self.account = None;
    }

    fn is_logged_in(&self) -> bool {
        self.account.is_some()
    }

    fn account_mut(&mut self) -> Option<&mut Account> {
        match self.account.as_mut() {
            Some(account) => Some(account.get_mut()),
            None => None,
        }
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "Login Test",
        NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(Application::new()))),
    )
}
