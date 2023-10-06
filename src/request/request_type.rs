//! HTTP [`RequestType`]s for [`Get`] and [`Post`]

/// Trait [`RequestType`] of [`Request`](super::Request)
pub trait RequestType {}

/// HTTP `Get` `RequestType` of [`Request`](super::Request)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Get;
impl RequestType for Get {}

/// HTTP `Post` `RequestType` of [`Request`](super::Request)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Post;
impl RequestType for Post {}
