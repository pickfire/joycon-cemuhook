use joycon::hidapi::HidApi;
use joycon::joycon_sys::{HID_IDS, NINTENDO_VENDOR_ID};
use joycon::JoyCon;
use pad_motion::protocol::{ConnectionType, ControllerData, ControllerInfo, DeviceType, SlotState};
use pad_motion::server::{DsServer, Server};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let running = Arc::new(AtomicBool::new(true));
    let server = Arc::new(Server::new(None, None).unwrap());
    let handle = server.clone().start(running.clone());

    let controller_info = ControllerInfo {
        slot_state: SlotState::Connected,
        device_type: DeviceType::FullGyro,
        connection_type: ConnectionType::USB,
        ..Default::default()
    };
    server.update_controller_info(controller_info);

    let hid = HidApi::new()?;
    let mut joycon = loop {
        // TOOD support hotplug and multiple devices
        if let Some(device_info) = hid
            .device_list()
            .find(|x| x.vendor_id() == NINTENDO_VENDOR_ID && HID_IDS.contains(&x.product_id()))
        {
            let device = device_info.open_device(&hid)?;
            break JoyCon::new(device, device_info.clone())?;
        }
        thread::sleep(Duration::from_millis(500));
    };
    joycon.enable_imu()?;
    joycon.load_calibration()?;

    println!("joycon connected");
    let now = Instant::now();
    while running.load(Ordering::SeqCst) {
        let report = joycon.tick()?;
        for frame in &report.imu.unwrap() {
            let controller_data = ControllerData {
                connected: report.info.connected(),
                motion_data_timestamp: now.elapsed().as_micros() as u64,
                accelerometer_x: frame.accel.y as f32,
                accelerometer_y: frame.accel.z as f32,
                accelerometer_z: frame.accel.x as f32,
                gyroscope_pitch: frame.gyro.y as f32,
                gyroscope_yaw: -frame.gyro.z as f32,
                gyroscope_roll: -frame.gyro.x as f32,
                ..Default::default()
            };
            server.update_controller_data(0, controller_data);
        }
    }

    handle.join().unwrap();
    Ok(())
}
