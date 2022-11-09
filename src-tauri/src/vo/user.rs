use serde::{Deserialize, Serialize};
use crate::model::user::User;
use crate::vo::formator::Convert;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct UserVo {
    pub uid: String,
    pub gid: String,
    pub name: String,
    pub groups: Vec<String>,
}

impl Convert<UserVo> for User {
    fn convert(&self) -> UserVo {
        UserVo {
            uid: self.uid.clone(),
            gid: self.gid.clone(),
            name: self.name.clone(),
            groups: self.groups.clone(),
        }
    }
}