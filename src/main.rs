use eframe::{egui, App, CreationContext, Frame};
use rand::prelude::*;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Trawling Demo",
        options,
        Box::new(|_cc: &eframe::CreationContext| {
            let mut locations: Vec<Location> = vec![];
            let mut discovered_locations: Vec<Location> = vec![];
            let home = Location {
                id: 0,
                name: "Home",
                x: 0.0,
                y: 0.0,
                is_shop: false,
                fish_price: None,
                paddle_price: None,
                paddle_upgrade: None,
                net_price: None,
                net_upgrade: None,
            };

            locations.push(home);
            discovered_locations.push(home);

            let shop1 = Location {
                id: 1,
                name: "Isle of Carp",
                x: 10.0,
                y: 0.0,
                is_shop: true,
                fish_price: Some(2.0),
                paddle_price: Some(3.0),
                paddle_upgrade: Some(0.1),
                net_price: Some(5.0),
                net_upgrade: Some(1),
            };

            locations.push(shop1);
            discovered_locations.push(shop1);

            let shop2 = Location {
                id: 2,
                name: "Grubtown",
                x: -25.0,
                y: 40.0,
                is_shop: false,
                fish_price: Some(3.5),
                paddle_price: Some(10.0),
                paddle_upgrade: Some(0.25),
                net_price: Some(25.0),
                net_upgrade: Some(2),
            };

            locations.push(shop2);

            let shop3 = Location {
                id: 3,
                name: "Hookville",
                x: 25.0,
                y: 70.0,
                is_shop: false,
                fish_price: Some(5.0),
                paddle_price: Some(25.0),
                paddle_upgrade: Some(0.5),
                net_price: Some(50.0),
                net_upgrade: Some(3),
            };

            locations.push(shop3);

            let app = MyApp {
                fish: 0,
                money: 0.0,
                trawl_prob: 0.1,
                row_speed: 0.1,
                trawl_efficiency: 1,
                locations: locations,
                current_location: Some(home),
                choosing_location: false,
                x: home.x,
                y: home.y,
                discovered_locations: discovered_locations,
                target_location: None,
                direction: None,
            };

            Ok(Box::new(app))
        }),
    )
}

#[derive(Default, Clone, Copy)]
struct Location<'a> {
    id: i64,
    name: &'a str,
    x: f64,
    y: f64,
    is_shop: bool,
    fish_price: Option<f64>,
    paddle_price: Option<f64>,
    paddle_upgrade: Option<f64>,
    net_price: Option<f64>,
    net_upgrade: Option<i64>,
}

struct Direction {
    x: f64,
    y: f64,
}

#[derive(Default)]
struct MyApp<'a> {
    fish: i64,
    money: f64,
    trawl_prob: f64,
    row_speed: f64,
    trawl_efficiency: i64,
    locations: Vec<Location<'a>>,
    discovered_locations: Vec<Location<'a>>,
    current_location: Option<Location<'a>>,
    x: f64,
    y: f64,
    choosing_location: bool,
    target_location: Option<Location<'a>>,
    direction: Option<Direction>,
}

impl App for MyApp<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        let mut rng = rand::rng();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|vui| {
                vui.heading(format!("Fish: {}", self.fish));
                vui.heading(format!("Money {:.2}", self.money));

                if self.current_location.is_none() {
                    let target_distance = (((self.x - self.target_location.unwrap().x).powf(2.0)
                        + (self.y - self.target_location.unwrap().y).powf(2.0))
                        as f64)
                        .sqrt();
                    vui.heading(format!(
                        "Next Stop: {} -- Distance Until Arrival: {:.2}",
                        self.target_location.unwrap().name,
                        target_distance
                    ));
                    if vui
                        .add_sized([300.0, 75.0], egui::Button::new("Row"))
                        .clicked()
                    {
                        if target_distance <= self.row_speed {
                            self.current_location = self.target_location;
                            self.target_location = None;
                            self.x = self.current_location.unwrap().x;
                            self.y = self.current_location.unwrap().y;
                        } else {
                            self.x += self.row_speed * self.direction.as_ref().unwrap().x;
                            self.y += self.row_speed * self.direction.as_ref().unwrap().y;
                        }

                        if rng.random_bool(self.trawl_prob) {
                            self.fish += self.trawl_efficiency;
                        }
                    }
                } else {
                    let loc = self.current_location.unwrap();

                    if self.choosing_location {
                        for location in self.discovered_locations.iter() {
                            if loc.id == location.id {
                                continue;
                            }

                            let magnitude = (((loc.x - location.x).powf(2.0)
                                + (loc.y - location.y).powf(2.0))
                                as f64)
                                .sqrt();

                            if vui
                                .add_sized(
                                    [300.0, 75.0],
                                    egui::Button::new(format!(
                                        "{} -- Distance: {}",
                                        location.name, magnitude
                                    )),
                                )
                                .clicked()
                            {
                                self.direction = Some(Direction {
                                    x: (location.x - loc.x) as f64 / magnitude,
                                    y: (location.y - loc.y) as f64 / magnitude,
                                });
                                self.current_location = None;
                                self.target_location = Some(*location);
                                self.choosing_location = false;
                            }
                        }
                    } else if loc.is_shop {
                        let paddle_price = loc.paddle_price.unwrap();
                        let net_price = loc.net_price.unwrap();
                        let fish_price = loc.fish_price.unwrap();
                        if vui
                            .add_sized(
                                [300.0, 75.0],
                                egui::Button::new(format!("Buy Paddles ${}", paddle_price)),
                            )
                            .clicked()
                        {
                            if self.money >= paddle_price {
                                self.money -= paddle_price;
                                self.row_speed += loc.paddle_upgrade.unwrap();
                            }
                        }
                        if vui
                            .add_sized(
                                [300.0, 75.0],
                                egui::Button::new(format!("Buy Bigger Net ${}", net_price)),
                            )
                            .clicked()
                        {
                            if self.money >= net_price {
                                self.money -= net_price;
                                self.trawl_efficiency += loc.net_upgrade.unwrap();
                            }
                        }

                        if vui
                            .add_sized(
                                [300.0, 75.0],
                                egui::Button::new(format!("Sell Fish ${}", fish_price)),
                            )
                            .clicked()
                        {
                            if self.fish > 0 {
                                self.money += fish_price;
                                self.fish -= 1;
                            }
                        }

                        if vui
                            .add_sized([300.0, 75.0], egui::Button::new("Choose Destination"))
                            .clicked()
                        {
                            self.choosing_location = true;
                        }
                    } else {
                        if vui
                            .add_sized([300.0, 75.0], egui::Button::new("Choose Destination"))
                            .clicked()
                        {
                            self.choosing_location = true;
                        }
                    }
                }
            });
        });
    }
}
