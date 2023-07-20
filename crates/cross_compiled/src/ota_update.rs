/* SPDX-License-Identifier: MIT
 *
 * This files uses part of code from:
 * https://github.com/taunusflieger/anemometer/blob/master/anemometer-production/src/task/ota.rs
 * Copyright (c) 2021-2023 Michael Zill
 * 
 * Copyright (c) 2023 Louis Mayencourt
 */
use core::mem;
use core::ptr;

use anyhow::{anyhow, Result};
use log::*; 

use embedded_svc::ota::{FirmwareInfo, FirmwareInfoLoader, LoadResult, Slot};
use esp_idf_svc::http::client::{Configuration, EspHttpConnection};
use esp_idf_svc::ota::{EspFirmwareInfoLoader, EspOta};
use esp_idf_sys::*;

use application::firmware_update::FirmwareUpdate;
use application::version::Version;

/// HTTP read buffer size
const WRITE_DATA_BUF_SIZE: usize = 8196;
/// HTTP transmit buffer size
const TX_BUF_SIZE: usize = 4096;

/// URL of the update version file hosted in GitHub
/// The file contains the version of the latest update available to download.
const OTA_VERSION_FILE_URL: &str = "https://raw.githubusercontent.com/lmayencourt/wordclock/rust/ota-image/version.txt?";

/// URL of the update firmware files hosted in GitHub
const OTA_FIRMWARE_FILE_URL: &str = "https://raw.githubusercontent.com/lmayencourt/wordclock/rust/ota-image/firmware-ota.bin?";

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
 
pub struct OtaUpdate;

impl FirmwareUpdate for OtaUpdate {

    fn read_update_version(&self) -> Result<Version> {
        let certificate = esp_idf_svc::tls::X509::pem_until_nul(ROOT_CA_CERTIFICATE);

        info!("Init HTTP client");
        let mut client = EspHttpConnection::new(&Configuration {
            buffer_size: Some(WRITE_DATA_BUF_SIZE),
            buffer_size_tx: Some(TX_BUF_SIZE),
            crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
            client_certificate: Some(certificate),
            ..Default::default()
        })?;

        info!("Send request");
        client.initiate_request(embedded_svc::http::Method::Get, OTA_VERSION_FILE_URL, &[])?;

        info!("Get response");
        client.initiate_response()?;

        if client.status() != 200 {
            return Err(anyhow!("HTTP return error code 200"));
        }

        info!("Read file content-length");
        let content_length:usize;
        if let Some(len) = client.header("Content-Length") {
            content_length = len.parse().unwrap();
            info!("Response size is {}", content_length);
        } else {
            return  Err(anyhow!("Reading content length from head failed"));
        }

        info!("Get file content");
        let mut ota_version_file: [u8; WRITE_DATA_BUF_SIZE] = [0; WRITE_DATA_BUF_SIZE];
        if let Ok(ota_version_len) = client.read(&mut ota_version_file) {
            // Important to strip down the buffer to only the read length.
            // String parsing will fail otherwise...
            let ota_version_string = String::from_utf8(ota_version_file[0..ota_version_len].to_vec()).unwrap();

            // Anomaly-003: Stack overflow when parsing OTA version
            //
            // This need ~24kB of heap memory. It will fail if heap usage is to
            // high. Changing CONFIG_ESP_MAIN_TASK_STACK_SIZE higher than 20kB
            // may affect the heap available at this stage. The configuration
            // server need ~25kB of main stack to parse properly the
            // configuration URI. There is then an unresolved conflict here.
            // To see how much heap is available, uncomment:
            // unsafe { heap_caps_print_heap_info(0); }
            //
            // Currently only print the version here, and return a dummy value.
            info!("Available version {}", ota_version_string);
            // Ok(Version::from_utf8(&ota_version_string)?)
            Ok(Version::new(2, 1, 0, Some("dummy")))
        } else {
            return Err(anyhow!("Failed to read OTA version file"));
        }
    }

