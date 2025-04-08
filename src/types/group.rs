use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct GroupsResponse {
    pub data: Vec<Group>,
    pub state: i8,
    pub msg: String
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Group {
    pub name: String,
    pub id: u16,
    pub kurs: u8,
    pub facul: String,
    #[serde(rename = "yearName")]
    pub year: String,
    #[serde(rename = "facultyID")]
    pub facul_id: u16
}
