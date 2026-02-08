use std::time::Duration;

use gilrs::{Axis, Button, Event, EventType, Gilrs};

const GAMEPAD_POLL_INTERVAL_MS: u64 = 16;
const GAMEPAD_REPEAT_FRAME_INTERVAL: u32 = 3;
const AXIS_THRESHOLD: f32 = 0.5;

struct MessageHandlers<Message> {
    previous_game: Message,
    next_game: Message,
    previous_section: Message,
    next_section: Message,
    launch: Message,
}

pub fn gamepad_worker<Message: 'static + Clone + Send>(
    on_previous_game: Message,
    on_next_game: Message,
    on_previous_section: Message,
    on_next_section: Message,
    on_launch: Message,
) -> impl iced::futures::Stream<Item = Message> {
    use iced::futures::stream::StreamExt;

    iced::stream::channel(50, move |mut output| async move {
        let mut gilrs = match Gilrs::new() {
            Ok(g) => g,
            Err(_) => return,
        };

        let mut interval = async_std::stream::interval(Duration::from_millis(GAMEPAD_POLL_INTERVAL_MS));
        let mut left_stick_x = 0.0_f32;
        let mut frame_counter = 0_u32;

        let handlers =
            MessageHandlers { previous_game: on_previous_game, next_game: on_next_game, previous_section: on_previous_section, next_section: on_next_section, launch: on_launch };

        loop {
            interval.next().await;
            frame_counter += 1;

            process_gamepad_events(&mut gilrs, &handlers, &mut left_stick_x, &mut output);

            if frame_counter.is_multiple_of(GAMEPAD_REPEAT_FRAME_INTERVAL) {
                send_thumbstick_repeat_message(left_stick_x, &handlers, &mut output);
            }
        }
    })
}

fn process_gamepad_events<Message: Clone>(
    gilrs: &mut Gilrs,
    handlers: &MessageHandlers<Message>,
    left_stick_x: &mut f32,
    output: &mut iced::futures::channel::mpsc::Sender<Message>,
) {
    while let Some(Event { event, .. }) = gilrs.next_event() {
        match event {
            EventType::ButtonPressed(button, _) => {
                handle_button_press(button, handlers, output);
            }
            EventType::AxisChanged(axis, value, _) => {
                handle_axis_change(axis, value, left_stick_x);
            }
            _ => {}
        }
    }
}

fn handle_button_press<Message: Clone>(button: Button, handlers: &MessageHandlers<Message>, output: &mut iced::futures::channel::mpsc::Sender<Message>) {
    let message = match button {
        Button::DPadLeft => Some(handlers.previous_game.clone()),
        Button::DPadRight => Some(handlers.next_game.clone()),
        Button::LeftTrigger2 => Some(handlers.previous_section.clone()),
        Button::RightTrigger2 => Some(handlers.next_section.clone()),
        Button::South => Some(handlers.launch.clone()),
        _ => None,
    };

    if let Some(msg) = message {
        let _ = output.try_send(msg);
    }
}

fn handle_axis_change(axis: Axis, value: f32, left_stick_x: &mut f32) {
    if axis == Axis::LeftStickX {
        *left_stick_x = value;
    }
}

fn send_thumbstick_repeat_message<Message: Clone>(left_stick_x: f32, handlers: &MessageHandlers<Message>, output: &mut iced::futures::channel::mpsc::Sender<Message>) {
    let message = if left_stick_x < -AXIS_THRESHOLD {
        Some(handlers.previous_game.clone())
    } else if left_stick_x > AXIS_THRESHOLD {
        Some(handlers.next_game.clone())
    } else {
        None
    };

    if let Some(msg) = message {
        let _ = output.try_send(msg);
    }
}
