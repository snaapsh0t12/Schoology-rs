use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize)]
pub struct School {
    id: Option<String>,
    title: Option<String>,
    address1: Option<String>,
    address2: Option<String>,
    city: Option<String>,
    state: Option<String>,
    postal_code: Option<String>,
    country: Option<String>,
    website: Option<String>,
    phone: Option<String>,
    fax: Option<String>,
    picture_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    uid: Option<String>,
    school_id: Option<String>,
    building_id: Option<String>,
    school_uid: String,
    name_title: Option<String>,
    name_title_show: Option<bool>,
    name_first: Option<String>,
    name_first_preferred: Option<String>,
    name_middle: Option<String>,
    name_middle_show: Option<bool>,
    name_last: Option<String>,
    name_display: Option<String>,
    username: Option<String>,
    primary_email: Option<String>,
    picture_url: Option<String>,
    gender: Option<String>,
    position: Option<String>,
    grad_year: Option<i8>,
    password: Option<String>,
    role_id: Option<i32>,
    tz_offset: Option<i32>,
    tz_name: Option<String>,
    parents: Option<String>,
    child_uids: Option<String>,
    send_message: Option<bool>,
    additional_buildings: Option<i32>,
}

impl User {
    pub async fn get_json(&self) {
        
    }
}


#[derive(oauth::Request)]
pub struct None {}