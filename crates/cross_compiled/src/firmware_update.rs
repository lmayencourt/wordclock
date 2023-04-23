/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use anyhow::{anyhow, Result};
use log::*; 

use esp_idf_svc::http::client::{Configuration, EspHttpConnection};

use application::version::Version;

const WRITE_DATA_BUF_SIZE: usize = 2048;
const TX_BUF_SIZE: usize = 1024;

/// URL of the update version file hosted in GitHub
/// The file contains the version of the latest update available to download.
const OTA_VERSION_FILE_URL: &str = "https://raw.githubusercontent.com/lmayencourt/wordclock/released/version.txt?";

/// Root CA certificate used for TLS connection.
const ROOT_CA_CERTIFICATE: &[u8;1391] = b" \
 -----BEGIN CERTIFICATE-----\n \
 MIIDxTCCAq2gAwIBAgIQAqxcJmoLQJuPC3nyrkYldzANBgkqhkiG9w0BAQUFADBs\n \
 MQswCQYDVQQGEwJVUzEVMBMGA1UEChMMRGlnaUNlcnQgSW5jMRkwFwYDVQQLExB3\n \
 d3cuZGlnaWNlcnQuY29tMSswKQYDVQQDEyJEaWdpQ2VydCBIaWdoIEFzc3VyYW5j\n \
 ZSBFViBSb290IENBMB4XDTA2MTExMDAwMDAwMFoXDTMxMTExMDAwMDAwMFowbDEL\n \
 MAkGA1UEBhMCVVMxFTATBgNVBAoTDERpZ2lDZXJ0IEluYzEZMBcGA1UECxMQd3d3\n \
 LmRpZ2ljZXJ0LmNvbTErMCkGA1UEAxMiRGlnaUNlcnQgSGlnaCBBc3N1cmFuY2Ug\n \
 RVYgUm9vdCBDQTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAMbM5XPm\n \
 +9S75S0tMqbf5YE/yc0lSbZxKsPVlDRnogocsF9ppkCxxLeyj9CYpKlBWTrT3JTW\n \
 PNt0OKRKzE0lgvdKpVMSOO7zSW1xkX5jtqumX8OkhPhPYlG++MXs2ziS4wblCJEM\n \
 xChBVfvLWokVfnHoNb9Ncgk9vjo4UFt3MRuNs8ckRZqnrG0AFFoEt7oT61EKmEFB\n \
 Ik5lYYeBQVCmeVyJ3hlKV9Uu5l0cUyx+mM0aBhakaHPQNAQTXKFx01p8VdteZOE3\n \
 hzBWBOURtCmAEvF5OYiiAhF8J2a3iLd48soKqDirCmTCv2ZdlYTBoSUeh10aUAsg\n \
 EsxBu24LUTi4S8sCAwEAAaNjMGEwDgYDVR0PAQH/BAQDAgGGMA8GA1UdEwEB/wQF\n \
 MAMBAf8wHQYDVR0OBBYEFLE+w2kD+L9HAdSYJhoIAu9jZCvDMB8GA1UdIwQYMBaA\n \
 FLE+w2kD+L9HAdSYJhoIAu9jZCvDMA0GCSqGSIb3DQEBBQUAA4IBAQAcGgaX3Nec\n \
 nzyIZgYIVyHbIUf4KmeqvxgydkAQV8GK83rZEWWONfqe/EW1ntlMMUu4kehDLI6z\n \
 eM7b41N5cdblIZQB2lWHmiRk9opmzN6cN82oNLFpmyPInngiK3BD41VHMWEZ71jF\n \
 hS9OMPagMRYjyOfiZRYzy78aG6A9+MpeizGLYAiJLQwGXFK3xPkKmNEVX58Svnw2\n \
 Yzi9RKR/5CYrCsSXaQ3pjOLAEFe4yHYSkVXySGnYvCoCWw9E1CAx2/S6cCZdkGCe\n \
 vEsXCS+0yx5DaMkHJ8HSXPfqIbloEpw8nL+e/IBcm2PN7EeqJSdnoDfzAIJ9VNep\n \
 +OkuE6N36B9K\n \
 -----END CERTIFICATE-----\n\0";

pub fn read_update_version() -> Result<Version> {
    let certificate = esp_idf_svc::tls::X509::pem_until_nul(ROOT_CA_CERTIFICATE);

    debug!("Init HTTP client");
    let mut client = EspHttpConnection::new(&Configuration {
        buffer_size: Some(WRITE_DATA_BUF_SIZE),
        buffer_size_tx: Some(TX_BUF_SIZE),
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
        client_certificate: Some(certificate),
        ..Default::default()
    })?;

    debug!("Send request");
    client.initiate_request(embedded_svc::http::Method::Get, OTA_VERSION_FILE_URL, &[])?;

    debug!("Get response");
    client.initiate_response()?;

    if client.status() != 200 {
        return Err(anyhow!("HTTP return error code 200"));
    }

    debug!("Read file content-length");
    let content_length:usize;
    if let Some(len) = client.header("Content-Length") {
        content_length = len.parse().unwrap();
        debug!("Response size is {}", content_length);
    } else {
        return  Err(anyhow!("Reading content length from head failed"));
    }

    debug!("Get file content");
    let mut ota_version_file: [u8; WRITE_DATA_BUF_SIZE] = [0; WRITE_DATA_BUF_SIZE];
    if let Ok(ota_version_len) = client.read(&mut ota_version_file) {
        // Important to strip down the buffer to only the read length.
        // String parsing will fail otherwise...
        Ok(Version::from_utf8(&ota_version_file[0..ota_version_len].to_vec())?)
    } else {
        return Err(anyhow!("Failed to read OTA version file"));
    }
}
