/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */
use std::sync::Mutex;

use anyhow::Result;
use log::*;

use esp_idf_svc::http::server::{Configuration, EspHttpServer};
use esp_idf_svc::http::server::EspHttpConnection;
use embedded_svc::{http::server::Request, io::Write, utils::http::Headers};

use application::configuration_server::ConfigurationServer;

pub struct ServerGlobalData {
    pub configuration_received: bool,
    pub uri: Option<String>,
}

/// Global variable to store HTTP URI query string to be parsed later on
///
/// Only available on this module. Use "is_configuration_received()" and
/// "get_config_uri()" for public access to the data.
static GLOBAL_CONFIG_SERVER_STATE: Mutex<ServerGlobalData> = Mutex::new(ServerGlobalData {
    configuration_received: false,
    uri: None,
});

/// HTTP server
///
/// Provide a single home page, containing the WordClock configuration form.
/// Handle the "/get" request when the "submit" button is pressed by the user.
pub struct HttpServer {
    _server: EspHttpServer,
}

impl HttpServer {
    pub fn new() -> Result<Self> {
        let mut server = EspHttpServer::new(&Configuration::default())?;

        server.fn_handler("/", embedded_svc::http::Method::Get, move |req| {
            home_page_handler(req)
        })?;

        server.fn_handler("/get", embedded_svc::http::Method::Get, move |req| {get_handler(req)})?;

        Ok(Self{_server: server})
    }
}

/// Display the configuration form on http "/" page request
fn home_page_handler(req: Request<&mut EspHttpConnection>) -> embedded_svc::http::server::HandlerResult {
    let mut headers = Headers::<1>::new();
    headers.set_cache_control("no-store");
    
    info!("Processing '/' request");
    let mut response = req.into_response(200, None, headers.as_slice())?;
    response.write_all(config_form("hello wordclock!").as_bytes())?;

    Ok(())
}

/// Handle the "/get" request when user press the "submit" button in the configuration form
fn get_handler(mut req: Request<&mut EspHttpConnection>) -> embedded_svc::http::server::HandlerResult {
    info!("Processing '/get' request");

    let mut headers = Headers::<1>::new();
    headers.set_cache_control("no-store");

    let uri = req.connection().uri();
    info!("got {:?}", uri);

    // do not perform memory intensive parsing here. Store the value in a global
    // variable to be handled by the main thread.
    GLOBAL_CONFIG_SERVER_STATE.lock().unwrap().uri = Some(uri.to_string());
    GLOBAL_CONFIG_SERVER_STATE.lock().unwrap().configuration_received = true;

    let mut response = req.into_ok_response()?;
    response.write_all("".as_bytes())?;

    Ok(())
}

/// Provide the html configuration form
fn config_form(content: impl AsRef<str>) -> String {
    format!(
        r#"
<!DOCTYPE HTML>
<html>
    <head>
    <title>Word-Clock</title>
    <meta name="viewport" content="width=device-width, initial-scale=1">
    </head>
    <body>
    <h1>{}</h1>
    <form action="/get" target="hidden-form">
        Wifi SSID (name): <input type="text" name="input_wifi_ssid"><br><br>
        Wifi password: <input type="text" name="input_wifi_password"><br><br>
        Night-mode starts at: <input type="text" name="input_night_mode_start">
        <div class="tooltip">&#9432;
            <span class="tooltiptext">E.g. 12:00 <br>
            Must be between 12:00 and 23:59</span>
        </div><br><br>
        Night-mode ends at: <input type="text" name="input_night_mode_end">
        <div class="tooltip">&#9432;
            <span class="tooltiptext">E.g. 08:00 <br>
            Must be between 00:00 and 12:00</span>
        </div><br><br>
        Display color: <input type="color" id="favcolor" name="favcolor" value="\#0000ff"><br><br>
        <input type="submit" value="Submit" onclick="submitMessage()">
    </form><br>
    <iframe style="display:none" name="hidden-form"></iframe>
    </body>
</html>
"#,
        content.as_ref()
    )
}

impl ConfigurationServer for HttpServer {
    fn is_configuration_received(&self) -> bool {
        GLOBAL_CONFIG_SERVER_STATE.lock().unwrap().configuration_received
    }

    fn get_config_uri(&mut self) -> Option<String> {
        let uri = GLOBAL_CONFIG_SERVER_STATE.lock().unwrap().uri.clone();
        GLOBAL_CONFIG_SERVER_STATE.lock().unwrap().uri = None;
        GLOBAL_CONFIG_SERVER_STATE.lock().unwrap().configuration_received = false;
        uri
    }
}