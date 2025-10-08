use iced::{
    application,
    widget::{button, column, container, pick_list, row, text, text_input, Space},
    window::Settings,
    Element, Fill, Font, Size,
};

#[derive(Default)]
struct Counter {
    selected_shuxin: Option<Shuxin>,
    mianban: String,
    gjl: String,
    out_table: String,
}

#[derive(Debug, Clone)]
enum Message {
    ShuxinSelected(Shuxin),
    MianbanChanged(String),
    GjlChanged(String),
    CalculatorClick,
}

impl Counter {
    fn view(&self) -> Element<'_, Message> {
        let text_in1: text_input::TextInput<'_, Message> =
            text_input("技能面板", &self.mianban).on_input(Message::MianbanChanged);
        let text_in2: text_input::TextInput<'_, Message> =
            text_input("最终攻击力", &self.gjl).on_input(Message::GjlChanged);
        let shuxin: pick_list::PickList<'_, Shuxin, &[Shuxin], Shuxin, Message> = pick_list(
            &Shuxin::ALL[..],
            self.selected_shuxin,
            Message::ShuxinSelected,
        );
        container(
            column![
                Space::with_height(0),
                row![
                    Space::with_height(0),
                    text("技能面板："),
                    text_in1,
                    text("最终攻击力："),
                    text_in2,
                    Space::with_height(0)
                ]
                .spacing(20.0),
                row![
                    Space::with_height(0),
                    text("内外功选项："),
                    shuxin,
                    button("开始计算").on_press(Message::CalculatorClick),
                    text(self.out_table.clone()),
                    Space::with_height(0)
                ]
                .spacing(20.0),
                row![Space::with_height(0),text("bug联系：qinthirteen@outlook.com"),Space::with_height(0)].spacing(20.0),
                Space::with_height(0),
            ]
            .spacing(20.0),
        )
        .center_x(Fill)
        .center_y(Fill)
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ShuxinSelected(shuxin) => {
                self.selected_shuxin = Some(shuxin);
            }
            Message::MianbanChanged(mianban) => {
                self.mianban = mianban;
            }
            Message::GjlChanged(gjl) => {
                self.gjl = gjl;
            }
            Message::CalculatorClick => {
                let y: u8 = if self
                    .selected_shuxin
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_default()
                    == "外功"
                {
                    160
                } else {
                    192
                };
                let x1: u32 = self.mianban.parse().unwrap();
                let x2: u32 = self.gjl.parse().unwrap();
                let out1: f32 = x1 as f32 / x2 as f32;
                let out2: u16 = ((out1 * y as f32) + 0.5) as u16;
                self.out_table = "技能倍率为：".to_string() + &out2.to_string();
            }
        }
    }
}
fn main() -> iced::Result {
    application(
        "简易的技能倍率计算器v1.0.0 ｜ 作者：昭弦",
        Counter::update,
        Counter::view,
    )
    .font(include_bytes!("../res/SourceHanSansSC-Regular.otf"))
    .default_font(Font::with_name("思源黑体"))
    .window(Settings {
        size: Size::new(500.0, 160.0),
        resizable: false,
        ..Settings::default()
    })
    .run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Shuxin {
    #[default]
    Ng,
    Wg,
}

impl Shuxin {
    const ALL: [Shuxin; 2] = [Shuxin::Ng, Shuxin::Wg];
}

impl std::fmt::Display for Shuxin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Shuxin::Ng => "内功",
                Shuxin::Wg => "外功",
            }
        )
    }
}
