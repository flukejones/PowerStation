use std::path::Path;
use udev::{Enumerator};

use crate::performance::gpu::tdp::{TDPDevice, TDPResult, TDPError};

use zbus::{Connection, Result};

use rog_dbus::RogDbusClientBlocking;
use rog_dbus::DbusProxies;
use rog_platform::error::PlatformError;
use rog_platform::platform::{GpuMode, Properties, ThrottlePolicy};
use rog_profiles::error::ProfileError;


/// Implementation of asusd with a fallback to asus-wmi sysfs
/// See https://www.kernel.org/doc/html/v6.8-rc4/admin-guide/abi-testing.html#abi-sys-devices-platform-platform-ppt-apu-sppt
pub struct ASUS {
    ppt_pl1_spl: String,
    ppt_pl2_sppt: String,
}

impl ASUS {

    /// test if we are in an asus system with asus-wmi loaded
    pub fn new() -> Option<Self> {
        //let context = Context::new().expect("Failed to create udev context");
        let mut enumerator = Enumerator::new(/*&context*/).expect("Failed to create enumerator");
        enumerator.match_subsystem("platform").expect("Failed to add subsystem filter");
        enumerator.match_sysname("asus-nb-wmi").expect("Failed to add sysname filter");

        for device in enumerator.scan_devices().expect("Failed to scan devices") {
            let ppt_pl1_spl = device.property_value("ppt_pl1_spl");
            let ppt_pl2_sppt = device.property_value("ppt_pl2_sppt");

            if let (Some(ppt_pl1_spl), Some(ppt_pl2_sppt)) = (ppt_pl1_spl, ppt_pl2_sppt) {
                let ppt_pl1_spl_str = ppt_pl1_spl.to_str().unwrap().to_string();
                let ppt_pl2_sppt_str = ppt_pl2_sppt.to_str().unwrap().to_string();

                log::info!("Found asus-wmi module");
                log::info!("ASUS ppt_pl1_spl: {}", ppt_pl1_spl_str);
                log::info!("ASUS ppt_pl2_sppt: {}", ppt_pl2_sppt_str);

                return Some(Self {
                    ppt_pl1_spl: ppt_pl1_spl_str,
                    ppt_pl2_sppt: ppt_pl2_sppt_str,
                });
            }
        }

        log::info!("Module asus-wmi not found");
        None
    }

}

impl TDPDevice for ASUS {
    fn tdp(&self) -> TDPResult<f64> {
        match RogDbusClientBlocking::new() {
            Ok((dbus, _)) => {
                
                let supported_properties = dbus.proxies().platform().supported_properties().unwrap();
                let supported_interfaces = dbus.proxies().platform().supported_interfaces().unwrap();
                

                match dbus.proxies().platform().ppt_apu_sppt() {
                    Ok(result) => {
                        log::info!("Initial ppt_apu_sppt: {}", result);
                        Ok(result as f64)
                    },
                    Err(err) => {
                        log::warn!("Error fetching ppt_apu_sppt: {}", err);
                        Err(TDPError::FailedOperation(format!("")))
                    }
                }
            },
            Err(err) => {
                log::warn!("Unable to use asusd to read tdp, asus-wmi interface will be used");
                Err(TDPError::FailedOperation(format!("")))
            }
        }
    }

    fn set_tdp(&mut self, value: f64) -> TDPResult<()> {
        todo!()
    }

    fn boost(&self) -> TDPResult<f64> {
        todo!()
    }

    fn set_boost(&mut self, value: f64) -> TDPResult<()> {
        todo!()
    }

    fn thermal_throttle_limit_c(&self) -> TDPResult<f64> {
        todo!()
    }

    fn set_thermal_throttle_limit_c(&mut self, limit: f64) -> TDPResult<()> {
        todo!()
    }

    fn power_profile(&self) -> TDPResult<String> {
        todo!()
    }

    fn set_power_profile(&mut self, profile: String) -> TDPResult<()> {
        todo!()
    }

}