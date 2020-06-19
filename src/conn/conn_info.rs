// Copyright (c) 2020 rust-mysql-simple contributors
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::borrow::Cow;

pub trait ConnInfo {
    /// Returns connection identifier.
    fn connection_id(&self) -> u32;

    /// Returns number of rows affected by the last query.
    fn affected_rows(&self) -> u64;

    /// Returns last insert id of the last query.
    fn last_insert_id(&self) -> Option<u64>;

    /// Returns number of warnings, reported by the server.
    fn warnings(&self) -> Option<u16>;

    /// [Info], reported by the server.
    ///
    /// Will be empty if not defined.
    ///
    /// [Info]: http://dev.mysql.com/doc/internals/en/packet-OK_Packet.html
    fn info_ref(&self) -> &[u8];

    /// [Info], reported by the server.
    ///
    /// Will be empty if not defined.
    ///
    /// [Info]: http://dev.mysql.com/doc/internals/en/packet-OK_Packet.html
    fn info_str(&self) -> Cow<str>;
}