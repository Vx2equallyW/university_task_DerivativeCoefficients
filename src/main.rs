use iced::{Sandbox};
use iced::widget::{container, text, row, column, text_input, button};

struct App {
    power: i32,
    power_input: String,
    ratios_vec: Vec<i32>,
    ratios_vec_input: String,
    derivative_coefficients: String
}

#[derive(Debug, Clone)]
enum Message {
    PowerInput(String),
    RatiosInput(String),
    Submitted,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            power: 0,
            power_input: String::default(),
            ratios_vec: vec![],
            ratios_vec_input: String::default(),
            derivative_coefficients: String::default()
        }
    }

    fn title(&self) -> String {
        String::from("Коэффициенты многочлена")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::PowerInput(value) => self.power_input = value,
            Message::RatiosInput(value) => self.ratios_vec_input = value,
            Message::Submitted => {
                self.ratios_vec.clear();
                self.power = self.power_input.parse().unwrap_or(0);
                for item in self.ratios_vec_input.split(" ").collect::<Vec<&str>>() {
                    println!("{}", item);
                    self.ratios_vec.push(item.parse().unwrap_or(0));
                }
                self.derivative_coefficients = power_and_ratios(self.power, &self.ratios_vec);
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        container(
            column!(
                row!(
                    column!(
                        text("Введите степень многочлена:"),
                        text_input("Введите число", &self.power_input)
                        .on_input(|input| Message::PowerInput(input))
                        .on_submit(Message::Submitted)
                    ),
                    column!(
                        text("Введите коэффициенты многочлена:"),
                        text_input("Пример: 5 5 3 1 8", &self.ratios_vec_input)
                        .on_input(|input| Message::RatiosInput(input))
                        .on_submit(Message::Submitted)
                    ),
                ),
                button("Подтвердить").on_press(Message::Submitted),
                text(&self.derivative_coefficients)
            )
        )
        .into()
            
    }
}

fn power_and_ratios(power: i32, ratios: &Vec<i32>) -> String {
    let mut derivative_coefficients: String = String::default();
    let mut power_iter = power;
    let mut iterator = 0;
    while power_iter > 0 && iterator < ratios.len() {
        derivative_coefficients += (ratios[iterator].pow(power_iter as u32))
            .to_string()
            .as_str();
        derivative_coefficients += " ";
        power_iter -= 1;
        iterator += 1;
    }
    derivative_coefficients
}


fn main() -> iced::Result {
    App::run(iced::Settings::default())
}
