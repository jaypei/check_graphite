extern crate url;
extern crate hyper;
extern crate json;

#[allow(unused_imports)]
use std::io::Read;
use std::time::Duration;
use config;
use self::hyper::header::{Headers, Authorization, Basic};
use self::hyper::Client;


fn build_url() -> String {
    let mut graphite_url = url::Url::parse(config::ARGS.flag_url.as_str())
        .expect("Invalid url parameter.");
    graphite_url.set_path("/render");
    let mut query_string = url::form_urlencoded::Serializer::new(String::new());
    query_string.append_pair("format", "json");
    query_string.append_pair("target", config::ARGS.arg_metric.as_str());
    query_string.append_pair("from", config::ARGS.flag_from.as_str());
    query_string.append_pair("until", config::ARGS.flag_until.as_str());
    graphite_url.set_query(Some(query_string.finish().as_str()));
    graphite_url.to_string()
}

fn get_http_body(url: String, username: String, password: String)
                     -> String {
    let mut headers = Headers::new();
    let timeout_secs = config::ARGS.flag_timeout;
    if username != "" {
        headers.set(
            Authorization(
                Basic {
                    username: username,
                    password: Some(password),
                }
            )
        );
    }
    let mut client = Client::new();
    if timeout_secs > 0 {
        client.set_read_timeout(Some(Duration::new(timeout_secs, 0)));
        client.set_write_timeout(Some(Duration::new(timeout_secs, 0)));
    }
    let mut res = client
        .get(url.as_str())
        .headers(headers)
        .send()
        .expect("Fetch graphite api failed.");
    if res.status != hyper::Ok {
        panic!("Fetch graphite api failed");
    }
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    body
}

pub fn http_fetch() -> String {
    let username = config::ARGS.flag_user.clone();
    let password = config::ARGS.flag_password.clone();
    get_http_body(build_url(), username, password)
}
