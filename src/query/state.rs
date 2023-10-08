//! Consistency states for `Query<State>`
//!
//! See [`ConsistencyLevel`](super::ConsistencyLevel) for details.
//!

/// Trait `State` for `Query`
pub trait State {}

/// `LevelNone`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LevelNone;
impl State for LevelNone {}

/// `LevelNoneMulti`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LevelNoneMulti;
impl State for LevelNoneMulti {}

/// `NoLevel`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NoLevel;
impl State for NoLevel {}

/// `NoLevelMulti`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NoLevelMulti;
impl State for NoLevelMulti {}

/// `LevelStrong`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LevelStrong;
impl State for LevelStrong {}

/// `LevelStrongMulti`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LevelStrongMulti;
impl State for LevelStrongMulti {}

/// `LevelWeak`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LevelWeak;
impl State for LevelWeak {}

/// `LevelWeakMulti`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LevelWeakMulti;
impl State for LevelWeakMulti {}
