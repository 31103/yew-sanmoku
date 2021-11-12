use crate::board::Board;
use crate::info::Info;
use std::fmt;
use yew::prelude::*;

#[derive(Clone, PartialEq, Copy)]
pub enum Square {
    None,
    X,
    O,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Square::None => write!(f, ""),
            Square::X => write!(f, "X"),
            Square::O => write!(f, "O"),
        }
    }
}

/// 盤面上の位置を示す際に使用する zero-origin
#[derive(Clone, PartialEq)]
pub struct Point {
    pub col: usize, // 行
    pub row: usize, // 列
}

/// 盤面の状態の遷移を格納する`struct`
#[derive(Clone, PartialEq)]
pub struct History {
    /// 盤面の状態
    pub squares: Vec<Square>,

    /// どこにOXを打ったか
    pub point: Point,
}

pub struct Game {
    link: ComponentLink<Self>,
    history: Vec<History>,
    x_is_next: bool,
    step_number: usize,
}

pub enum Msg {
    Click(usize),
    Jump(usize),
}

impl Component for Game {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            history: vec![History {
                squares: vec![Square::None; 9],
                point: Point { col: 0, row: 0 },
            }],
            x_is_next: true,
            step_number: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click(i) => {
                // プレイ時点を示す`step_number`よりも未来の盤面を削除する。
                self.history.truncate(self.step_number + 1);

                let mut current = self.history.iter().last().unwrap().clone();

                // 勝敗が決しているか、既に打たれている箇所であれば、状態変更できないため
                // 早期returnする。
                if calculate_winner(&current.squares).is_some()
                    || current.squares[i] != Square::None
                {
                    return false;
                }

                // 盤面を更新し、打点を記録する。
                current.squares[i] = if self.x_is_next { Square::X } else { Square::O };
                current.point.col = i % 3;
                current.point.row = i / 3;

                // 現在の盤面を`history`に`push`する。
                self.history.push(current);

                self.x_is_next = !self.x_is_next;
                self.step_number += 1;

                true
            }
            Msg::Jump(step) => {
                self.step_number = step;
                self.x_is_next = (step % 2) == 0;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let history = &self.history;
        let current = history[self.step_number].clone();

        html! {
            <div class="game">
                <Board
                    squares=current.squares.clone()
                    on_click=self.link.callback(move |i:usize|Msg::Click(i))
                />

                <Info
                    history=history.clone()
                    step_number=self.step_number
                    x_is_next=self.x_is_next
                    jump=self.link.callback(move |i:usize|Msg::Jump(i))
                />
            </div>
        }
    }
}

/// 与えられた盤面の勝者を判定する関数。
/// 勝者がいなければ、`None`を返す。
pub fn calculate_winner(squares: &Vec<Square>) -> Option<Square> {
    let lines = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];
    for i in 0..lines.len() {
        let [a, b, c] = lines[i];
        if squares[a] != Square::None && squares[a] == squares[b] && squares[a] == squares[c] {
            return Some(squares[a]);
        }
    }
    None
}
