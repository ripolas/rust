use eframe::egui;
use egui::{Button, Label};
use rand::seq::SliceRandom;
use rand::thread_rng;
fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::new(MyApp {
                board: new_board(4, 4),
                cards_open: 0,
                first_opened_card: CardInfo {
                    value: -1,
                    show_result: false,
                    card_cleared: false,
                    x: 0,
                    y: 0,
                },
                score: 0,
            }))
        }),
    )
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
    cards_open: i32,
    first_opened_card: CardInfo,
    score: i32,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.score as usize >= self.board.len() * self.board[0].len() {
                ui.heading("Good job! You've won!");
                if ui.button("Play again").clicked() {
                    self.score = 0;
                    self.cards_open = 0;
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
                                // ui.add_sized(
                                //     [100.0, 100.0],
                                //     Label::new(format!("{}", self.board[i][j].value)),
                                // );
                                ui.add_sized(
                                    [100.0, 100.0],
                                    egui::Image::from_uri(format!(
                                        "file://../imgs/{}.jpg",
                                        (self.board[i][j].value + 1)
                                    )),
                                );
                                //ui.image(egui::include_image!("../imgs/1.jpg"));
                            } else if ui.add_sized([100.0, 100.0], Button::new("")).clicked() {
                                if self.cards_open >= 2 {
                                    self.cards_open = 0;
                                    self.board = close_cards(self.board.clone());
                                }
                                self.board[i][j].show_result = true;
                                self.cards_open += 1;
                                if self.cards_open == 1 {
                                    self.first_opened_card = self.board[i][j];
                                }
                                if self.cards_open == 2
                                    && (self.first_opened_card.value == self.board[i][j].value)
                                {
                                    self.board[i][j].card_cleared = true;
                                    self.board[self.first_opened_card.x as usize]
                                        [self.first_opened_card.y as usize]
                                        .card_cleared = true;
                                    self.score += 2;
                                }
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