    fn download_update(&self) -> Result<()> {
        let mut ota_write_data: [u8; WRITE_DATA_BUF_SIZE] = [0; WRITE_DATA_BUF_SIZE];
        let invalid_fw_version: String = String::new();
        let found_invalid_fw = false;
        let mut update_summary: String = String::new();
        let certificate = esp_idf_svc::tls::X509::pem_until_nul(ROOT_CA_CERTIFICATE);

        info!("Init HTTP client");
        let mut client = EspHttpConnection::new(&Configuration {
            buffer_size: Some(WRITE_DATA_BUF_SIZE),
            buffer_size_tx: Some(TX_BUF_SIZE),
            crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
            client_certificate: Some(certificate),
            ..Default::default()
        })?;

        info!("Send request");
        client.initiate_request(embedded_svc::http::Method::Get, OTA_FIRMWARE_FILE_URL, &[])?;

        info!("Get response");
        client.initiate_response()?;

        if client.status() != 200 {
            return Err(anyhow!("HTTP return error code 200"));
        }

        info!("Read file content-length");
        let content_length:usize;
        if let Some(len) = client.header("Content-Length") {
            content_length = len.parse().unwrap();
            info!("Response size is {}", content_length);
        } else {
            return  Err(anyhow!("Reading content length from head failed"));
        }

        info!("Initialize OTA update");
        let update_partition: esp_partition_t =
        unsafe { *esp_ota_get_next_update_partition(ptr::null()) };
        let partition_label =
            std::str::from_utf8(unsafe { std::mem::transmute(&update_partition.label as &[i8]) })
                .unwrap()
                .trim_matches(char::from(0));
        info!(
            "Writing to partition {} subtype {:#4x} size {:#10x} at offset {:#10x}",
            partition_label, update_partition.subtype, update_partition.size, update_partition.address
        );
        info!("Writing to partition {:?}", update_partition);

        let mut ota = EspOta::new().expect("EspOta::new should have been successfull");

        let boot_slot = ota.get_boot_slot().unwrap();
        let run_slot = ota.get_running_slot().unwrap();
        let update_slot = ota.get_update_slot().unwrap();

        let ota_update = ota.initiate_update()?;

        let mut bytes_read_total = 0;
        let mut image_header_was_checked = false;

        loop {
            let data_read = client.read(&mut ota_write_data)?;

            // Check if first segment and process image meta data
            if !image_header_was_checked
                && data_read
                    > mem::size_of::<esp_image_header_t>()
                        + mem::size_of::<esp_image_segment_header_t>()
                        + mem::size_of::<esp_app_desc_t>()
            {
                let mut esp_fw_loader_info = EspFirmwareInfoLoader::new();
                let res = match esp_fw_loader_info.load(&ota_write_data) {
                    Ok(load_result) => load_result,
                    Err(err) => {
                        error!("failed to retrive firmware info from download: {err}");
                        return Err(anyhow!("failed to retrive firmware info from download: {err}"));
                    }
                };
                if res != LoadResult::Loaded {
                    // error!("incomplete data for retriving FW info for downloaded FW");
                    return Err(anyhow!("Incomplete data for retriving FW info for downloaded FW"));
                }

                let fw_info = esp_fw_loader_info.get_info();
                match fw_info {
                    Ok(info) => {
                        format_update_summary(
                            &mut update_summary,
                            boot_slot.clone(),
                            run_slot.clone(),
                            update_slot.clone(),
                            info.clone(),
                        );
                        info!("\n{update_summary}\n");

                        if found_invalid_fw && invalid_fw_version == info.version.to_string() {
                            info!("New FW has same version as invalide firmware slot. Stopping update");
                            return Err(anyhow!("New FW has same version as invalide firmware slot. Stopping update"));
                        }
                    }
                    Err(e) => return Err(e.into())
                }

                image_header_was_checked = true;
            }

            bytes_read_total += data_read;

            if data_read > 0 {
                if let Err(err) = ota_update.write(&ota_write_data) {
                    error!("ERROR failed to write update with: {err:?}");
                    return Err(anyhow!("ERROR failed to write update with: {err:?}"));
                }
            }

            // Check if we have read an entire buffer. If not,
            // we assume it was the last segment and we stop
            if ota_write_data.len() > data_read {
                break;
            }
        }

        if bytes_read_total == content_length {
            if let Err(err) = ota_update.complete() {
                error!("OTA update failed. esp_ota_end failed {:?}", err);
                return Err(anyhow!("OTA update failed. esp_ota_end failed {:?}", err));
            }
        } else {
            ota_update.abort().unwrap();
            error!("ERROR firmware update failed");
            return Err(anyhow!("ERROR firmware update failed"));
        };

        Ok(())
    }

    fn reboot_to_new_image(&self) {
        unsafe {
            esp_idf_sys::esp_restart();
        }
    }
}

fn format_update_summary(
    update_summary: &mut String,
    boot_slot: Slot,
    run_slot: Slot,
    update_slot: Slot,
    ota_image_info: FirmwareInfo,
    ) {
    let mut label: String = String::new();

    update_summary.push_str("OTA Update Summary\n");
    update_summary.push_str("==================\n");
    update_summary.push_str("Boot   partition: ");
    label.push_str(&boot_slot.label.to_string());
    update_summary.push_str(label.as_str());
    update_summary.push_str(", ");
    add_firmware_info(update_summary, boot_slot.firmware);

    update_summary.push_str("\nRun    partition: ");
    label = String::new();
    label.push_str(&run_slot.label.to_string());
    update_summary.push_str(label.as_str());
    update_summary.push_str(", ");
    add_firmware_info(update_summary, run_slot.firmware);

    update_summary.push_str("\nUpdate partition: ");
    label = String::new();
    label.push_str(&update_slot.label.to_string());
    update_summary.push_str(label.as_str());
    update_summary.push_str(", ");
    add_firmware_info(update_summary, update_slot.firmware);
    update_summary.push_str("\n");

    update_summary.push_str("\nDownloaded FW  : ");
    add_firmware_info(update_summary, Some(ota_image_info));
    update_summary.push_str("\n");
}

fn add_firmware_info(
    update_summary: &mut String,
    firmware: Option<FirmwareInfo>,
    ) {
    let mut version: String = String::new();
    let mut released: String = String::new();
    let mut description: String = String::new();

    if let Some(fw) = firmware {
        version.push_str(&fw.version.to_string());
        update_summary.push_str(version.as_str());
        update_summary.push_str(", ");
        released.push_str(&fw.released.to_string());
        update_summary.push_str(released.as_str());
        if let Some(desc) = fw.description {
            update_summary.push_str(", ");
            description.push_str(&desc.to_string());
            update_summary.push_str(description.as_str());
        }
    }
}