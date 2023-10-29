use serde::{Deserialize, Serialize};

#[repr(i16)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    Banned = 0,
    UnconfirmedMail = 1,
    User = 2,
    Admin = 3,
}