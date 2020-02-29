/****************************************************************************
    Copyright (c) 2015 Osspial All Rights Reserved.

    This file is part of hidapi-rs, based on hidapi_rust by Roland Ruckerbauer.
****************************************************************************/

//! Prints out a list of HID devices

extern crate hidapi;

use hidapi::HidApi;

fn main() {
    println!("Printing all available hid devices:");

    match HidApi::new() {
        Ok(api) => {
            for device in api.device_list() {
                println!(
                    "VID: {:04x}, PID: {:04x}, Serial: {}",
                    device.vendor_id(),
                    device.product_id(),
                    match device.serial_number() {
                        Some(s) => s,
                        _ => "<COULD NOT FETCH>",
                    }
                );
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
