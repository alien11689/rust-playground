use std::collections::HashMap;

#[derive(serde::Deserialize, Debug)]
pub struct LeaderBoard {
    pub members: HashMap<String, Member>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Member {
    pub stars: u32,
    pub name: String,
}
