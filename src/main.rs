use std::fs;

use eframe::egui;
use egui::load::{ImageLoadResult, SizedTexture, TextureLoadResult, TexturePoll};
use egui::{Button, Context, Image, Label, TextureOptions};
use rand::seq::SliceRandom;
use rand::thread_rng;
fn main() -> eframe::Result {
    let w: usize = 4;
    let h: usize = 4;
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "",
        options,
        Box::new(move |cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::new(MyApp {
                board: new_board(w, h),
                first_opened_card: None,
                score: 0,
                imgs: (0..(w * h / 2)).map(|_| None).collect(),
            }))
        }),
    )
}
fn load_imgs(ctx: Context, n: i32) -> Vec<Option<SizedTexture>> {
    let mut vec: Vec<Option<SizedTexture>> = Vec::new();
    //let apikey: String = fs::read_to_string("apikey.txt").unwrap();
    let res = reqwest::blocking::get(
        "https://images-api.nasa.gov/search?q=star&media_type=image&keywords=star&center=GSFC",
    )
    .unwrap()
    .json::<serde_json::Value>()
    .unwrap();
    for i in 0..n {
        println!(
            "{}",
            res["collection"]["items"][i as usize]["links"][0]["href"]
                .as_str()
                .unwrap()
        );
        while true {
            let a = ctx.try_load_texture(
                res["collection"]["items"][i as usize]["links"][0]["href"]
                    .as_str()
                    .unwrap(),
                TextureOptions::default(),
                egui::SizeHint::default(),
            );
            match a.unwrap() {
                TexturePoll::Ready { texture } => {
                    vec.push(Some(texture));
                    break;
                }
                TexturePoll::Pending { size } => (),
            }
        }
    }
    return vec;
}
fn new_board(width: usize, height: usize) -> Vec<Vec<CardInfo>> {
    let mut tst: Vec<Vec<CardInfo>> = Vec::new();
    let nums: Vec<CardInfo> = (0..(width * height / 2))
        .map(|a| CardInfo {
            value: a as i32,
            show_result: false,
            card_cleared: false,
            x: 0,
            y: 0,
        })
        .collect();
    let mut repeated: Vec<CardInfo> = nums
        .into_iter()
        .flat_map(|n| std::iter::repeat(n).take(2))
        .collect();
    repeated.shuffle(&mut thread_rng());
    for i in 0..height {
        tst.push(Vec::new());
        for j in 0..width {
            tst[i].push(CardInfo {
                x: i as i32,
                y: j as i32,
                ..repeated[j + i * width]
            });
        }
    }
    tst
}

#[derive(Copy, Clone)]
struct CardInfo {
    value: i32,
    show_result: bool,
    card_cleared: bool,
    x: i32,
    y: i32,
}

struct MyApp {
    board: Vec<Vec<CardInfo>>,
    first_opened_card: Option<CardInfo>,
    score: i32,
    imgs: Vec<Option<SizedTexture>>,
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if (self.imgs[0].is_none()) {
                self.imgs = load_imgs(
                    ctx.clone(),
                    (self.board.len() * self.board[0].len() / 2) as i32,
                );
            }
            if self.score as usize >= self.board.len() * self.board[0].len() {
                ui.heading("Good job! You've won!");
                if ui.button("Play again").clicked() {
                    self.score = 0;
                    self.board = new_board(self.board.len(), self.board[0].len());
                }
            } else {
                ui.heading("Game");
                egui::Grid::new("grid").show(ui, |ui| {
                    for i in 0..self.board.len() {
                        for j in 0..self.board[0].len() {
                            if self.board[i][j].card_cleared {
                                ui.add_sized([100.0, 100.0], Label::new(""));
                            } else if self.board[i][j].show_result {
                                ui.add_sized(
                                    [100.0, 100.0],
                                    Image::from_texture(
                                        self.imgs[self.board[i][j].value as usize].unwrap(),
                                    )
                                    .max_width(100.0)
                                    .max_height(100.0),
                                );
                            } else if ui.add_sized([100.0, 100.0], Button::new("")).clicked() {
                                match self.first_opened_card {
                                    None => {
                                        self.board = close_cards(self.board.clone());
                                        self.first_opened_card = Some(self.board[i][j]);
                                    }
                                    Some(first_opened_card)
                                        if (first_opened_card.value == self.board[i][j].value) =>
                                    {
                                        self.board[i][j].card_cleared = true;
                                        self.board[first_opened_card.x as usize]
                                            [first_opened_card.y as usize]
                                            .card_cleared = true;
                                        self.score += 2;
                                        self.first_opened_card = None;
                                    }
                                    _ => {
                                        self.first_opened_card = None;
                                    }
                                }
                                self.board[i][j].show_result = true;
                            }
                        }
                        ui.end_row();
                    }
                });
            }
        });
    }
}
fn close_cards(board: Vec<Vec<CardInfo>>) -> Vec<Vec<CardInfo>> {
    let mut tst: Vec<Vec<CardInfo>> = board;
    for i in 0..tst.len() {
        for j in 0..tst[0].len() {
            tst[i][j].show_result = false;
        }
    }
    tst
}
