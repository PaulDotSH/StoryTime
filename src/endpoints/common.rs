use crate::user::Role;
use axum::http::HeaderMap;
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};

pub static MAX_INDEX: i16 = 9;
pub static MIN_INDEX: i16 = 2;
pub static FORMAT: &str = "%Y-%m-%d %H:%M:%S%.f";
pub fn generate_token(length: usize) -> String {
    Alphanumeric.sample_string(&mut thread_rng(), length)
}

// Using expect in all of these functions since we set these headers in the auth handler, and the request wont arrive in an endpoint if these aren't set
#[inline(always)]
pub fn get_username_from_header(headers: &HeaderMap) -> &str {
    let id = headers.get("id").expect("Header \"id\" doesn't exist");
    id.to_str()
        .expect("Header cannot be converted into a string")
}

#[inline(always)]
pub fn get_role_from_header(headers: &HeaderMap) -> Role {
    Role::from(get_role_i16_from_header(headers))
}

#[inline(always)]
pub fn get_role_i16_from_header(headers: &HeaderMap) -> i16 {
    let role = headers
        .get("role")
        .expect("Header \"role\" doesn't exist")
        .to_str()
        .expect("header cannot be converted to string");
    role.parse::<i16>().expect("Header is not valid role")
}
