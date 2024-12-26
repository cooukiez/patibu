use cpal::traits::{DeviceTrait, HostTrait};
use log::{error, info};

pub struct Audio {

}

impl Audio {
    pub fn new() -> Self {
        info!("supported hosts: {:?}", cpal::ALL_HOSTS);
        let available_hosts = cpal::available_hosts();
        info!("available hosts: {:?}", available_hosts);

        for host_id in available_hosts {
            let host = cpal::host_from_id(host_id).unwrap();

            let default_in = host.default_input_device().map(|e| e.name().unwrap());
            info!("[{}] default input device: {:?}", host_id.name(), default_in);

            let devices = host.devices().unwrap();
            for (i, device) in devices.enumerate() {
                info!("[{}] device {}: {:?}", host_id.name(), i, device.name().unwrap());

                if let Ok(conf) = device.default_input_config() {
                    println!("      default input stream config: {:?}", conf);
                }
                let input_configs = match device.supported_input_configs() {
                    Ok(f) => f.collect(),
                    Err(e) => {
                        error!("      error getting supported input configs: {:?}", e);
                        Vec::new()
                    }
                };
            }
        }

        Self {

        }
    }
}