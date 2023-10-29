use serde::{Deserialize, Serialize};

#[repr(i16)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    Banned = 0,
    UnconfirmedMail = 1,
    User = 2,
    Admin = 3,
}

impl From<i16> for Role {
    fn from(value: i16) -> Self {
        match value {
            0 => Role::Banned,
            1 => Role::UnconfirmedMail,
            2 => Role::User,
            3 => Role::Admin,
            _ => Role::Banned,
        }
    }
}
