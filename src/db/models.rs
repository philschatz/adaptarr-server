use chrono::NaiveDateTime;

use super::schema::*;

#[derive(Associations, Clone, Debug, Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    /// User's email address. We use this for identification (e.g. when logging
    /// into the system) and communication.
    pub email: String,
    /// User's display name. This is visible to other users.
    pub name: String,
    /// Hash of password, currently Argon2.
    pub password: Vec<u8>,
    /// Salt used for hashing password.
    pub salt: Vec<u8>,
    /// Is this user an administrator?
    pub is_super: bool,
}

#[derive(Clone, Copy, Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a [u8],
    pub salt: &'a [u8],
    pub is_super: bool,
}

#[derive(Associations, Clone, Copy, Debug, Identifiable, Queryable)]
#[belongs_to(User, foreign_key = "user")]
pub struct Session {
    /// ID of this session.
    pub id: i32,
    /// ID of the user owning this session.
    pub user: i32,
    /// Maximum age for the session, after which it must not be used.
    pub expires: NaiveDateTime,
    /// Date of the last use of a session. Sessions which were not used for some
    /// time should expire, even if they are still valid according to `expires`.
    pub last_used: NaiveDateTime,
    /// If this an administrative session? To limit attack surface
    /// administrative sessions are granted for a short time, after which they
    /// become normal sessions again.
    pub is_super: bool,
}

#[derive(Clone, Copy, Debug, Insertable)]
#[table_name = "sessions"]
pub struct NewSession {
    pub user: i32,
    pub expires: NaiveDateTime,
    pub last_used: NaiveDateTime,
    pub is_super: bool,
}
