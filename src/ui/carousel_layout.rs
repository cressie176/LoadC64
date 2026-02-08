pub struct CarouselLayout {
    number_of_regular_games_each_side: usize,
    number_of_games: usize,
    offset: i32,
    canvas_padding: f32,
}

impl CarouselLayout {
    const NORMAL_GAME_WIDTH: f32 = 240.0;
    const CURRENT_GAME_WIDTH: f32 = Self::NORMAL_GAME_WIDTH * 1.2;
    const NORMAL_GAME_HEIGHT: f32 = 320.0;
    const CURRENT_GAME_HEIGHT: f32 = Self::NORMAL_GAME_HEIGHT * 1.2;
    const GAME_CONTAINER_SPACING: f32 = 10.0;

    pub fn new(window_width: f32) -> Self {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let number_of_regular_games_each_side = (((window_width - Self::CURRENT_GAME_WIDTH) / 2.0) / (Self::NORMAL_GAME_WIDTH + Self::GAME_CONTAINER_SPACING)).floor() as usize;

        #[allow(clippy::cast_precision_loss)]
        let total_carousel_width = (number_of_regular_games_each_side as f32 * 2.0).mul_add(Self::NORMAL_GAME_WIDTH + Self::GAME_CONTAINER_SPACING, Self::CURRENT_GAME_WIDTH);

        let canvas_padding = (window_width - total_carousel_width) / 2.0;
        let number_of_games = number_of_regular_games_each_side * 2 + 1;
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let offset = -(number_of_regular_games_each_side as i32);

        Self { number_of_regular_games_each_side, number_of_games, offset, canvas_padding }
    }

    pub const fn current_game_index(&self) -> usize {
        self.number_of_regular_games_each_side
    }

    pub const fn game_width(&self, index: usize) -> f32 {
        if index == self.number_of_regular_games_each_side { Self::CURRENT_GAME_WIDTH } else { Self::NORMAL_GAME_WIDTH }
    }

    pub const fn game_height(&self, index: usize) -> f32 {
        if index == self.number_of_regular_games_each_side { Self::CURRENT_GAME_HEIGHT } else { Self::NORMAL_GAME_HEIGHT }
    }

    pub const fn spacing() -> f32 {
        Self::GAME_CONTAINER_SPACING
    }

    pub const fn number_of_games(&self) -> usize {
        self.number_of_games
    }

    pub const fn offset(&self) -> i32 {
        self.offset
    }

    pub const fn canvas_padding(&self) -> f32 {
        self.canvas_padding
    }
}
