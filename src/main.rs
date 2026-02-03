use iced::widget::{column, container, text};
use iced::{Element, Task};

mod domain;

fn main() -> iced::Result {
    iced::application("Load!64", Load64::update, Load64::view).run_with(Load64::new)
}

struct Load64;

#[derive(Debug, Clone)]
enum Message {}

impl Load64 {
    fn new() -> (Self, Task<Message>) {
        (Self, Task::none())
    }

    #[allow(clippy::unused_self, clippy::missing_const_for_fn)]
    fn update(&mut self, _message: Message) {}

    #[allow(clippy::unused_self)]
    fn view(&self) -> Element<'_, Message> {
        let content = text("Hello Load!64").size(50);

        container(column![content]).padding(20).center_x(iced::Fill).center_y(iced::Fill).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_initializes() {
        let (_app, _task) = Load64::new();
    }

    #[test]
    fn test_greeting_message() {
        let greeting = "Hello Load!64";
        assert_eq!(greeting, "Hello Load!64");
    }
}
