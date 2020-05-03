use oauth2::*;
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use webbrowser;

fn main() {
    
    let client = BasicClient::new(
        ClientId::new("6cP8WxDNmeMdjJKYmN".to_string()),
        Some(ClientSecret::new("YB3uf95Jyf6UdGh7EBA9LdfJnBwCGxSD".to_string())),
        AuthUrl::new("https://bitbucket.org/site/oauth2/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://bitbucket.org/site/oauth2/access_token".to_string()).unwrap())
    ).set_redirect_url(RedirectUrl::new("http://localhost:8765".to_string()).unwrap());
    println!("Hello, world!");

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Add scopes
        .add_scope(Scope::new("read".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    let auth_url = auth_url.to_string();
    webbrowser::open(&auth_url).unwrap();

    //get auth_code from site
    let auth_code = "auth_code";

    let token_result = client
        .exchange_code(AuthorizationCode::new(auth_code.to_string()))
        .set_pkce_verifier(pkce_verifier)
        .request(http_client);

    println!("TOKEN: {:?}", token_result);
}
