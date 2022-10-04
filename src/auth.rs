use crate::realms;
use reqwest;
use oauth;


pub struct Auth {
    // realm: Option<String>,
    pub oauth_consumer_key: String,
    // oauth_token: Option<String>,
    // oauth_nonce: Option<String>,
    // oauth_timestamp: Option<String>,
    // oauth_signature_method: Option<String>,
    // oauth_version: Option<String>,
    pub oauth_signature: String,
}


impl Auth {
    pub async fn authenticate(&self, userid: &str) -> Result<reqwest::Response, reqwest::Error>{
        let uri = "https://api.schoology.com/v1/users/".to_owned() + userid;
        let request = realms::None {};
        let key = &*self.oauth_consumer_key;
        let secret = &*self.oauth_signature;

        let token = oauth::Token::from_parts(key, secret, "", "");
        let mut authorization_header = oauth::get(&uri.clone(), &request, &token, oauth::PLAINTEXT);
        authorization_header.insert_str(6, "realm=\"Schoology API\",");
        // println!("{}\n", authorization_header.len());
        // println!("{}", authorization_header);


        let stuff = reqwest::Client::new().get(&uri.clone()).header("Authorization", authorization_header).send().await;
        return stuff
    }
}