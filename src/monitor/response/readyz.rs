use std::str::FromStr;

use crate::Error;

/// Data container for response of [`monitor::Endpoint::Readyz`](crate::monitor::Endpoint::Readyz)
///
/// See also [`monitor::Readyz`](crate::monitor::Readyz)
///
#[derive(Debug, PartialEq)]
pub struct Readyz {
    /// `true` when node ready
    pub is_node_ok: bool,
    /// `true` when leader is ok ([`Query::is_noleader()`](crate::Query::is_noleader()) false)
    pub is_leader_ok: bool,
    /// `true` when store is ok ([`Query::is_noleader()`](crate::Query::is_noleader()) false)
    pub is_store_ok: bool,
}

const MAX_READYZ_LINE_LEN: usize = 12;

impl FromStr for Readyz {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut readyz = Readyz {
            is_node_ok: false,
            is_leader_ok: false,
            is_store_ok: false,
        };

        let lines = s.lines();
        for line in lines {
            if line.len() > MAX_READYZ_LINE_LEN || !line.starts_with('[') {
                return Err("parse readyz failed".into());
            }

            if line.ends_with("node ok") {
                readyz.is_node_ok = true;
            } else if line.ends_with("leader ok") {
                readyz.is_leader_ok = true;
            } else if line.ends_with("store ok") {
                readyz.is_store_ok = true;
            } else {
                return Err("readyz unknown line".into());
            }
        }

        Ok(readyz)
    }
}
