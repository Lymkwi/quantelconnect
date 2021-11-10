extern crate reqwest;
extern crate clap;
extern crate kuchiki;

use clap::{Arg, App};
use reqwest::header;
use kuchiki::traits::TendrilSink;

const QUANTIC_DOMAIN: &str = "https://www.quantic-telecom.net";
const QUANTIC_ENDPOINT: &str = "/connexion-reseau";
const QUANTIC_PORTAL: &str = "https://www.quantic-telecom.net/connexion-reseau";
const QUANTIC_ACCOUNT: &str = "https://www.quantic-telecom.net/compte";

fn parse_html(data: String) -> String {
    // Descend down into the form
    let document = kuchiki::parse_html().one(data);
    let mut underscore_token = String::new();
    for data in document.select("input").unwrap() {
        let elem = data.as_node().as_element().unwrap();
        // Get the name
        let attribs = elem.attributes.borrow();
        if let Some(name) = attribs.get("name") {
            if name == "_token" {
                underscore_token.push_str(attribs.get("value").unwrap());
            }    
        }
    }
    underscore_token
}

fn connect(username: &str, password: &str, confirm_other_cons: bool)
    -> Result<bool, Box<dyn std::error::Error>> {
    // Step 0 : Build the Reqwest client
    // Default headers list
    let headers: header::HeaderMap = header::HeaderMap::new();
    let user_agent: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:91.0) Gecko/20100101 Firefox/91.0";
    let client = reqwest::blocking::Client::builder()
        .user_agent(user_agent)
        //.danger_accept_invalid_certs(true)
        //.proxy(reqwest::Proxy::all("http://localhost:8080/")?)
        .default_headers(headers)   // Default headers to fool the site
        .cookie_store(true)         // We need cookies to log in
        .build()
        .unwrap();
    // Step 1 : Fetch connection page
    let res1 = client.get(QUANTIC_PORTAL)
        .send()?
        .text()?;
    println!("Obtained login page \u{2713}");
    // Step 2 : Retrieve login form token
    let underscore_token = parse_html(res1);
    println!("Found underscore token : {}", underscore_token);
    // Step 3 : POST request
    let params: [(&str, &str);4] = [
        ("_token", &underscore_token), ("email", username),
        ("password", password),
        ("confirm_other_connections",
             if confirm_other_cons { "on" } else { "off" })
    ];
    let res2 = client.post(QUANTIC_PORTAL)
        .form(&params)
        .header("Connection", "close")
        .header("Referer", QUANTIC_PORTAL)
        .header("Origin", QUANTIC_DOMAIN)
        .header("Filename", QUANTIC_ENDPOINT)
        .send()?;
    println!("POST made (Status {})", res2.status());
    // Step 4 : Check by fetching account page
    let res3 = client.get(QUANTIC_ACCOUNT)
        .send()?
        .text()?;
    // If it contains that string (which is a comment on the page) it's done
    Ok(res3.contains("Static sidebar for desktop"))
}

fn main() {
    let args = App::new("QuantelConnect")
        .version("0.2")
        .author("Lux A. Phifollen <limefox@vulpinecitrus.info>")
        .about("Command line utility to automatically connect to the QuanticTelecom captive portal")
        .arg(Arg::with_name("username")
             .short("u")
             .long("username")
             .value_name("login")
             .help("User name used to authenticate to the portal")
             .takes_value(true))
        .arg(Arg::with_name("password")
             .short("p")
             .long("password")
             .value_name("pass")
             .help("Password for the captive portal account")
             .takes_value(true))
        .arg(Arg::with_name("confirm_other_connections")
             .short("f")
             .long("force")
             .help("Confirm that you wish to disconnect other devices"))
        .get_matches();

    let logopt: Option<&str> = args.value_of("username");
    if logopt.is_none() {
        eprintln!("ERROR: Missing user name. Aborting.");
        std::process::exit(1);
    }
    let login: String = String::from(logopt.unwrap());

    let passopt: Option<&str> = args.value_of("password");
    if passopt.is_none() {
        eprintln!("ERROR: Missing password. Aborting.");
        std::process::exit(1);
    }
    let pass: String = String::from(passopt.unwrap());

    let force: bool = args.is_present("confirm_other_connections");

    match connect(&login, &pass, force) {
        Ok(true)  => println!("Succesfully connected \u{2713}"),
        Ok(false) => {
            println!("Failed to connect \u{2717}");
            std::process::exit(1);
        },
        Err(e)    => {
            println!("Technical Error while connecting: {}", e);
            std::process::exit(1);
        }
    }
}
