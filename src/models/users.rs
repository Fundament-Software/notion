use crate::ids::UserId;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UserCommon {
    pub id: UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

impl Hash for UserCommon {
    fn hash<H: Hasher>(
        &self,
        state: &mut H,
    ) {
        self.id.hash(state);
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Person {
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Bot {
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum User {
    Person {
        #[serde(flatten)]
        common: UserCommon,
        person: Person,
    },
    Bot {
        #[serde(flatten)]
        common: UserCommon,
        bot: Bot,
    },
}

impl Hash for User {
    fn hash<H: Hasher>(
        &self,
        state: &mut H,
    ) {
        use User::*;
        match self {
            Person { common, .. } | Bot { common, .. } => &common.id,
        }
        .hash(state);
    }
}
