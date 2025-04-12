use std::future::{Ready, ready};

use actix_web::{FromRequest, HttpRequest, dev::Payload};

pub struct UserId(pub String);

//TODO: Throw an error if the header is not present
impl FromRequest for UserId {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let user_id = req
            .headers()
            .get("X-User-ID")
            .and_then(|h| h.to_str().ok())
            .unwrap_or_default();

        ready(Ok(UserId(user_id.to_string())))
    }
}
