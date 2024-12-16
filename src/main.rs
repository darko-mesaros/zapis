use std::path::PathBuf;

use block_utils;

fn main() {
    println!("Hello, zapis!");
    let devices = block_utils::get_block_devices().unwrap();
    let mut usb_devices: Vec<PathBuf> = Vec::new();

    println!("All devices:");
    println!("{:#?}", devices);

    for device in devices {
        //let info = block_utils::get_block_dev_properties(&device).unwrap();
        //println!("{:#?}", info);
        //let devinfo = block_utils::get_device_info(&device).unwrap();
        //println!("DevInfo: {:#?}", devinfo);
        
        // NOTE: On my Linux (Arch) system with a generic flash drive I can extract these. I am not
        // sure if these are the right to check to test for every drive, but could be.
        // "ID_USB_DRIVER": "usb-storage",
        // "ID_USB_TYPE": "disk"
        let usb_driver = block_utils::get_block_dev_property(&device, "ID_USB_DRIVER").unwrap();
        let usb_type = block_utils::get_block_dev_property(&device, "ID_USB_TYPE").unwrap();

        if usb_driver.is_some_and(|x| x == "usb-storage") && usb_type.is_some_and(|x| x == "disk") {
            usb_devices.push(device.clone());
        };
    }
    println!("========================================");
    println!("USB Devices:");
    match usb_devices.len() {
        0 => println!("No usb devices found"),
        _ => println!("{:#?}", usb_devices),
    }
}
