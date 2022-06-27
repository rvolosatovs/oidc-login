use anyhow::Context;
use clap::Parser;
use oauth2::basic::BasicClient;
use oauth2::devicecode::StandardDeviceAuthorizationResponse;
use oauth2::ureq::http_client;
use oauth2::url::Url;
use oauth2::{AuthType, AuthUrl, ClientId, DeviceAuthorizationUrl, Scope, TokenResponse, TokenUrl};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, default_value_t = Url::parse("https://dev-l20dl4-w.us.auth0.com").unwrap())]
    url: Url,
}

const CLIENT_ID: &str = "mhIPNbaEGEvkgqhznnWUb6vFwnfFy0dM";

fn main() -> Result<(), anyhow::Error> {
    let Args { url } = Args::parse();

    let dev_auth_url = DeviceAuthorizationUrl::new(format!("{url}oauth/device/code"))
        .context("Failed to construct device authorization URL")?;
    let auth_url =
        AuthUrl::new(format!("{url}authorize")).context("Failed to construct authorization URL")?;
    let token_url =
        TokenUrl::new(format!("{url}oauth/token")).context("Failed to construct token URL")?;

    let client = BasicClient::new(
        ClientId::new(CLIENT_ID.into()),
        None,
        auth_url,
        Some(token_url),
    )
    .set_auth_type(AuthType::RequestBody)
    .set_device_authorization_url(dev_auth_url);

    let details: StandardDeviceAuthorizationResponse = client
        .exchange_device_code()
        .context("Failed to construct device authorization request")?
        .add_scope(Scope::new("openid".into()))
        .request(http_client)
        .context("Failed to request device code")?;

    println!(
        "Open this URL in your browser:\n{}\nand enter the code: {}",
        **details.verification_uri(),
        details.user_code().secret()
    );

    let res = client
        .exchange_device_access_token(&details)
        .request(http_client, std::thread::sleep, None)
        .context("Failed to exchange device code for a token")?;
    println!("Token: {}", res.access_token().secret());
    Ok(())
}
