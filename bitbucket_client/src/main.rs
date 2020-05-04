extern crate reqwest;
use oauth2::*;
use oauth2::basic::{BasicClient,BasicTokenType};
use std::net::TcpListener;
use std::io::{BufRead,BufReader,Write};
use url::Url;
use webbrowser;

fn main() {
    
    let client = BasicClient::new(
        ClientId::new("YOUR BITBUCKET CLIENT ID".to_string()),
        Some(ClientSecret::new("YOUR BITBUCKET SECRET".to_string())),
        AuthUrl::new("https://bitbucket.org/site/oauth2/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://bitbucket.org/site/oauth2/access_token".to_string()).unwrap())
    ).set_redirect_url(RedirectUrl::new("http://localhost:8765".to_string()).unwrap());
    println!("Getting client secret...");

    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("account".to_string()))
        .add_scope(Scope::new("team".to_string()))
        .add_scope(Scope::new("repository".to_string()))
        .add_scope(Scope::new("pullrequest".to_string()))
        .add_scope(Scope::new("pullrequest:write".to_string()))
        .url();

    let auth_url = auth_url.to_string();

    if webbrowser::open(&auth_url).is_ok() {
        println!("Opening browser: {}", auth_url);
    }
    let auth_code = web_server();

    println!("Getting user token...");
    let token_result = client
        .exchange_code(AuthorizationCode::new(auth_code.to_string()))
        .request(oauth2::reqwest::http_client)
        .unwrap();

    println!("GOT TOKEN: {:?}", token_result);
    let repositories = get_repositories(token_result.token_type(), token_result.access_token());
    println!("RESPONSE: {:?}", repositories);
}

fn get_repositories(token_type: &BasicTokenType, token: &AccessToken) -> String{
    let url = "https://api.bitbucket.org/2.0/repositories";
    let client = reqwest::blocking::Client::new();
    println!("Getting repositories...");
    let body = client.get(url)
        .header("Authorization", format!("{:?} {}", token_type, token.secret()))
        .send()
        .unwrap();
    body.text().unwrap()
}

fn web_server() -> String{
    let listener = TcpListener::bind("127.0.0.1:8765").unwrap();
    println!("Listening connection in port 8765");
    let mut code = String::new();
    for stream in listener.incoming(){
        match stream {
            Ok(mut stream) => {
                let mut reader = BufReader::new(&stream);
                let mut request_line = String::new();
                reader.read_line(&mut request_line).unwrap();
                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost:8765".to_string() + redirect_url)).unwrap();

                let code_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();

                let (_, value) = code_pair;
                code = value.into_owned();
                println!("CODE: {}", code);

                let message = "<SCRIPT>window.close();</SCRIPT>";
                let response = format!(
                    "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                    message.len(),
                    message
                );
                stream.write_all(response.as_bytes()).unwrap();                break;
            },
            Err(e) => println!("ERROR: {:?}", e),
        }
    }
    code
}