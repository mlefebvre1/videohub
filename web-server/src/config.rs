use lazy_static::lazy_static;
use rocket::Config;
use std::{net::Ipv4Addr, str::FromStr};

const DEFAULT_VIDEOHUB_IPV4_ADDR: &str = "10.26.135.196";

lazy_static! {
    pub static ref VIDEOHUB_IPV4_ADDR: Ipv4Addr = {
        let s: String = Config::figment()
            .extract_inner("videohub_addr")
            .unwrap_or_else(|_| DEFAULT_VIDEOHUB_IPV4_ADDR.to_string());
        Ipv4Addr::from_str(&s).unwrap()
    };
}
