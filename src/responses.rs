use serde::Deserialize;
use crate::errors;

#[derive(Deserialize, Debug)]
pub struct Container<T> {
    code: i32,
    message: String,
    result: T,
    total: i32,
    success: bool,
}

impl<T> Container<T> {
    pub fn inner(self) -> Result<T, errors::RequestError> {
        if self.code == 200 {
            return Ok(self.result);
        } else {
            return Err(errors::RequestError::BadRequest {
                code: self.code,
                message: self.message,
            });
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


#[derive(Deserialize, Debug)]
pub struct InnerIoTUserRuntimeDataLog {
    id: i32,
    gatewayId: String,
    dataArea: DataAreaInner,

    createTime: String, // TODO(richo) native datetime : "2022-11-08 12:22:21",
    updateTime: String,

}

pub type IoTUserRuntimeDataLog = Container<InnerIoTUserRuntimeDataLog>;

#[derive(Deserialize, Debug)]
struct DataAreaInner {
      bms_work: [i32; 2],
      solarSupply: i32,
      genChBat: i32,
      remoteSolar1Power: i32,
      di: [i32; 4],
      soc: i32,
      gridChBat: i32,
      r#do: [i32; 4],
      remoteSolarEn: i32,
      p_uti: f32,
      sinHTemp: i32,
      mode: i32,
      batOutGrid: i32,
      electricity_type: i32,
      signal: i32,
      soOutGrid: i32,
      main_sw: [i32; 3], // TODO(richo) this is probably the state of the switches? Are thre gauarnteed to be 3?
      slaver_stat: i32,
      fhpSn: Vec<String>,
      p_sun: f32,
      report_type: i32,
      remoteSolar2Power: i32,
      kwh_fhp_chg: f32,
      kwh_sun: f32,
      name: String,
      kwhGridLoad: i32,
      kwh_gen: i32,
      gridPhaseConSet: i32,
      kwh_uti_out: f32,
      kwh_load: f32,
      soChBat: i32,
      fhpSoc: Vec<i32>,
      p_fhp: f32,
      sinLTemp: i32,
      fhpPower: Vec<f32>,
      solarPower: i32,
      infi_status: Vec<i32>,
      connType: i32,
      p_gen: i32,
      kwhGenLoad: i32,
      p_load: f32,
      pro_load: Vec<i32>,
      pe_stat: Vec<i32>,
      kwh_uti_in: f32,
      run_status: i32,
      kwhSolarLoad: i32,
      t_amb: f32,
      wifiSignal: i32,
      kwhFhpLoad: i32,
      BFPVApboxRelay: i32,
      kwh_fhp_di: f32,
      genStat: i32,
      elecnet_state: i32,
}
