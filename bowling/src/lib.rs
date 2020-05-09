use Error::*;
use Frame::*;
use FrameError::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
enum Frame {
    Strike,
    Spare(u16, u16),
    Open(u16, u16),
    Fill(u16, Option<u16>),
}

impl Frame {
    fn first(&self) -> u16 {
        match self {
            Strike => 10,
            Spare(f, _) => *f,
            Open(f, _) => *f,
            Fill(f, _) => *f,
        }
    }

    fn second(&self) -> Option<u16> {
        match self {
            Spare(_, x) => Some(*x),
            Open(_, x) => Some(*x),
            Fill(_, Some(x)) => Some(*x),
            _ => None,
        }
    }

    fn value(&self, n_1_roll: Option<u16>, n_2_roll: Option<u16>) -> Result<u16, &str> {
        match (self, n_1_roll, n_2_roll) {
            (Strike, Some(x), Some(y)) => Ok(10 + x + y),
            (Spare(_, _), Some(x), _) => Ok(10 + x),
            (Open(f1, f2), _, _) => Ok(f1 + f2),
            (Fill(_, _), _, _) => Ok(0),
            _ => Err("Not enough info to determine value of frame"),
        }
    }
}

enum FrameError {
    FrameNotFinished,
}

#[derive(Copy, Clone, Debug)]
enum FillType {
    Strike,
    Spare,
    Nill,
}

#[derive(Copy, Clone, Debug)]
struct FrameBuilder {
    first: Option<u16>,
    second: Option<u16>,
    fill_type: FillType,
}

impl FrameBuilder {
    fn new(fill_type: FillType) -> Self {
        Self {
            first: None,
            second: None,
            fill_type: fill_type,
        }
    }

    fn add_roll(&mut self, pins: u16) -> Result<(), Error> {
        match self.fill_type {
            FillType::Nill =>
            match (self.first, self.second, pins) {
                (None, _, x) if x <= 10 => {
                    self.first = Some(x);
                    Ok(())
                }
                (None, _, x) if x > 10 => Err(NotEnoughPinsLeft),
                (Some(f), None, x) if f + x <= 10 => {
                    self.second = Some(x);
                    Ok(())
                }
                (Some(f), None, x) if f + x > 10 => Err(NotEnoughPinsLeft),
                (_, _, _) => unreachable!(),
            },
            FillType::Strike => match (self.first, self.second, pins) {
                (None, _, x) if x <= 10 => {
                    self.first = Some(x);
                    Ok(())
                },
                (Some(10), _, x) if x <= 10 => {
                    self.second = Some(x);
                    Ok(())
                },
                (Some(f), None, x) if f + x <= 10 => {
                    self.second = Some(x);
                    Ok(())
                },
                (Some(_), Some(_), _) => Err(NotEnoughPinsLeft),
                (_, _, _) => Err(NotEnoughPinsLeft)
            },
            FillType::Spare => match (self.first, pins) {
                (None, x) if x <= 10 => {
                    self.first = Some(x);
                    Ok(())
                },
                (Some(_), _) => Err(NotEnoughPinsLeft),
                (_, _) => Err(NotEnoughPinsLeft)
            }
        }
    }

    fn get_frame(self) -> Result<Frame, FrameError> {
        match (self.first, self.second, self.fill_type) {
            (Some(10), _, FillType::Nill) => Ok(Strike),
            (Some(f), Some(s), FillType::Nill) if f + s == 10 => Ok(Spare(f, s)),
            (Some(f), Some(s), FillType::Nill) if f + s != 10 => Ok(Open(f, s)),
            (Some(f), Some(s), FillType::Strike) => Ok(Fill(f, Some(s))),
            (Some(_), None, FillType::Strike) => Err(FrameNotFinished),
            (None, None, FillType::Strike) => Err(FrameNotFinished),
            (None, None, FillType::Spare) => Err(FrameNotFinished),
            (Some(f), None, FillType::Spare) => Ok(Fill(f, None)),
            (None, _, FillType::Nill) => Err(FrameNotFinished),
            (_, None, FillType::Nill) => Err(FrameNotFinished),
            (_, _, _) => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct BowlingGame {
    frames: Vec<Frame>,
    frame_builder: FrameBuilder,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: Vec::<Frame>::new(),
            frame_builder: FrameBuilder::new(FillType::Nill),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        // handle end of game

        if self.game_complete() {
            return Err(GameComplete);
        }

        // handle fill type

        let out = self.frame_builder.add_roll(pins);

        // handle end of frame
        match self.frame_builder.get_frame() {
            Ok(frame) => {
                self.frames.push(frame);
                let current_fill_type = match (self.frames.len(), self.frames.last()) {
                    (10, Some(Strike)) => FillType::Strike,
                    (10, Some(Spare(_, _))) => FillType::Spare,
                    _ => FillType::Nill,
                };
                self.frame_builder = FrameBuilder::new(current_fill_type)
            }
            Err(_) => (),
        };

        out
    }

    fn game_complete(&self) -> bool {
        match (self.frames.len(), self.frames.last()) {
            (10, Some(Open(_, _))) => true,
            (11, Some(Fill(_, _))) => true,
            _ => false,
        }
    }

    pub fn score(&self) -> Option<u16> {
        if !self.game_complete() {
            return None;
        }

        let mut _score = 0;
        for (idx, frame) in self.frames.iter().enumerate() {
            let next_roll = self.frames.get(idx + 1).map(|x| x.first());
            let second_roll = match self.frames.get(idx + 1) {
                None => None,
                Some(Strike) => self.frames.get(idx + 2).map(|x| x.first()),
                Some(x) => x.second(),
            };
            _score += frame.value(next_roll, second_roll).unwrap();
        }
        Some(_score)
    }
}
