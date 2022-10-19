#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console window on Windows in release

use eframe::egui;

fn main() {
	let options = eframe::NativeOptions::default();
	
	eframe::run_native(
		"Nana Editor",
		options,
		Box::new(|_cc| Box::new(MyApp::default())),
	);
}

struct MyApp {
	name: String,
	age: u32,
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
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading(format!("Editing File \"{}\"", self.name));
			
			ui.horizontal(|ui| {
				ui.label("Your name: ");
				ui.text_edit_singleline(&mut self.name);
			});
		});
	}
}
