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
use application::configuration_form::CONFIGURATION_FORM;

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
    response.write_all(CONFIGURATION_FORM.as_bytes())?;

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