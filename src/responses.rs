use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Container<T> {
    code: i32,
    message: String,
    result: T,
    total: i32,
    success: bool,
}

impl<T> Container<T> {
    pub fn inner(self) -> Result<T, ()> {
        if self.code == 200 {
            return Ok(self.result);
        } else {
            return Err(());
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct InnerChargePowerDetails {
    pub batterySoc: f32,
    pub touMinSoc: i32,
    pub currentStateDuration: Option<f32>,
    pub highEnergyConsumption: f32,
    pub averageEnergyConsumption: f32,
    pub currentTime: Option<String>,
    pub highEnergyTime: String,
    pub averageTime: String,
}

pub type ChargePowerDetails = Container<InnerChargePowerDetails>;
