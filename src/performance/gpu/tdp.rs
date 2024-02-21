pub enum TDPError {
    FeatureUnsupported,
    FailedOperation(String),
    InvalidArgument(String),
    IOError(String),
}

pub enum TDPDevices {
    ASUS(super::amd::asus::ASUS),
    AMD(super::amd::tdp::TDP),
    INTEL(super::intel::tdp::TDP)
}

impl TDPDevices {
    pub fn asus(&self) -> Option<&super::amd::asus::ASUS> {
        if let Self::ASUS(asus) = self {
            return Some(asus);
        }
        None
    }

    pub fn amd(&self) -> Option<&super::amd::tdp::TDP> {
        if let Self::AMD(amd) = self {
            return Some(amd);
        }
        None
    }

    pub fn intel(&self) -> Option<&super::intel::tdp::TDP> {
        if let Self::INTEL(intel) = self {
            return Some(intel);
        }
        None
    }

    // TODO: or just define the basic stuff here
    async fn tdp(&self) -> TDPResult<f64> {
        match self {
            TDPDevices::ASUS(dev) => dev.tdp().await,
            TDPDevices::AMD(dev) => dev.tdp().await,
            TDPDevices::INTEL(dev) => dev.tdp().await,
        }
    }
}

// TODO: Or you can define all the work here
impl TDPDevice for TDPDevices {
    fn tdp(&self) -> impl std::future::Future<Output = TDPResult<f64>> {
        match self {
            TDPDevices::ASUS(dev) => dev.tdp(),
            TDPDevices::AMD(dev) => dev.tdp(),
            TDPDevices::INTEL(dev) => dev.tdp(),
        }
    }

    fn set_tdp(&mut self, value: f64) -> impl std::future::Future<Output = TDPResult<()>> {
        todo!()
    }

    fn boost(&self) -> impl std::future::Future<Output = TDPResult<f64>> {
        todo!()
    }

    fn set_boost(&mut self, value: f64) -> impl std::future::Future<Output = TDPResult<()>> {
        todo!()
    }

    fn thermal_throttle_limit_c(&self) -> impl std::future::Future<Output = TDPResult<f64>> {
        todo!()
    }

    fn set_thermal_throttle_limit_c(&mut self, limit: f64) -> impl std::future::Future<Output = TDPResult<()>> {
        todo!()
    }

    fn power_profile(&self) -> impl std::future::Future<Output = TDPResult<String>> {
        todo!()
    }

    fn set_power_profile(&mut self, profile: String) -> impl std::future::Future<Output = TDPResult<()>> {
        todo!()
    }
}

impl Into<String> for TDPError {
    fn into(self) -> std::string::String {
        todo!()
    }
}

pub type TDPResult<T> = Result<T, TDPError>;

pub trait TDPDevice : Sync + Send {
    fn tdp(&self) -> impl std::future::Future<Output = TDPResult<f64>>;
    fn set_tdp(&mut self, value: f64) -> impl std::future::Future<Output = TDPResult<()>>;
    fn boost(&self) -> impl std::future::Future<Output = TDPResult<f64>>;
    fn set_boost(&mut self, value: f64) -> impl std::future::Future<Output = TDPResult<()>>;
    fn thermal_throttle_limit_c(&self) -> impl std::future::Future<Output = TDPResult<f64>>;
    fn set_thermal_throttle_limit_c(&mut self, limit: f64) -> impl std::future::Future<Output = TDPResult<()>>;
    fn power_profile(&self) -> impl std::future::Future<Output = TDPResult<String>>;
    fn set_power_profile(&mut self, profile: String) -> impl std::future::Future<Output = TDPResult<()>>;
}