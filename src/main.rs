use eframe::{egui, App, CreationContext, Frame};
use rand::prelude::*;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Trawling Demo",
        options,
        Box::new(|_cc: &eframe::CreationContext| {
            let mut app = MyApp {
                distance: 2.0,
                fish: 0,
                money: 0.0,
                trawl_prob: 0.1,
                first_shop: 10.0,
                past_first: false,
                second_shop: 50.0,
                past_second: false,
                third_shop: 100.0,
                past_third: false,
                row_speed: 0.1,
                trawl_efficiency: 1,
            };

            Ok(Box::new(app))
        }),
    )
}


#[derive(Default)]
struct MyApp {
    distance: f64,
    fish: i64,
    money: f64,
    trawl_prob: f64,
    row_speed: f64,
    trawl_efficiency: i64,
    first_shop: f64,
    past_first: bool,
    second_shop: f64,
    past_second: bool,
    third_shop: f64,
    past_third: bool,
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        let mut rng = rand::rng();
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.label("Enter your name:");
            // ui.text_edit_singleline(&mut self.name);

            ui.vertical_centered(|vui| {
                vui.heading(format!("Distance Traveled: {:.2}", self.distance));
                vui.heading(format!("Fish Caught: {}", self.fish));
                vui.heading(format!("Money {:.2}", self.money));

                if self.distance >= self.first_shop && !self.past_first {
                    if vui.add_sized([300.0, 75.0], egui::Button::new("Buy Paddles $3")).clicked() {
                        if self.money >= 3.0 {
                            self.row_speed += 0.1;
                            self.money -= 3.0;
                        }
                    }
                    if vui.add_sized([300.0, 75.0], egui::Button::new("Buy Bigger Net $5")).clicked() {
                        if self.money >= 5.0 {
                            self.money -= 5.0;
                            self.trawl_efficiency += 1;
                        }
                    }

                    if vui.add_sized([300.0, 75.0], egui::Button::new("Sell Fish $2")).clicked() {
                        if self.fish > 0 {
                            self.money += 2.0;
                            self.fish -= 1;
                        }
                    }

                    if vui.add_sized([300.0, 75.0], egui::Button::new("Continue Rowing")).clicked() {
                        self.past_first = true;
                    }

                } else if self.distance >= self.second_shop && !self.past_second {
                    if vui.add_sized([300.0, 75.0], egui::Button::new("Buy Paddles $10")).clicked() {
                        if self.money >= 10.0 {
                            self.row_speed += 0.25;
                            self.money -= 10.0;
                        }
                    }
                    if vui.add_sized([300.0, 75.0], egui::Button::new("Buy Bigger Net $25")).clicked() {
                        if self.money >= 25.0 {
                            self.money -= 25.0;
                            self.trawl_efficiency += 2;
                        }
                    }

                    if vui.add_sized([300.0, 75.0], egui::Button::new("Sell Fish $3.50")).clicked() {
                        if self.fish > 0 {
                            self.money += 3.5;
                            self.fish -= 1;
                        }
                    }

                    if vui.add_sized([300.0, 75.0], egui::Button::new("Continue Rowing")).clicked() {
                        self.past_second = true;
                    }

                } else if self.distance >= self.third_shop && !self.past_third {
                    if vui.add_sized([300.0, 75.0], egui::Button::new("Buy Paddles $25")).clicked() {
                        if self.money >= 25.0 {
                            self.row_speed += 0.5;
                            self.money -= 25.0;
                        }
                    }
                    if vui.add_sized([300.0, 75.0], egui::Button::new("Buy Bigger Net $50")).clicked() {
                        if self.money >= 50.0 {
                            self.money -= 50.0;
                            self.trawl_efficiency += 3;
                        }
                    }

                    if vui.add_sized([300.0, 75.0], egui::Button::new("Sell Fish $5")).clicked() {
                        if self.fish > 0 {
                            self.money += 5.0;
                            self.fish -= 1;
                        }
                    }

                    if vui.add_sized([300.0, 75.0], egui::Button::new("Continue Rowing")).clicked() {
                        self.past_third = true;
                    }

                } else if vui.add_sized([300.0, 75.0], egui::Button::new("Row")).clicked() {
                    self.distance += self.row_speed;
                    if rng.random_bool(self.trawl_prob) {
                        self.fish += self.trawl_efficiency;
                    }
                }

            }
                
            );

            // ui.label(format!("Hello {}, counter = {}", self.name, self.counter));
        });
    }
}
