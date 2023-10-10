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
    response.write_all(config_form("WordClock configuration").as_bytes())?;

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
        r##"
        <!DOCTYPE HTML>
        <html>
            <head>
            <title>Word-Clock</title>
            <meta name="viewport" content="width=device-width, initial-scale=1">
            </head>
            <style>
                * {{
                    box-sizing: border-box;
                    font-family: Arial, Helvetica, sans-serif;
                }}
                body {{
                    color: #2c2c2c;
                    margin: 32px;
                }}
                .config-card {{
                    border: #ccd5ff solid;
                    border-radius: 16px;
                    margin-bottom: 16px;
                    padding: 8px;
                }}
                h2.config-title {{
                    margin-top: 2px;
                    margin-bottom: 16px;
                }}
                .config-element {{
                    padding: 8px;
                    border-radius: 16px;
                }}
                input[type=text] {{
                    border-radius: 16px;
                    width: auto;
                    margin-top: 4px;
                }}
                .config-element>label {{
                    position:absolute;
                    margin-top: -16px;
                    text-align:center;
                    vertical-align:bottom;
                    font-style: normal;
                    font-weight: 400;
                    padding-top:2px;
                }}
                input[type=submit] {{
                    background-color: #1755e6;
                    color: white;
                    padding: 10px;
                    padding-left: 24px;
                    padding-right: 24px;
                    border: none;
                    border-radius: 24px;
                }}
                input[type=submit]:hover {{
                    background-color: #0a3494;
                }}
            </style>
            <body>
            <h1>{}</h1>
            <form action="/get" target="hidden-form">
                <div class="config-card">
                    <h2 class="config-title">WiFi</h2>
                    <div class="config-element">
                        <label for="input_wifi_ssid">SSID (name)</label>
                        <input type="text" class="form-control" name="input_wifi_ssid" placeholder="Your WiFi network name">
                    </div>
                    <div class="config-element">
                        <label for="input_wifi_password">Password</label>
                        <input type="text" name="input_wifi_password" placeholder="Your WiFi network password">
                    </div>
                </div>
                <div class="config-card">
                    <h2 class="config-title">Night-mode</h2>
                    <div class="config-element">
                        <label for="input_night_mode_start">Start at:</label>
                        <input type="text" name="input_night_mode_start" placeholder="22:00">
                        Must be between 12:00 and 23:59 
                    </div>
                    <div class="config-element">
                        <label for="input_night_mode_end">End at:</label>
                        <input type="text" name="input_night_mode_end" placeholder="06:30">
                        Must be between 00:00 and 12:00
                    </div>
                </div>
                <div class="config-card">
                    <h2 class="config-title">Display color</h2>
                    <input type="color" id="favcolor" name="favcolor" value="#ffffff">
                </div>
                <input id="submit" type="submit" value="Submit" onclick="submitMessage()">
            </form>
            <iframe style="display:none" name="hidden-form"></iframe>
            </body>
        </html>
"##,
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