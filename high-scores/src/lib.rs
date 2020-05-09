#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32]
}

impl<'a> HighScores <'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores {
            scores: scores
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_three = self.scores.to_vec();
        top_three.sort_unstable();
        top_three = top_three.iter().rev().map(|x| *x).collect::<Vec<u32>>();
        match top_three.len() {
            0 => Vec::<u32>::new(),
            1 => top_three[0..1].to_vec(),
            2 => top_three[0..2].to_vec(),
            _ => top_three[0..3].to_vec()
        }
    }
}
