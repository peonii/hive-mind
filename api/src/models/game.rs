use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Serialize, Deserialize, Clone)]
pub enum GameState {
    // Pre-start state.
    //
    // This is before the host joins, and transitions immediately into
    // Starting once at least one connection is established.
    WaitingToStart,

    // The starting game state.
    //
    // Players may join during this state, and normally immutable
    // variables such as `code` or `players_num` may change here.
    //
    // The field is a representation of how many players are ready
    // to start. Once this field reaches the connection count, the game
    // moves to a Guessing state.
    Starting(usize),

    // Players are attempting to guess each others' words.
    // The field is a representation of how many players are needed
    // until everyone gives their answer.
    //
    // This state is advanced once the waiting count reaches 0.
    Guessing(usize),

    // Answers are being shown. This state is advanced
    // once the waiting count reaches 0.
    Answers(usize),

    // The game is finished, everyone has guessed the same word.
    // This state is automatically triggered by the players winning,
    // no matter how many players are still waiting to advance Answers.
    Finished,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    // Immutable variables that are created at the start of a game
    // and then changed never again.
    pub code: u16,
    pub players_num: usize,

    // Mutable game state.
    pub state: GameState,
    // SmolStr is used here to save on an additional
    // heap allocation each time an answer is added.
    //
    // It is important for Vec to be initialized with the
    // proper capacity, to avoid additional allocations.
    pub answers: Vec<SmolStr>,

    // State that is used for resource management.
    pub connections: usize,
}
