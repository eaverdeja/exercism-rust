use std::array;

const FRAMES_COUNT: usize = 10;
const PIN_COUNT: u16 = 10;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq, Eq)]
struct Frame {
    throws: Vec<u16>,
    fill_balls: Option<u8>,
    index: usize,
}

impl Frame {
    fn new(index: usize) -> Self {
        Frame {
            throws: Vec::new(),
            fill_balls: None,
            index,
        }
    }

    fn score(&self, game: &BowlingGame) -> u16 {
        self.throws
            .iter()
            .enumerate()
            .map(|(i, t)| self.compute_score_for_throw(*t, i, game))
            .sum()
    }

    fn compute_score_for_throw(&self, throw: u16, idx: usize, game: &BowlingGame) -> u16 {
        if self.is_strike(0) {
            self.score_for_strike(throw, game)
        } else if idx == 1 && self.is_spare() {
            self.score_for_spare(throw, game)
        } else {
            throw
        }
    }

    fn score_for_strike(&self, throw: u16, game: &BowlingGame) -> u16 {
        if let Some(next_frame) = self.next_frame(game) {
            return if next_frame.throws.first() == Some(&10) {
                // Consecutive strikes
                if let Some(second_next_frame) = next_frame.next_frame(game) {
                    // If we still have frames, the game hasn't ended yet.
                    // We get bonus points for our second strike, plus
                    // the first throw from the 2nd next frame.
                    let extra_throw = second_next_frame
                        .throws
                        .first()
                        .expect("frame should have at least one throw");
                    throw + PIN_COUNT + extra_throw
                } else {
                    // Otherwise, we're at the 2nd last frame.
                    // In this case, we get bonus points from
                    // the next 2 throws as usual
                    throw + next_frame.next_two_throws()
                }
            } else {
                // Normal strikes get bonus points from the next two throws
                throw + next_frame.next_two_throws()
            };
        }
        // Last frame strikes do not get bonus points
        // from the next two throws
        throw
    }

    fn score_for_spare(&self, throw: u16, game: &BowlingGame) -> u16 {
        if let Some(next_frame) = self.next_frame(game) {
            // Spares get bonus points from the next throw
            let next_throw = next_frame
                .throws
                .first()
                .expect("frame should have at least one throw");

            return throw + next_throw;
        }
        // Last frame spares do not get bonus points
        // from the next throw
        throw
    }

    fn next_frame<'a>(&'a self, game: &'a BowlingGame) -> Option<&'a Frame> {
        if self.index + 1 < FRAMES_COUNT {
            game.frames.get(self.index + 1)
        } else {
            None
        }
    }

    fn remaining_pins(&self) -> u16 {
        if self.fill_balls.is_some() {
            let double_strike = self.is_strike(0) && self.is_strike(1);
            if self.throws.len() == 1 || self.is_spare() || double_strike {
                return PIN_COUNT;
            }

            return PIN_COUNT - self.last_throw();
        };

        if self.is_strike(0) || self.throws.len() >= 2 {
            return PIN_COUNT - self.next_two_throws();
        };

        PIN_COUNT - self.last_throw()
    }

    fn is_strike(&self, throw: usize) -> bool {
        self.throws.get(throw) == Some(&PIN_COUNT)
    }

    fn is_spare(&self) -> bool {
        let mut throws = self.throws.iter();
        if let (Some(first_throw), Some(second_throw)) = (throws.next(), throws.next()) {
            first_throw + second_throw == PIN_COUNT
        } else {
            false
        }
    }

    fn last_throw(&self) -> &u16 {
        self.throws.last().unwrap_or(&0)
    }

    fn next_two_throws(&self) -> u16 {
        self.throws.iter().take(2).sum()
    }
}

pub struct BowlingGame {
    frames: [Frame; FRAMES_COUNT],
    current_frame: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: array::from_fn(Frame::new),
            current_frame: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.current_frame >= FRAMES_COUNT {
            return Err(Error::GameComplete);
        }

        self.process_roll(pins)?;

        if self.has_fill_balls_to_process() {
            self.process_fill_balls()
        } else {
            self.advance_frame_if_needed()
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        // The ball is still rolling!
        if self.current_frame < FRAMES_COUNT {
            return None;
        }
        Some(self.frames.iter().map(|f| f.score(self)).sum())
    }

    fn process_roll(&mut self, pins: u16) -> Result<(), Error> {
        let frame = self.current_frame_mut();
        if pins > frame.remaining_pins() {
            return Err(Error::NotEnoughPinsLeft);
        }

        frame.throws.push(pins);
        Ok(())
    }

    fn has_fill_balls_to_process(&mut self) -> bool {
        // The end of the game is special - strikes and spares get fill balls
        let frame = self.current_frame_mut();
        let throws = frame.throws.len();

        if frame.index == FRAMES_COUNT - 1 && frame.remaining_pins() == 0 && throws <= 2 {
            // Strikes get two fill balls
            if throws == 1 {
                frame.fill_balls = Some(2)
            }
            // Spares get one fill ball
            else {
                frame.fill_balls = Some(1)
            }
        }
        frame.fill_balls.unwrap_or(0) > 0
    }

    fn process_fill_balls(&mut self) {
        let frame = self.current_frame_mut();
        if let Some(balls) = frame.fill_balls {
            if balls > 0 {
                frame.fill_balls = Some(balls - 1);
            }
        }
    }

    fn advance_frame_if_needed(&mut self) {
        let frame = self.current_frame();
        if frame.remaining_pins() == 0 || frame.throws.len() >= 2 {
            self.current_frame += 1;
        }
    }

    fn current_frame(&self) -> &Frame {
        &self.frames[self.current_frame]
    }

    fn current_frame_mut(&mut self) -> &mut Frame {
        &mut self.frames[self.current_frame]
    }
}
