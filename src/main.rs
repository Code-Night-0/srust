use rspotify::{
    model::{AdditionalType, Country, CurrentlyPlayingContext, Market},
    prelude::OAuthClient,
    scopes, AuthCodeSpotify, Credentials, OAuth,
};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};
use url::Url;

fn listen() -> Vec<String> {
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    let conn = listener.accept().unwrap();
    let mut stream = conn.0;
    let reader = BufReader::new(&mut stream);

    let request: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    stream.write_all(response.as_bytes()).unwrap();
    drop(listener);
    request
}

fn parse_code(request: Vec<String>) -> (String, String) {
    let mut header = request[0].split_whitespace();
    header.next();
    let path = header.next().unwrap();

    let mut url = Url::parse("http://127.0.0.1:8888").unwrap();

    url = url.join(path).unwrap();
    let params = url.query_pairs().collect::<HashMap<_, _>>();

    let code = params.get("code").unwrap();
    let state = params.get("state").unwrap();
    (code.to_string(), state.to_string())
}

fn main() {
    let creds = Credentials::new(
        "41dc61f349114f1ba273a2581b74094a",
        "f762db0774504c7d885d466ce1ba8615",
    );
    let oauth = OAuth {
        redirect_uri: "http://127.0.0.1:8888/callback".to_string(),
        scopes: scopes!("user-read-currently-playing"),
        ..Default::default()
    };

    let mut spotify = AuthCodeSpotify::new(creds, oauth);
    // Obtaining the access token
    let url = spotify.get_authorize_url(false).unwrap();
    webbrowser::open(&url).unwrap();
    let request = listen();
    let (code, _) = parse_code(request);
    spotify.request_token(&code).unwrap();

    // Running the requests
    let market = Market::Country(Country::Turkey);
    let additional_types = [AdditionalType::Episode];
    let current = spotify
        .current_playing(Some(&market), Some(&additional_types))
        .unwrap()
        .unwrap()
        .item
        .unwrap();

    println!("{current:#?}");
}
