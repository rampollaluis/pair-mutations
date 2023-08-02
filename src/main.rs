use std::{hash::Hash, collections::HashMap};
use eframe::egui::{self, Ui};
use serde::{Deserialize, Serialize};
use arboard::Clipboard;
use chrono::prelude::*;

use state_persistence::{load_state, save_state};
use pairs_handler::{pairs_to_string, generate_pairs};

mod state_persistence;
mod pairs_handler;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(980.0, 600.0)),
        ..Default::default()
    };

    // Provide a default state if the file doesn't exist or can't be read
    let app_state = match load_state::<MyApp>() {
        Ok(state) => state,
        Err(_) => MyApp::default(), 
    };
    
    eframe::run_native(
        "PairMutations",
        options,
        Box::new(|_cc| Box::new(app_state)),
    )

}

#[derive(Serialize, Deserialize, Debug)]
struct MyApp {
    members: Vec<Member>,
    history: HashMap<String, Vec<Vec<String>>>,
    copied_to_clipboard: bool,
    search: String,
    today_pairs: String,
    show_add_member_dialog: bool,
    new_member: String,
}

#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Member {
    name: String,
    ooo: bool,
    carry: bool,
    solo: bool,
    new: bool,
}

impl Member {
    fn new(name: &str) -> Member {
        Member {
            name: name.to_string(),
            ooo: false,
            carry: false,
            solo: false,
            new: false,
        }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            members: Vec::new(),
            history: HashMap::new(),
            copied_to_clipboard: false,
            search: String::new(),
            today_pairs: String::new(),
            show_add_member_dialog: false,
            new_member: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("leftPanel").resizable(false).show(ctx, |ui| {

            if ui.button("Edit Members").clicked() {
                self.show_add_member_dialog = true;
            }
        });

        if self.show_add_member_dialog {
           self.edit_members_dialog(ctx);
        }

        egui::CentralPanel::default().show(ctx, |ui| {  

            ui.columns(2, |columns| {
                // First column for the members list, cta, and output
                columns[0].heading("Members");
                columns[0].vertical(|ui| {
                    self.members_list(ui);
                });
            
                columns[0].horizontal(|ui| {
                    self.generate_pairs_btn(ui);

                    if ui.button("Save data").clicked() {
                        if let Err(e) = save_state(self) {
                            eprintln!("Error saving data: {}", e);
                        } else {
                            println!("Data saved to file.");
                        }
                    }
                });


                if !self.today_pairs.is_empty() {
                    columns[0].horizontal(|ui| {
                        // TODO: run once and save to variable because this is running every frame. also: refactor
                        let mut pairs_output = self.today_pairs.replace(' ', " 👥");
                        pairs_output = pairs_output.replace('+', "/");
                        pairs_output = format!("{}{}", '👥', pairs_output);
                        ui.label(pairs_output);
                    });

                    columns[0].horizontal(|ui| {
                        self.copy_to_clipboard_btn(ui);
    
                        if self.copied_to_clipboard {
                            ui.label(String::from("Copied!"));
                        }
                    });
                }

                // second column for history
                columns[1].heading("History");
                columns[1].horizontal(|ui| {
                    let search_label = ui.label("Search: ");
                    ui.text_edit_singleline(&mut self.search)
                        .labelled_by(search_label.id);
                });
                columns[1].vertical(|ui| {
                    // Step 1: Collect the HashMap keys and values in a Vec of tuples
                    let mut history_vec: Vec<(&String, &Vec<Vec<String>>)> = self.history.iter().collect();
                
                    // Step 2: Sort the Vec by the keys
                    history_vec.sort_by_key(|k| k.0);
                
                    // Step 3: Iterate over the sorted Vec and display the content in the UI
                    for day in history_vec {
                        let text = format!("{} {}", day.0, pairs_to_string(day.1.to_vec()));
                        if text.contains(&self.search) {
                            ui.label(text);
                        }
                    }
                });
            });
            

        });
    }
}

impl MyApp {
    fn members_list(&mut self, ui: &mut Ui) {
        for member in &mut self.members {
            ui.label(member.name.clone());
            ui.horizontal(|ui| {
                ui.checkbox(&mut member.carry, "Carrying");
                ui.checkbox(&mut member.solo, "Solo");
                ui.checkbox(&mut member.ooo, "Out Of Office");
                ui.checkbox(&mut member.new, "New Member/Intern")
            });

            ui.separator();
        }
    }

    fn edit_members_dialog(&mut self, ctx: &egui::Context) {
        egui::Window::new("Edit Members")
            .title_bar(true)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("Members");
    
                let mut members_to_remove: Vec<usize> = vec![];
    
                ui.horizontal(|ui| {
                    // left half
                    ui.vertical(|ui| {
                        for (index, member) in self.members.iter_mut().enumerate() {
                            ui.horizontal(|ui| {
                                let x_button = egui::Button::new("x")
                                    .fill(egui::Color32::TRANSPARENT)
                                    .frame(false);

                                if ui.add(x_button).clicked() {
                                    members_to_remove.push(index);
                                }
                                ui.label(&member.name);
                            });
                        }
                    });
    
                    // Add space between the two views
                    ui.add_space(f32::from(ui.available_width()) / 2.0 - ui.spacing().item_spacing.x);
    
                    // right half
                    ui.vertical(|ui| {
                        ui.label("Add member");

                        let name_label = ui.label("Name: ");
                        ui.text_edit_singleline(&mut self.new_member)
                            .labelled_by(name_label.id);

                        // TODO: find how to put side by side and _centered_ (ui.with_layout)
                        if ui.button("Cancel").clicked() {
                            self.show_add_member_dialog = false;
                            self.new_member.clear();
                        }

                        if ui.button("Add").clicked() {
                            // self.show_add_member_dialog = false;
                            self.members.push(Member::new(&self.new_member));
                            self.new_member.clear();
                        }

                    });
                });
    
                // Remove members in reverse order to prevent shifting indices
                for index in members_to_remove.into_iter().rev() {
                    self.members.remove(index);
                }
            });
    }
    
    fn generate_pairs_btn(&mut self, ui: &mut Ui) {
        if ui.button("Generate Pairs").clicked() {
            self.copied_to_clipboard = false;
            let pairs = generate_pairs(&self.members, &self.history);
            let today = Utc::now();
            let today = today.format("%Y-%m-%d");
            self.today_pairs = pairs_to_string(pairs.clone());
            self.history.insert(today.to_string(), pairs);
        }
    }

    fn copy_to_clipboard_btn(&mut self, ui: &mut Ui) {
        if ui.button("Copy to clipboard").clicked() {
            let mut clipboard = Clipboard::new().unwrap();
            clipboard.set_text(&self.today_pairs).unwrap();
            self.copied_to_clipboard = true;

            println!("clipboard set to: \"{}\"", &self.today_pairs);   
        }
    }

}

/* TODO:
    Features
        - toml/yml for settings (customize output, auto copy, etc.)
        - allow to manually set pair and roll for rest

        pairs_handler
        - allow triples
        - solo/carry/ooo/new logic

        UI
            Output
            - copy automatically checkbox/setting

            Search
            - ignore caps
            - make search only show that person + who they were paired with that day
            - if you type 2 names, it shows you when they've paired
            - doesnt matter if with spaces, dash, slash, plus
    
    Bugs
        Add Member
        - can add with empty string

        Search
        - not showing solos

    Future
        - set standard group # (eg. triples instead of pairs)
        - pick pair for pre-ipm (track in history which pair hasn't done pre ipm in the longest collectively)
        - pick random (or based on history) member for retro
 */
