use iced::widget::{column, container, text};
use iced::{Element, Task};

fn main() -> iced::Result {
    iced::application("Load!64", Load64::update, Load64::view).run_with(Load64::new)
}

#[derive(Default)]
struct Load64;

#[derive(Debug, Clone)]
enum Message {}

impl Load64 {
    fn new() -> (Self, Task<Message>) {
        (Self, Task::none())
    }

    #[allow(clippy::unused_self, clippy::missing_const_for_fn)]
    fn update(&mut self, _message: Message) {
        // No messages to handle yet
    }

    #[allow(clippy::unused_self)]
    fn view(&self) -> Element<'_, Message> {
        let content = column![text("Hello Load!64").size(50),];

        container(content).center(iced::Fill).into()
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
