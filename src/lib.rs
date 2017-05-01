extern crate hyper;
extern crate hyper_native_tls;

use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;

#[derive(Debug)]
pub enum FamilySearchError {
    BadApiResponse
}

impl Display for FamilySearchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            FamilySearchError::BadApiResponse => write!(f, "The FamilySearch \
                                                            API returned \
                                                            unexpected data."),
        }

    }
}
impl Error for FamilySearchError {
    fn description(&self) -> &str {
        match *self {
            FamilySearchError::BadApiResponse => "bad response"
        }
    }

    fn cause(&self) -> Option<&Error> {
        return None;
    }
}

mod server {
    pub const SANDBOX: &'static str = "https://identint.familysearch.org";
    pub const PRODUCTION: &'static str = "https://familysearch.org/platform";
}


mod auth {
    use server;
    use FamilySearchError;
    use hyper::Client;
    use hyper::status::StatusCode;
    use hyper::net::HttpsConnector;
    use hyper_native_tls::NativeTlsClient;
    use std::io::Read;

    pub fn get_code_no_auth(client_id: &str) -> Result<String, FamilySearchError> {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);
        let endpoint = "/cis-web/oauth2/v3/token";
        let url = server::SANDBOX.to_string() + endpoint + "?client_id=" + client_id;
        let mut res = client.post(&url)
            .send().unwrap();

        if res.status != StatusCode::Ok {
            return Err(FamilySearchError::BadApiResponse);
        }
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        return Ok(body);
    }

}

#[cfg(test)]
mod tests {
    use auth;

    #[test]
    fn retrieve_token() {
        println!("{:?}", auth::get_code_no_auth("ham"));
    }
}
