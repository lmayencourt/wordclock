/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

/// HTML configuration form
pub const CONFIGURATION_FORM: &str = r##"
    <!DOCTYPE HTML>
    <html>
        <head>
        <title>Word-Clock</title>
        <meta name="viewport" content="width=device-width, initial-scale=1">
        </head>
        <style>
            * {
                box-sizing: border-box;
                font-family: Arial, Helvetica, sans-serif;
            }
            body {
                color: #2c2c2c;
                margin: 32px;
            }
            .config-card {
                border-radius: 16px;
                margin-bottom: 16px;
                padding: 8px;
                box-shadow: 0 4px 8px 0 #00000033, 0 6px 20px 0 #00000030;
            }
            h2.config-title {
                margin-top: 2px;
                margin-bottom: 16px;
            }
            .config-element {
                padding: 8px;
                border-radius: 16px;
            }
            input[type=text] {
                border-width: 1px;
                border-radius: 16px;
                width: auto;
                margin-top: 4px;
            }
            input[type=time] {
                border-width: 1px;
                border-radius: 16px;
                width: auto;
                margin-top: 4px;
            }
            .config-element>label {
                position:absolute;
                margin-top: -16px;
                text-align:center;
                vertical-align:bottom;
                font-style: normal;
                font-weight: 400;
                padding-top:2px;
            }
            input[type=submit] {
                background-color: #1755e6;
                color: white;
                padding: 10px;
                padding-left: 24px;
                padding-right: 24px;
                border: none;
                border-radius: 24px;
                box-shadow: 0 4px 8px 0 #00000033, 0 6px 20px 0 #00000030;
            }
            input[type=submit]:hover {
                background-color: #0a3494;
            }
        </style>
        <body>
            <h1>WordClock configuration</h1>
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
                        <input type="time" name="input_night_mode_start" placeholder="22:00">
                        Must be between 12:00 and 23:59 
                    </div>
                    <div class="config-element">
                        <label for="input_night_mode_end">End at:</label>
                        <input type="time" name="input_night_mode_end" placeholder="06:30">
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
"##;