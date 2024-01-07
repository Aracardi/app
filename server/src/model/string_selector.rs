pub enum StringSelector {
    // A random player
    PlayerRandom(),
    // A player offset from the current turn
    PlayerOffset(usize),
    SubStage(), // ?!?!?!?!?
}

pub enum SubStageStringSelector {
    Target,
    
}

pub enum PollStringSelector {
    Result,
}
