pub enum InvalidMove {
    WrongTurn,
    EmptySourceCell,
    OutOfBounds,
    OutOfRange,
    OutOfSight,
    ProvokeCheck,
    FriendlyFire,
    InvalidPromotionPiece,
}
