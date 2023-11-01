use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

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

impl Display for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Role::Banned => {
                    "Banned"
                }
                Role::UnconfirmedMail => {
                    "Unconfirmed Mail"
                }
                Role::User => {
                    "User"
                }
                Role::Admin => {
                    "Admin"
                }
            }
        )
    }
}
